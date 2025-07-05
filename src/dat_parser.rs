use anyhow::Result;
use log::{debug, warn};
use std::collections::HashMap;
use std::path::Path;

use crate::error::DatError;

#[derive(Debug, Clone)]
pub struct DatEntry {
    pub crc32: u32,
    pub name: String,
    pub size: Option<u64>,
    pub md5: Option<String>,
    pub sha1: Option<String>,
}

#[derive(Debug, Default)]
pub struct DatCollection {
    // CRC32 -> ROM name mapping
    crc_to_name: HashMap<u32, String>,
    // System -> DatEntry mapping
    pub entries: HashMap<String, Vec<DatEntry>>,
    // All entries indexed by CRC32 for fast lookup
    crc_to_entry: HashMap<u32, DatEntry>,
}

impl DatCollection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_from_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        self.load_dat_file(path.as_ref())
    }

    pub fn find_by_crc32(&self, crc32: u32) -> Option<&DatEntry> {
        self.crc_to_entry.get(&crc32)
    }

    pub fn load_directory(dir: &Path) -> Result<Self> {
        let mut collection = Self::new();

        if !dir.exists() {
            warn!("Diretório DAT não encontrado: {}", dir.display());
            return Ok(collection);
        }

        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "dat") {
                if let Err(e) = collection.load_dat_file(&path) {
                    warn!("Falha ao carregar DAT {}: {}", path.display(), e);
                }
            }
        }

        debug!("Carregados {} entradas DAT de {}", collection.crc_to_name.len(), dir.display());
        Ok(collection)
    }

    pub fn load_dat_file(&mut self, path: &Path) -> Result<()> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| DatError::LoadFailed {
                path: path.to_path_buf(),
                source: e,
            })?;

        // Detect DAT format and parse accordingly
        if content.contains("clrmamepro") || content.contains("game (") {
            self.parse_clrmamepro_dat(&content, path)?;
        } else if content.contains("<?xml") {
            self.parse_xml_dat(&content, path)?;
        } else {
            // Try simple format: CRC32=Name
            self.parse_simple_dat(&content, path)?;
        }

        Ok(())
    }

    fn parse_clrmamepro_dat(&mut self, content: &str, path: &Path) -> Result<()> {
        let system_name = self.extract_system_name_from_path(path);
        let mut entries = Vec::new();

        for line in content.lines() {
            let line = line.trim();
            
            // Look for game entries: game ( ... )
            if line.starts_with("game (") {
                if let Some(entry) = self.parse_clrmamepro_game_block(content, line)? {
                    self.crc_to_name.insert(entry.crc32, entry.name.clone());
                    self.crc_to_entry.insert(entry.crc32, entry.clone());
                    entries.push(entry);
                }
            }
        }

        if !entries.is_empty() {
            self.entries.insert(system_name, entries);
        }

        Ok(())
    }

    fn parse_clrmamepro_game_block(&self, _content: &str, _line: &str) -> Result<Option<DatEntry>> {
        // TODO: Implement full ClrMamePro DAT parsing
        // This is a complex format that requires proper parsing of nested blocks
        Ok(None)
    }

    fn parse_xml_dat(&mut self, _content: &str, path: &Path) -> Result<()> {
        // TODO: Implement XML DAT parsing for No-Intro/Redump XML formats
        // This requires a proper XML parser
        warn!("XML DAT parsing not yet implemented: {}", path.display());
        Ok(())
    }

    fn parse_simple_dat(&mut self, content: &str, path: &Path) -> Result<()> {
        let system_name = self.extract_system_name_from_path(path);
        let mut entries = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            let line = line.trim();
            
            // Skip comments and empty lines
            if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
                continue;
            }

            // Parse format: CRC32=Name or CRC32:Name or CRC32|Name
            if let Some((crc_str, name)) = line.split_once('=')
                .or_else(|| line.split_once(':'))
                .or_else(|| line.split_once('|')) {
                
                match u32::from_str_radix(crc_str.trim(), 16) {
                    Ok(crc32) => {
                        let name = name.trim().to_string();
                        let entry = DatEntry {
                            crc32,
                            name: name.clone(),
                            size: None,
                            md5: None,
                            sha1: None,
                        };
                        
                        self.crc_to_name.insert(crc32, name);
                        self.crc_to_entry.insert(crc32, entry.clone());
                        entries.push(entry);
                    }
                    Err(_) => {
                        warn!("CRC32 inválido na linha {} de {}: {}", 
                              line_num + 1, path.display(), crc_str);
                    }
                }
            }
        }

        if !entries.is_empty() {
            self.entries.insert(system_name, entries);
        }

        Ok(())
    }

    fn extract_system_name_from_path(&self, path: &Path) -> String {
        path.file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string()
    }

    pub fn get_name_by_crc(&self, crc32: u32) -> Option<String> {
        self.crc_to_name.get(&crc32).cloned()
    }

    pub fn get_system_entries(&self, system: &str) -> Option<&[DatEntry]> {
        self.entries.get(system).map(|v| v.as_slice())
    }

    pub fn total_entries(&self) -> usize {
        self.crc_to_name.len()
    }

    pub fn systems_count(&self) -> usize {
        self.entries.len()
    }
}

// Helper function to create simple DAT files for testing
#[allow(dead_code)]
pub fn create_simple_dat(entries: &[(u32, &str)]) -> String {
    let mut content = String::new();
    content.push_str("# Simple DAT format\n");
    content.push_str("# CRC32=Name\n\n");

    for (crc32, name) in entries {
        content.push_str(&format!("{:08X}={}\n", crc32, name));
    }

    content
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn test_simple_dat_parsing() {
        let dat_content = r#"
# Test DAT file
# CRC32=Name format

12345678=Super Mario Bros (USA)
ABCDEF01=The Legend of Zelda (USA)
        "#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(dat_content.as_bytes()).unwrap();

        let mut collection = DatCollection::new();
        collection.parse_simple_dat(dat_content, temp_file.path()).unwrap();

        assert_eq!(collection.total_entries(), 2);
        assert_eq!(
            collection.get_name_by_crc(0x12345678).unwrap(),
            "Super Mario Bros (USA)"
        );
        assert_eq!(
            collection.get_name_by_crc(0xABCDEF01).unwrap(),
            "The Legend of Zelda (USA)"
        );
    }

    #[test]
    fn test_invalid_crc_handling() {
        let dat_content = r#"
INVALID_CRC=Invalid Game
12345678=Valid Game
        "#;

        let mut collection = DatCollection::new();
        let temp_path = PathBuf::from("test.dat");
        collection.parse_simple_dat(dat_content, &temp_path).unwrap();

        // Should only have the valid entry
        assert_eq!(collection.total_entries(), 1);
        assert_eq!(
            collection.get_name_by_crc(0x12345678).unwrap(),
            "Valid Game"
        );
    }

    #[test]
    fn test_create_simple_dat() {
        let entries = [
            (0x12345678, "Game 1"),
            (0xABCDEF01, "Game 2"),
        ];

        let dat_content = create_simple_dat(&entries);
        
        assert!(dat_content.contains("12345678=Game 1"));
        assert!(dat_content.contains("ABCDEF01=Game 2"));
    }
}
