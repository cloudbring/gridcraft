//! Just enough of the Compound File Binary (CFB/OLE2) container to tell what a non-zip
//! spreadsheet file is: Excel wraps password-protected workbooks in a CFB holding an
//! `EncryptedPackage` stream, while Excel 97-2003 `.xls` (and other legacy Office files) are
//! CFB files without one. Parsing the container itself lives in the shared `craft-cfb` crate
//! (one reader across the ArtCraft apps); the streams are read through `craft_cfb::Cfb` at the
//! call sites, and every read there is bounds-checked with bounded loops.

use craft_cfb::Cfb;

/// The CFB signature at the start of the file.
pub(crate) const SIGNATURE: [u8; 8] = [0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1];

/// Stream names that mark an encrypted (password-protected) OOXML package.
const ENCRYPTED_STREAMS: [&str; 2] = ["EncryptedPackage", "EncryptionInfo"];

/// Whether the compound file holds an encryption stream. Reads the directory through the shared
/// `craft-cfb` reader; when the structure can't be followed (truncated or malformed), falls
/// back to scanning the bytes for the UTF-16LE stream names, so sniffing of damaged-but-
/// encrypted files stays as tolerant as before.
pub(crate) fn is_encrypted(bytes: &[u8]) -> bool {
    match Cfb::open(bytes) {
        Ok(cf) => cf.entries().any(|e| ENCRYPTED_STREAMS.iter().any(|s| e.path().rsplit('/').next().is_some_and(|n| n.eq_ignore_ascii_case(s)))),
        Err(_) => ENCRYPTED_STREAMS.iter().any(|e| contains_utf16(bytes, e)),
    }
}

fn contains_utf16(hay: &[u8], needle: &str) -> bool {
    let n: Vec<u8> = needle.encode_utf16().flat_map(u16::to_le_bytes).collect();
    !n.is_empty() && hay.windows(n.len()).any(|w| w == n.as_slice())
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// A minimal compound file (512-byte sectors: FAT at sector 0, directory at sector 1) whose
    /// directory holds a root entry plus the given stream names. Deliberately loose (no
    /// mini-stream cutoff, a flat directory): `Cfb::open` rejects it, so it exercises the
    /// `is_encrypted` fallback that damaged files take.
    pub(crate) fn build(streams: &[&str]) -> Vec<u8> {
        let mut b = vec![0u8; 512 * 3];
        b[..8].copy_from_slice(&SIGNATURE);
        b[24..26].copy_from_slice(&0x3Eu16.to_le_bytes()); // minor version
        b[26..28].copy_from_slice(&3u16.to_le_bytes()); // major version
        b[28..30].copy_from_slice(&0xFFFEu16.to_le_bytes()); // byte order
        b[30..32].copy_from_slice(&9u16.to_le_bytes()); // sector shift
        b[32..34].copy_from_slice(&6u16.to_le_bytes()); // mini sector shift
        b[44..48].copy_from_slice(&1u32.to_le_bytes()); // FAT sectors
        b[48..52].copy_from_slice(&1u32.to_le_bytes()); // first directory sector
        b[60..64].copy_from_slice(&0xFFFF_FFFEu32.to_le_bytes()); // no mini FAT
        b[68..72].copy_from_slice(&0xFFFF_FFFEu32.to_le_bytes()); // no DIFAT sectors
        for i in 0..109 {
            b[76 + i * 4..80 + i * 4].copy_from_slice(&0xFFFF_FFFFu32.to_le_bytes());
        }
        b[76..80].copy_from_slice(&0u32.to_le_bytes()); // FAT lives in sector 0
        // FAT (sector 0, at byte 512): sector 0 is a FAT sector, sector 1 ends its chain.
        for i in 0..128 {
            b[512 + i * 4..516 + i * 4].copy_from_slice(&0xFFFF_FFFFu32.to_le_bytes());
        }
        b[512..516].copy_from_slice(&0xFFFF_FFFDu32.to_le_bytes());
        b[516..520].copy_from_slice(&0xFFFF_FFFEu32.to_le_bytes());
        // Directory (sector 1, at byte 1024).
        for (k, name) in std::iter::once("Root Entry").chain(streams.iter().copied()).take(4).enumerate() {
            let e = 1024 + k * 128;
            let units: Vec<u16> = name.encode_utf16().take(31).collect();
            for (i, u) in units.iter().enumerate() {
                b[e + i * 2..e + i * 2 + 2].copy_from_slice(&u.to_le_bytes());
            }
            b[e + 64..e + 66].copy_from_slice(&((units.len() as u16 + 1) * 2).to_le_bytes());
            b[e + 66] = if k == 0 { 5 } else { 2 };
        }
        b
    }

    #[test]
    fn reads_directory_names() {
        // A well-formed file: the shared reader walks the directory tree.
        let file = crate::testing::compound_file(&[("EncryptionInfo", &[0; 8]), ("EncryptedPackage", &[0; 8])]);
        let cf = Cfb::open(&file).unwrap();
        let names: Vec<&str> = cf.entries().map(|e| e.path()).collect();
        assert_eq!(names, ["", "EncryptionInfo", "EncryptedPackage"]);
        assert!(is_encrypted(&file));
        // The loose `build` fixture sniffs through the damaged-file fallback.
        assert!(is_encrypted(&build(&["EncryptionInfo", "EncryptedPackage"])));
        assert!(!is_encrypted(&build(&["Workbook", "\u{5}SummaryInformation"])));
    }

    #[test]
    fn hostile_headers_never_panic() {
        let mut b = build(&["Workbook"]);
        for at in [30usize, 48, 68, 76, 512, 516, 1024 + 64] {
            for v in [0u32, 1, 7, 0xFFFF_FFFA, 0xFFFF_FFFF, 0x7FFF_FFFF] {
                let mut c = b.clone();
                c[at..at + 4].copy_from_slice(&v.to_le_bytes());
                let _ = is_encrypted(&c);
                for cut in [0usize, 8, 100, 600, 1100] {
                    let _ = is_encrypted(&c[..cut.min(c.len())]);
                }
            }
        }
        // A directory chain that loops back on itself stops at the bound.
        b[516..520].copy_from_slice(&1u32.to_le_bytes());
        assert!(!is_encrypted(&b));
    }

    /// `compound_file` with one regular-sector stream of `len` bytes (FAT in sector 0, directory
    /// in sector 1, the stream from sector 2), cut right after the stream's last byte, the way
    /// some writers leave the final sector unpadded.
    fn unpadded(len: usize) -> (Vec<u8>, Vec<u8>) {
        let data: Vec<u8> = (0..len).map(|i| (i % 251) as u8).collect();
        let mut file = crate::testing::compound_file(&[("Workbook", &data)]);
        file.truncate(512 * 3 + len);
        (file, data)
    }

    // The old in-crate reader tolerated a file ending inside a chain's last sector while the
    // stream's declared bytes were all present; the shared craft-cfb reader requires every
    // sector of a chain to be fully present, so such files are damage now (ticket #123).

    #[test]
    fn an_unpadded_last_sector_is_damage() {
        let (file, _) = unpadded(5000);
        assert_ne!(file.len() % 512, 0);
        let cf = Cfb::open(&file).unwrap();
        assert!(cf.read_stream("Workbook").is_err());
        // One byte short of the stream's declared size is damage too.
        let cf = Cfb::open(&file[..file.len() - 1]).unwrap();
        assert!(cf.read_stream("Workbook").is_err());
    }

    #[test]
    fn an_unpadded_last_mini_sector_is_damage() {
        let data: Vec<u8> = (0..1000u32).map(|i| (i % 251) as u8).collect();
        let mut file = crate::testing::compound_file(&[("Workbook", &data)]);
        // Header, FAT, directory, mini FAT, then the mini stream holding the stream's bytes.
        file.truncate(512 * 4 + data.len());
        let cf = Cfb::open(&file).unwrap();
        assert!(cf.read_stream("Workbook").is_err());
    }

    #[test]
    fn a_cut_sector_inside_a_chain_is_damage() {
        let (mut file, _) = unpadded(5000);
        // The cut sector (11) points on to the directory sector and the stream claims those
        // bytes too: the partial sector would splice 392 bytes into the middle of the stream.
        file[512 + 11 * 4..512 + 12 * 4].copy_from_slice(&1u32.to_le_bytes());
        file[1024 + 128 + 120..1024 + 128 + 124].copy_from_slice(&5512u32.to_le_bytes());
        let cf = Cfb::open(&file).unwrap();
        assert!(cf.read_stream("Workbook").is_err());
    }

    #[test]
    fn a_cut_mini_sector_inside_a_chain_is_damage() {
        let small = [7u8; 100];
        let data: Vec<u8> = (0..1000u32).map(|i| (i % 251) as u8).collect();
        let mut file = crate::testing::compound_file(&[("Small", &small), ("Workbook", &data)]);
        // "Small" holds mini sectors 0-1, "Workbook" 2-17; the mini stream starts at sector 3.
        file.truncate(512 * 4 + 128 + data.len());
        let cf = Cfb::open(&file).unwrap();
        assert!(cf.read_stream("Workbook").is_err());
        // The cut mini sector (17) points on to sector 0 and the stream claims those bytes too.
        file[1536 + 17 * 4..1536 + 18 * 4].copy_from_slice(&0u32.to_le_bytes());
        file[1024 + 2 * 128 + 120..1024 + 2 * 128 + 124].copy_from_slice(&1064u32.to_le_bytes());
        let cf = Cfb::open(&file).unwrap();
        assert!(cf.read_stream("Workbook").is_err());
    }
}
