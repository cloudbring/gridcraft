//! OPC package access: reading zip parts with decompression limits, relationships and part-name
//! resolution; writing a zip package deterministically.

use std::io::{Cursor, Write};

use craft_ooxml::opc::{ContentTypes, LazyLimits, LazyPackage, OpcError, OpcLimits, TargetMode};

use crate::IoError;
use crate::xml::{self, El};

/// Largest decompressed part accepted.
pub const MAX_PART: u64 = 512 * 1024 * 1024;
/// Largest total decompressed size read from one package.
pub const MAX_TOTAL: u64 = 1024 * 1024 * 1024;
/// Most entries accepted in the zip directory.
pub const MAX_ENTRIES: usize = 200_000;

/// The shared reader with gridcraft's limits: case-insensitive part names, `Ok(None)` for a
/// missing part, and this crate's error type. The zip index, byte budget, bomb guard and
/// relationship parsing are `craft-ooxml`'s.
pub struct Package<'a> {
    pkg: LazyPackage<&'a [u8]>,
}

fn limits() -> OpcLimits {
    let mut l = OpcLimits::default();
    l.zip.max_entries = MAX_ENTRIES;
    l.zip.max_entry_bytes = MAX_PART;
    l.zip.max_total_bytes = MAX_TOTAL;
    l.max_archive_bytes = usize::MAX;
    l
}

fn io_error(e: OpcError) -> IoError {
    match e {
        OpcError::Limit(m) => IoError::TooLarge(m.into()),
        e => IoError::Zip(e.to_string()),
    }
}

/// A relationship.
#[derive(Clone, Debug)]
pub struct Rel {
    pub id: String,
    /// Last path segment of the relationship type (`worksheet`, `styles`, …): the same for the
    /// transitional and strict namespaces.
    pub kind: String,
    /// Resolved part name (no leading `/`), or the raw target for external relationships.
    pub target: String,
    pub external: bool,
}

pub fn normalize(name: &str) -> String {
    name.replace('\\', "/").trim_start_matches('/').to_ascii_lowercase()
}

impl<'a> Package<'a> {
    pub fn open(bytes: &'a [u8]) -> Result<Package<'a>, IoError> {
        LazyPackage::open_with(bytes, &limits(), LazyLimits { charge_failed_reads_full_cap: true, no_expansion_ratio_guard: true })
            .map(|pkg| Package { pkg })
            .map_err(io_error)
    }

    /// The package's parsed `[Content_Types].xml` (empty when missing or unparseable).
    pub fn content_types(&self) -> &ContentTypes {
        &self.pkg.content_types
    }

    pub fn has(&self, part: &str) -> bool {
        self.stored(part).is_some()
    }

    /// The archive's spelling of `part`. An entry spelled exactly like the request wins over a
    /// case-variant of it (`xl/worksheets/sheet1.xml` next to `Sheet1.xml`): the index is sorted,
    /// so the case-insensitive lookup alone would pick whichever spelling sorts first.
    fn stored(&self, part: &str) -> Option<String> {
        let part = part.replace('\\', "/");
        let exact = part.trim_start_matches('/');
        if self.pkg.declared_size(exact).is_some() {
            return Some(exact.to_string());
        }
        self.pkg.find(&part).map(str::to_string)
    }

    /// Reads a part's bytes; `Ok(None)` when it doesn't exist.
    pub fn read(&mut self, part: &str) -> Result<Option<Vec<u8>>, IoError> {
        self.read_limited(part, MAX_PART)
    }

    /// Reads a part the reader can do without (the caller warns and goes on when it fails). A
    /// damaged part is charged its whole allowance instead of ending the package's budget.
    pub fn read_optional_limited(&mut self, part: &str, maximum: u64) -> Result<Option<Vec<u8>>, IoError> {
        let Some(name) = self.stored(part) else { return Ok(None) };
        self.pkg.read_optional(&name, maximum).map_err(io_error)
    }

    /// Reads a small metadata part without allowing it to expand to the general part limit. A
    /// read that fails mid-part ends the package's budget.
    pub fn read_limited(&mut self, part: &str, maximum: u64) -> Result<Option<Vec<u8>>, IoError> {
        let Some(name) = self.stored(part) else { return Ok(None) };
        let declared = self.pkg.declared_size(&name).unwrap_or(0);
        let read = self.pkg.read_optional(&name, maximum.min(MAX_PART));
        if matches!(read, Err(OpcError::Corrupt(_))) || (matches!(read, Err(OpcError::Limit(_))) && declared <= maximum) {
            // The work was spent; make the exhaustion terminal so later reads can't repeat it.
            let _ = self.pkg.charge(u64::MAX);
        }
        read.map_err(io_error)
    }

    pub fn read_xml(&mut self, part: &str) -> Result<Option<El>, IoError> {
        match self.read(part)? {
            Some(b) => xml::parse(&b).map(Some).map_err(|e| prefix(part, e)),
            None => Ok(None),
        }
    }

    /// Relationships of a part (`xl/workbook.xml` → `xl/_rels/workbook.xml.rels`). Missing or
    /// broken rels parts give an empty list.
    pub fn rels(&mut self, part: &str) -> Result<Vec<Rel>, IoError> {
        let rels = self.pkg.rels_of(part, MAX_PART).map_err(io_error)?;
        Ok(rels
            .into_iter()
            .map(|r| Rel {
                kind: r.kind.rsplit('/').next().unwrap_or("").to_string(),
                external: r.mode == TargetMode::External,
                id: r.id,
                target: r.target,
            })
            .collect())
    }
}

/// The first part whose declared unpacked size is over [`MAX_PART`], with that size.
pub fn oversized_part(bytes: &[u8]) -> Option<(String, u64)> {
    let mut l = limits();
    l.zip.max_entry_bytes = u64::MAX;
    let pkg = LazyPackage::open(bytes, &l).ok()?;
    pkg.part_names().find_map(|n| pkg.declared_size(n).filter(|&s| s > MAX_PART).map(|s| (n.to_string(), s)))
}

fn prefix(part: &str, e: IoError) -> IoError {
    match e {
        IoError::Xml(m) => IoError::Xml(format!("{part}: {m}")),
        o => o,
    }
}

/// `xl/worksheets/sheet1.xml` → (`xl/worksheets`, `sheet1.xml`).
pub fn split_dir(part: &str) -> (&str, &str) {
    let p = part.trim_start_matches('/');
    match p.rfind('/') {
        Some(i) => (p.get(..i).unwrap_or(""), p.get(i + 1..).unwrap_or("")),
        None => ("", p),
    }
}

/// Resolves a relationship target against the source part's directory.
pub fn resolve(dir: &str, target: &str) -> String {
    let target = target.replace('\\', "/");
    let target = target.split('#').next().unwrap_or("").to_string();
    let mut segs: Vec<&str> = if target.starts_with('/') { vec![] } else { dir.split('/').filter(|s| !s.is_empty()).collect() };
    for s in target.split('/') {
        match s {
            "" | "." => {}
            ".." => {
                segs.pop();
            }
            s => segs.push(s),
        }
    }
    segs.join("/")
}

// ---------------------------------------------------------------- writing

pub struct ZipOut {
    zip: zip::ZipWriter<Cursor<Vec<u8>>>,
}

impl ZipOut {
    pub fn new() -> ZipOut {
        ZipOut { zip: zip::ZipWriter::new(Cursor::new(Vec::new())) }
    }
    pub fn add(&mut self, name: &str, data: &[u8]) -> Result<(), IoError> {
        let opts = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .compression_level(Some(6))
            .last_modified_time(zip::DateTime::default())
            .large_file(data.len() as u64 >= u32::MAX as u64);
        self.zip.start_file(name, opts).map_err(|e| IoError::Zip(e.to_string()))?;
        self.zip.write_all(data).map_err(|e| IoError::Zip(e.to_string()))?;
        Ok(())
    }
    pub fn finish(self) -> Result<Vec<u8>, IoError> {
        self.zip.finish().map(|c| c.into_inner()).map_err(|e| IoError::Zip(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolving() {
        assert_eq!(resolve("xl", "worksheets/sheet1.xml"), "xl/worksheets/sheet1.xml");
        assert_eq!(resolve("xl/worksheets", "../drawings/d1.xml"), "xl/drawings/d1.xml");
        assert_eq!(resolve("xl/worksheets", "/xl/tables/t.xml"), "xl/tables/t.xml");
        assert_eq!(resolve("", "xl/workbook.xml"), "xl/workbook.xml");
        assert_eq!(resolve("a", "../../../x"), "x");
        assert_eq!(split_dir("xl/workbook.xml"), ("xl", "workbook.xml"));
        assert_eq!(split_dir("x.xml"), ("", "x.xml"));
    }

    /// Two entries whose names differ only in case: the one spelled like the request is read, as
    /// before the swap to craft-ooxml (whose sorted index put `Sheet1.xml` ahead of `sheet1.xml`).
    #[test]
    fn a_case_variant_entry_does_not_shadow_the_exact_one() {
        let z = crate::tests::make_zip(&[("xl/worksheets/sheet1.xml", b"lower"), ("xl/worksheets/Sheet1.xml", b"upper")]);
        let mut pkg = Package::open(&z).unwrap();
        assert_eq!(pkg.read("xl/worksheets/sheet1.xml").unwrap().as_deref(), Some(&b"lower"[..]));
        assert_eq!(pkg.read("xl/worksheets/Sheet1.xml").unwrap().as_deref(), Some(&b"upper"[..]));
        assert_eq!(pkg.read("XL/worksheets/SHEET1.xml").unwrap().as_deref(), Some(&b"upper"[..]), "no exact spelling: sorted first");
    }

    /// An over-budget read must spend the package's allowance once and for all: loops over many
    /// broken parts (a sheet's optional VML relationships, for one) otherwise re-decompress the
    /// remaining allowance again and again.
    #[test]
    fn a_failed_part_ends_the_package_budget() {
        let z = crate::tests::make_zip(&[("a.xml", b"0123456789"), ("b.xml", b"hello")]);
        let mut pkg = Package::open(&z).unwrap();
        assert!(pkg.read("a.xml").unwrap().is_some());
        // Nothing left to spend within the per-part limit either.
        pkg.pkg.charge(MAX_TOTAL - pkg.pkg.total_read() - 1).unwrap();
        assert!(pkg.read("b.xml").is_err());
        assert_eq!(pkg.pkg.remaining_budget(), 0, "the failed read is charged");
        // Oversized repeated attempts stay cheap errors now: no allowance is left to pay for.
        assert!(matches!(pkg.read("b.xml"), Err(IoError::TooLarge(_))));
    }

    /// A part the reader can do without, damaged inside an intact archive, costs its own
    /// allowance: the parts after it are still read.
    #[test]
    fn a_damaged_optional_part_costs_only_its_allowance() {
        use std::io::Write as _;
        let mut w = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
        let stored = zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
        w.start_file("a.xml", stored).unwrap();
        w.write_all(b"<a>damaged</a>").unwrap();
        w.start_file("b.xml", zip::write::SimpleFileOptions::default()).unwrap();
        w.write_all(b"hello").unwrap();
        let mut z = w.finish().unwrap().into_inner();
        let at = z.windows(7).position(|x| x == b"damaged").unwrap();
        z[at] ^= 0x20;
        let mut pkg = Package::open(&z).unwrap();
        assert!(matches!(pkg.read_optional_limited("a.xml", 1024), Err(IoError::Zip(_))));
        assert_eq!(pkg.pkg.total_read(), 1024, "the failed read is charged its allowance");
        assert_eq!(pkg.read("b.xml").unwrap().as_deref(), Some(&b"hello"[..]));
        // An ordinary read of it still ends the package's budget.
        assert!(pkg.read("a.xml").is_err());
        assert_eq!(pkg.pkg.remaining_budget(), 0);
    }
}
