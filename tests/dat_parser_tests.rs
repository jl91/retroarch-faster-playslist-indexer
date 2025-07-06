use retroarch_fast_indexer::dat_parser::DatCollection;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_mame_clrmamepro_dat_parsing() {
    let sample_dat = r#"clrmamepro (
    name "MAME 0.245"
    description "MAME 0.245"
)

game (
    name "1942"
    description "1942 (Revision B)"
    rom ( name "srb-03.m3" size 2048 crc 36d7200e sha1 b13e04e8b53c76e5c0f2045b5d72f13a7027a2c4 )
    rom ( name "srb-04.m4" size 2048 crc 3ebf6858 sha1 c2d12cd37b16033e0bb4a0c8bcb8a12175e72de5 )
)

game (
    name "pacman"
    description "Pac-Man (Midway)"
    rom ( name "pacman.6e" size 4096 crc c1e6ab10 sha1 e87e059c5be45753f7e9f33dff8f35211c9b91c1 )
    rom ( name "pacman.6f" size 4096 crc 1a6fb2d4 sha1 674d3a7f00d8be5e38b1fdc208ebef5a92d38329 )
)
"#;

    // Create temporary file
    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    temp_file.write_all(sample_dat.as_bytes()).expect("Failed to write to temp file");
    
    // Parse the DAT
    let mut collection = DatCollection::new();
    collection.load_dat_file(temp_file.path()).expect("Failed to parse DAT");
    
    // Test lookups
    assert!(collection.find_by_crc32(0x36d7200e).is_some());
    assert!(collection.find_by_crc32(0xc1e6ab10).is_some());
    
    let entry1942 = collection.find_by_crc32(0x36d7200e).unwrap();
    assert_eq!(entry1942.name, "1942 (Revision B)");
    
    let entry_pacman = collection.find_by_crc32(0xc1e6ab10).unwrap();
    assert_eq!(entry_pacman.name, "Pac-Man (Midway)");
    
    // Test name lookup
    assert_eq!(collection.get_name_by_crc(0x36d7200e).unwrap(), "1942 (Revision B)");
    assert_eq!(collection.get_name_by_crc(0xc1e6ab10).unwrap(), "Pac-Man (Midway)");
}

#[test]
fn test_mame_multiline_game_format() {
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
    
    let entry = collection.find_by_crc32(0x4b5b7d8d).unwrap();
    assert_eq!(entry.name, "Street Fighter (US, Set 1)");
}

#[test]
fn test_invalid_crc_handling() {
    let sample_dat = r#"clrmamepro (
    name "TEST"
)

game (
    name "badrom"
    description "Bad ROM Example"
    rom ( name "bad.bin" size 1024 crc INVALID sha1 abcdef )
)
"#;

    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    temp_file.write_all(sample_dat.as_bytes()).expect("Failed to write to temp file");
    
    let mut collection = DatCollection::new();
    // Should not fail, just skip invalid entries
    collection.load_dat_file(temp_file.path()).expect("Failed to parse DAT");
    
    // Should have no entries due to invalid CRC
    assert_eq!(collection.total_entries(), 0);
}
