use retroarch_fast_indexer::dat_parser::DatCollection;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_debug_mame_parsing() {
    let sample_dat = r#"
game (
    name "1942"
    description "1942 (Revision B)"
    rom ( name "srb-03.m3" size 2048 crc 36d7200e sha1 b13e04e8b53c76e5c0f2045b5d72f13a7027a2c4 )
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
    
    println!("Total entries loaded: {}", collection.total_entries());
    
    // Try looking for the CRCs
    let crc1 = 0x36d7200e;
    let crc2 = 0xc1e6ab10;
    
    println!("Looking for CRC: {:08X}", crc1);
    if let Some(entry) = collection.find_by_crc32(crc1) {
        println!("Found: {}", entry.name);
    } else {
        println!("Not found!");
    }
    
    println!("Looking for CRC: {:08X}", crc2);
    if let Some(entry) = collection.find_by_crc32(crc2) {
        println!("Found: {}", entry.name);
    } else {
        println!("Not found!");
    }
    
    // Print all entries
    println!("All loaded entries:");
    for i in 0..collection.total_entries() {
        if let Some(name) = collection.get_name_by_crc(crc1) {
            println!("  {:08X} -> {}", crc1, name);
            break;
        }
    }
}
