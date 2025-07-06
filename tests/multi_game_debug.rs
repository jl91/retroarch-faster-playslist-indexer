use retroarch_fast_indexer::dat_parser::DatCollection;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_debug_multiple_games() {
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

    println!("Sample DAT content:");
    println!("{}", sample_dat);
    
    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    temp_file.write_all(sample_dat.as_bytes()).expect("Failed to write to temp file");
    
    let mut collection = DatCollection::new();
    collection.load_dat_file(temp_file.path()).expect("Failed to parse DAT");
    
    println!("Total entries loaded: {}", collection.total_entries());
    
    // Test each CRC individually
    let crc1 = 0x36d7200e;
    let crc2 = 0x3ebf6858;
    let crc3 = 0xc1e6ab10;
    let crc4 = 0x1a6fb2d4;
    
    println!("Looking for CRC1: {:08X}", crc1);
    if let Some(entry) = collection.find_by_crc32(crc1) {
        println!("  Found: {}", entry.name);
    } else {
        println!("  Not found!");
    }
    
    println!("Looking for CRC2: {:08X}", crc2);
    if let Some(entry) = collection.find_by_crc32(crc2) {
        println!("  Found: {}", entry.name);
    } else {
        println!("  Not found!");
    }
    
    println!("Looking for CRC3: {:08X}", crc3);
    if let Some(entry) = collection.find_by_crc32(crc3) {
        println!("  Found: {}", entry.name);
    } else {
        println!("  Not found!");
    }
    
    println!("Looking for CRC4: {:08X}", crc4);
    if let Some(entry) = collection.find_by_crc32(crc4) {
        println!("  Found: {}", entry.name);
    } else {
        println!("  Not found!");
    }
}
