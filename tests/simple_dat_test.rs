use retroarch_fast_indexer::dat_parser::DatCollection;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_simple_single_game() {
    let sample_dat = r#"clrmamepro (
    name "MAME"
)

game (
    name "pacman"
    description "Pac-Man (Midway)"
    rom ( name "pacman.6e" size 4096 crc c1e6ab10 sha1 e87e059c5be45753f7e9f33dff8f35211c9b91c1 )
)
"#;

    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    temp_file.write_all(sample_dat.as_bytes()).expect("Failed to write to temp file");
    
    let mut collection = DatCollection::new();
    collection.load_dat_file(temp_file.path()).expect("Failed to parse DAT");
    
    println!("Total entries: {}", collection.total_entries());
    
    let crc = 0xc1e6ab10;
    println!("Looking for CRC: {:08X}", crc);
    
    if let Some(entry) = collection.find_by_crc32(crc) {
        println!("Found: {} (CRC: {:08X})", entry.name, entry.crc32);
        assert_eq!(entry.name, "Pac-Man (Midway)");
    } else {
        println!("Not found!");
        panic!("Entry not found for CRC {:08X}", crc);
    }
}
