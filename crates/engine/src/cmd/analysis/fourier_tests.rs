use gridcraft_core::{CellRef, Value};
use serde_json::json;

use super::MAX_FOURIER;
use crate::Session;

fn stagewise_fft(a: &mut [(f64, f64)], inverse: bool) {
    let n = a.len();
    if n < 2 || !n.is_power_of_two() {
        return;
    }
    let bits = n.trailing_zeros();
    for i in 0..n {
        let j = i.reverse_bits() >> (usize::BITS - bits);
        if i < j {
            a.swap(i, j);
        }
    }
    let sign = if inverse { 1.0 } else { -1.0 };
    let mut len = 2;
    while len <= n {
        let ang = sign * 2.0 * std::f64::consts::PI / len as f64;
        // One twiddle table per stage: every butterfly of the stage reuses the same len/2
        // factors, so the table replaces the per-butterfly sin/cos. Each factor is still
        // computed directly, not by repeated multiplication, and the total over all stages is
        // O(n) factors (n − 2) against the O(n log n) of the per-butterfly form.
        let twiddles: Vec<(f64, f64)> = (0..len / 2).map(|k| ((ang * k as f64).cos(), (ang * k as f64).sin())).collect();
        for start in (0..n).step_by(len) {
            for (k, &(wr, wi)) in twiddles.iter().enumerate() {
                let (i, j) = (start + k, start + k + len / 2);
                let (Some(&(ur, ui)), Some(&(xr, xi))) = (a.get(i), a.get(j)) else { continue };
                let (vr, vi) = (xr * wr - xi * wi, xr * wi + xi * wr);
                if let Some(u) = a.get_mut(i) {
                    *u = (ur + vr, ui + vi);
                }
                if let Some(x) = a.get_mut(j) {
                    *x = (ur - vr, ui - vi);
                }
            }
        }
        len <<= 1;
    }
    if inverse {
        let nf = n as f64;
        a.iter_mut().for_each(|v| *v = (v.0 / nf, v.1 / nf));
    }
}

pub(super) fn fft(values: &mut [(f64, f64)], inverse: bool) {
    let mut complex: Vec<_> = values.iter().map(|&(real, imaginary)| craft_fft::Complex::new(real, imaginary)).collect();
    super::complex_fft(&mut complex, inverse).unwrap();
    for (value, transformed) in values.iter_mut().zip(complex) {
        *value = (transformed.re, transformed.im);
    }
}

#[test]
fn fft_matches_issue_23_for_random_power_of_two_inputs() {
    let mut max_relative_error = 0.0_f64;
    let mut comparisons = 0;
    for exponent in 0..=MAX_FOURIER.trailing_zeros() {
        let length = 1 << exponent;
        for seed in 0..32 {
            for magnitude in [1e-12, 1.0, 1e12] {
                let original: Vec<_> = input(seed, length).into_iter().map(|(real, imaginary)| (real * magnitude, imaginary * magnitude)).collect();
                for inverse in [false, true] {
                    let mut expected = original.clone();
                    stagewise_fft(&mut expected, inverse);
                    let mut actual = original.clone();
                    fft(&mut actual, inverse);
                    for (index, (actual, expected)) in actual.iter().zip(expected).enumerate() {
                        let error = (actual.0 - expected.0).hypot(actual.1 - expected.1);
                        let scale = expected.0.hypot(expected.1).max(magnitude * f64::EPSILON);
                        let relative_error = error / scale;
                        max_relative_error = max_relative_error.max(relative_error);
                        comparisons += 1;
                        assert!(
                            relative_error <= 1e-9,
                            "length={length} seed={seed} inverse={inverse} magnitude={magnitude} bin={index}: {actual:?} vs {expected:?}; relative error={relative_error}"
                        );
                    }
                }
            }
        }
    }
    println!("issue #23 differential: {comparisons} complex bins, maximum relative error={max_relative_error:e}");
}

fn input(seed: u64, length: usize) -> Vec<(f64, f64)> {
    let mut state = seed ^ 0x9E37_79B9_7F4A_7C15;
    let mut next = || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        (state >> 11) as f64 / (1u64 << 53) as f64 * 20.0 - 10.0
    };
    (0..length).map(|_| (next(), next())).collect()
}

fn session(length: usize) -> Session {
    let mut session = Session::new();
    session.new_workbook();
    let values: Vec<_> = input(23, length).into_iter().map(|(real, imaginary)| json!([format!("{real}{imaginary:+}i")])).collect();
    session.execute("range.setValues", json!({"range": "A1", "values": values})).unwrap();
    session
}

#[test]
fn fourier_accepts_the_limit_and_rejects_other_lengths_without_edits() {
    let mut session = session(MAX_FOURIER + 1);
    for length in [0, 3, 5, MAX_FOURIER - 1, MAX_FOURIER + 1] {
        let range = if length == 0 { "B1".to_string() } else { format!("A1:A{length}") };
        for inverse in [false, true] {
            assert!(
                session
                    .execute("data.analysisToolPak", json!({"tool": "fourierAnalysis", "range": range, "inverse": inverse, "output": "C1"}))
                    .is_err()
            );
            assert_eq!(session.doc().unwrap().wb.sheet(0).unwrap().value(CellRef::parse("C1").unwrap()), Value::Empty);
        }
    }
    session.execute("data.analysisToolPak", json!({"tool": "fourierAnalysis", "range": format!("A1:A{MAX_FOURIER}"), "output": "C1"})).unwrap();
    assert!(matches!(session.doc().unwrap().wb.sheet(0).unwrap().value(CellRef::new(MAX_FOURIER as u32 - 1, 2)), Value::Text(_)));
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
#[allow(clippy::disallowed_methods)]
#[ignore = "release timing: cargo test -p gridcraft-engine --release fourier_command_benchmark -- --ignored --nocapture --test-threads=1"]
fn fourier_command_benchmark() {
    for inverse in [false, true] {
        let params = json!({"tool": "fourierAnalysis", "range": format!("A1:A{MAX_FOURIER}"), "output": "C1", "inverse": inverse});
        let mut command_times = Vec::new();
        let mut kernel_times = Vec::new();
        for iteration in 0..103 {
            let mut session = session(MAX_FOURIER);
            let started = web_time::Instant::now();
            std::hint::black_box(session.execute("data.analysisToolPak", params.clone()).unwrap());
            let command_time = started.elapsed().as_secs_f64() * 1e6;
            let mut values: Vec<_> = input(23, MAX_FOURIER).into_iter().map(|(real, imaginary)| craft_fft::Complex::new(real, imaginary)).collect();
            let started = web_time::Instant::now();
            super::complex_fft(std::hint::black_box(&mut values), inverse).unwrap();
            std::hint::black_box(&values);
            let kernel_time = started.elapsed().as_secs_f64() * 1e6;
            if iteration >= 3 {
                command_times.push(command_time);
                kernel_times.push(kernel_time);
            }
        }
        command_times.sort_by(f64::total_cmp);
        kernel_times.sort_by(f64::total_cmp);
        println!(
            "MAX_FOURIER={MAX_FOURIER} inverse={inverse}: command median={:.3} us; kernel median={:.3} us (100 runs, setup excluded, planning included)",
            command_times[50], kernel_times[50]
        );
    }
}
