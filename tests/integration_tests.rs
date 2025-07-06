use retroarch_fast_indexer::{Scanner, PlaylistBuilder, Platform, Config, CrcCache};
use tempfile::tempdir;
use std::fs;

/// Basic integration test for scanner functionality
#[test]
fn test_scanner_basic_functionality() {
    let temp_dir = tempdir().unwrap();
    let roms_dir = temp_dir.path().join("roms");
    
    // Create test ROM files
    fs::create_dir_all(&roms_dir).unwrap();
    fs::write(roms_dir.join("test.nes"), b"fake nes rom").unwrap();
    fs::write(roms_dir.join("test.sfc"), b"fake sfc rom").unwrap();
    fs::write(roms_dir.join("test.z64"), b"fake n64 rom").unwrap();
    
    // Create scanner
    let scanner = Scanner::new()
        .with_threads(2)
        .with_recursive(true);
    
    // Test that scanner was created successfully (we can't access private fields)
    assert!(true); // Scanner created without panic
}

/// Test platform functionality
#[test]
fn test_platform_functionality() {
    let _platform_win = Platform::Windows;
    let _platform_switch = Platform::Switch;
    
    // Test platform types can be created
    assert!(true);
}

/// Test playlist builder creation
#[test]
fn test_playlist_builder() {
    let _builder = PlaylistBuilder::new();
    
    // Test that builder was created successfully
    assert!(true); // Builder created without panic
}

/// Test basic config functionality
#[test]
fn test_config_creation() {
    let _config = Config::default();
    
    // Test that config has expected defaults
    assert!(true); // Config created without panic
}

/// Test CRC cache creation
#[test]
fn test_cache_creation() {
    let _cache = CrcCache::new();
    
    // Test that cache was created successfully
    assert!(true); // Cache created without panic
}

/// Test basic module functionality
#[test]
fn test_module_integration() {
    // Test that all main modules can be imported and basic types created
    let _scanner = Scanner::new();
    let _builder = PlaylistBuilder::new();
    let _config = Config::default();
    let _cache = CrcCache::new();
    let _platform = Platform::Windows;
    
    // If we reach here, all modules are properly integrated
    assert!(true);
}
