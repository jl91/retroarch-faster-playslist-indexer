use std::io::Write;
use tempfile::NamedTempFile;
use retroarch_fast_indexer::dat_parser::DatCollection;

#[test]
fn debug_multiline_test() {
    let sample_dat = r#"clrmamepro (
    name "MAME"
)

game
(
    name "streetfighter"
    description "Street Fighter (US, Set 1)"
    rom
    (
        name "sf_36.bin"
        size 32768
        crc 4b5b7d8d
        sha1 4d3c263e2d13c0b395ad55d07d2e7ca1ba3bb16e
    )
)
"#;

    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    temp_file.write_all(sample_dat.as_bytes()).expect("Failed to write to temp file");
    
    let mut collection = DatCollection::new();
    collection.load_dat_file(temp_file.path()).expect("Failed to parse DAT");
    
    println!("Total entries: {}", collection.total_entries());
    
    // Try to find any CRC that exists by manually checking the internal structure
    // Since we can't directly iterate over all entries, let's try the expected CRC first
    
    // Try to find by the expected CRC
    let expected_crc = 0x4b5b7d8d;
    println!("Looking for CRC32: 0x{:08x}", expected_crc);
    
    match collection.find_by_crc32(expected_crc) {
        Some(entry) => println!("Found entry: {}", entry.name),
        None => println!("Entry not found!"),
    }
}
