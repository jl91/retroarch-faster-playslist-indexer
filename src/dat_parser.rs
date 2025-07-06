use anyhow::Result;
use log::{debug, warn};
use std::collections::HashMap;
use std::path::Path;

use crate::error::DatError;
use crate::mame_xml::MameXmlDatabase;

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
    // MAME XML database for MAME 2003-Plus fallback
    mame_xml_db: Option<MameXmlDatabase>,
}

impl DatCollection {
    pub fn new() -> Self {
        let mut collection = Self {
            crc_to_name: HashMap::new(),
            entries: HashMap::new(),
            crc_to_entry: HashMap::new(),
            mame_xml_db: None,
        };
        
        // Always try to load MAME XML database for MAME fallback
        if let Err(e) = collection.load_mame_xml_database() {
            warn!("Falha ao carregar MAME XML database: {}", e);
        }
        
        collection
    }

    /// Load MAME XML database for fallback lookup
    pub fn load_mame_xml_database(&mut self) -> Result<()> {
        match MameXmlDatabase::load_embedded() {
            Ok(db) => {
                let (total_roms, _) = db.stats();
                debug!("Carregado MAME XML database com {} ROMs para fallback", total_roms);
                self.mame_xml_db = Some(db);
                Ok(())
            }
            Err(e) => {
                warn!("Falha ao carregar MAME XML database: {}", e);
                Ok(()) // Non-fatal error, continue without MAME XML fallback
            }
        }
    }

    /// Try to load MAME XML database from file, fallback to embedded
    pub fn load_mame_xml_database_from_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        match MameXmlDatabase::load_from_file(path.as_ref()) {
            Ok(db) => {
                let (total_roms, _) = db.stats();
                debug!("Carregado MAME XML database de arquivo com {} ROMs", total_roms);
                self.mame_xml_db = Some(db);
                Ok(())
            }
            Err(_) => {
                // Fallback to embedded
                self.load_mame_xml_database()
            }
        }
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
        } else {
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
        }

        // Always try to load MAME XML database as fallback
        if let Err(e) = collection.load_mame_xml_database() {
            warn!("Falha ao carregar MAME XML database: {}", e);
        }

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

        // Parse the entire content looking for game blocks
        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;
        
        while i < lines.len() {
            let line = lines[i].trim();
            
            if line.starts_with("game (") || (line == "game" && i + 1 < lines.len() && lines[i+1].trim() == "(") {
                // Found a game block, parse it
                let mut game_content = Vec::new();
                let mut brace_count = 0;
                
                // Handle both "game (" and "game\n(" formats
                if line.starts_with("game (") {
                    // For "game (" format, we need to handle the opening brace
                    brace_count = 1;
                    // Add the content after "game ("
                    let content_part = line[5..].trim();
                    if !content_part.is_empty() && content_part != "(" {
                        game_content.push(content_part.to_string());
                    }
                    i += 1;
                } else {
                    // For "game\n(" format
                    i += 1; // Skip "game"
                    if i < lines.len() && lines[i].trim() == "(" {
                        brace_count = 1;
                        i += 1; // Skip "("
                    }
                }
                
                // Collect all lines until we close the game block
                while i < lines.len() && brace_count > 0 {
                    let current_line = lines[i].trim();
                    
                    // Count opening and closing braces
                    let open_count = current_line.matches('(').count();
                    let close_count = current_line.matches(')').count();
                    
                    brace_count += open_count;
                    brace_count -= close_count;
                    
                    // Only add the line if we haven't closed all braces yet
                    if brace_count > 0 || (brace_count == 0 && close_count == 0) {
                        game_content.push(lines[i].to_string());
                    }
                    i += 1;
                }
                
                // Parse this game block
                let game_block = game_content.join("\n");
                let game_entries = self.parse_clrmamepro_game_block(&game_block, "")?;
                for entry in game_entries {
                    debug!("Parsed MAME game: {} (CRC: {:08X})", entry.name, entry.crc32);
                    self.crc_to_name.insert(entry.crc32, entry.name.clone());
                    self.crc_to_entry.insert(entry.crc32, entry.clone());
                    entries.push(entry);
                }
            } else {
                i += 1;
            }
        }

        if !entries.is_empty() {
            debug!("Loaded {} MAME DAT entries for system '{}'", entries.len(), system_name);
            self.entries.insert(system_name, entries);
        }

        Ok(())
    }

    fn parse_clrmamepro_game_block(&self, content: &str, _line: &str) -> Result<Vec<DatEntry>> {
        // Basic ClrMamePro DAT parsing
        // Format example:
        // game (
        //     name "1942"
        //     description "1942 (Revision B)"
        //     rom ( name "srb-03.m3" size 2048 crc 36d7200e sha1 b13e04e8... )
        // )
        // 
        // Or multiline ROM format:
        // rom
        // (
        //     name "file.bin"
        //     size 32768
        //     crc 4b5b7d8d
        // )
        
        let mut game_name = String::new();
        let mut game_description = String::new();
        let mut entries = Vec::new();
        
        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;
        
        while i < lines.len() {
            let line = lines[i].trim();
            
            // Skip empty lines and comments
            if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
                i += 1;
                continue;
            }
            
            // Parse game name: name "gamename"
            if line.starts_with("name ") && line.contains('"') {
                if let Some(name) = self.extract_quoted_value(line) {
                    game_name = name;
                }
                i += 1;
                continue;
            }
            
            // Parse description: description "Game Description"
            if line.starts_with("description ") && line.contains('"') {
                if let Some(desc) = self.extract_quoted_value(line) {
                    game_description = desc;
                }
                i += 1;
                continue;
            }
            
            // Handle both single line and multiline ROM entries
            if line.contains("rom (") && line.contains("crc") {
                // Single line ROM: rom ( name "file.rom" size 12345 crc abcd1234 )
                if let Some(entry) = self.parse_rom_line(line, &game_name, &game_description) {
                    entries.push(entry);
                }
                i += 1;
            } else if line == "rom" || line.starts_with("rom") {
                // Multiline ROM block
                if let Some((entry, lines_consumed)) = self.parse_multiline_rom_block(&lines[i..], &game_name, &game_description) {
                    entries.push(entry);
                    i += lines_consumed;
                } else {
                    i += 1;
                }
            } else {
                i += 1;
            }
        }
        
        Ok(entries)
    }
    
    fn extract_quoted_value(&self, line: &str) -> Option<String> {
        if let Some(start) = line.find('"') {
            if let Some(end) = line.rfind('"') {
                if start < end {
                    return Some(line[start+1..end].to_string());
                }
            }
        }
        None
    }
    
    fn parse_rom_line(&self, line: &str, game_name: &str, game_description: &str) -> Option<DatEntry> {
        // Parse: rom ( name "file.rom" size 12345 crc abcd1234 sha1 xyz... )
        let mut crc32_str = None;
        let mut size_val = None;
        
        // Split by spaces and look for crc value - handle different CRC formats
        let parts: Vec<&str> = line.split_whitespace().collect();
        for i in 0..parts.len() {
            if parts[i] == "crc" && i + 1 < parts.len() {
                let crc_candidate = parts[i + 1];
                // Remove common prefixes and suffixes from CRC
                let crc_clean = crc_candidate
                    .trim_matches(')')
                    .trim_matches('(')
                    .trim_matches('"');
                crc32_str = Some(crc_clean);
            }
            if parts[i] == "size" && i + 1 < parts.len() {
                if let Ok(size) = parts[i + 1].parse::<u64>() {
                    size_val = Some(size);
                }
            }
        }
        
        if let Some(crc_str) = crc32_str {
            // Try parsing with and without 0x prefix
            let crc_result = if crc_str.starts_with("0x") || crc_str.starts_with("0X") {
                u32::from_str_radix(&crc_str[2..], 16)
            } else {
                u32::from_str_radix(crc_str, 16)
            };
            
            if let Ok(crc32) = crc_result {
                let display_name = if !game_description.is_empty() {
                    game_description.to_string()
                } else if !game_name.is_empty() {
                    game_name.to_string()
                } else {
                    format!("Unknown ROM (CRC: {:08X})", crc32)
                };
                
                return Some(DatEntry {
                    crc32,
                    name: display_name,
                    size: size_val,
                    md5: None,
                    sha1: None,
                });
            }
        }
        
        None
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
        // Primary DAT lookup - this is now the first step for MAME systems
        // The returned name will be used to lookup description in MAME XML
        if let Some(name) = self.crc_to_name.get(&crc32) {
            return Some(name.clone());
        }

        // No luck with DAT lookup
        if let Some(ref _mame_db) = self.mame_xml_db {
            debug!("DAT lookup failed for CRC32 {:08X}, will fallback to filename-based MAME XML lookup", crc32);
        }

        None
    }

    /// Get ROM name by filename for MAME ROMs with enhanced DAT→XML lookup
    /// New strategy: DAT → XML → Description
    /// 1. Try to get ROM name from filename (without extension)
    /// 2. If found in MAME XML, get the description from XML
    /// 3. This provides better labels than raw DAT names
    pub fn get_mame_name_by_filename(&self, filename: &str) -> Option<String> {
        if let Some(ref mame_db) = self.mame_xml_db {
            // Remove file extension and try to match ROM name
            let rom_name = filename
                .split('.')
                .next()
                .unwrap_or(filename);
            
            if let Some(description) = mame_db.get_description(rom_name) {
                debug!("MAME XML lookup by filename: {} -> {}", rom_name, description);
                return Some(description.to_string());
            }
        }
        None
    }

    /// Enhanced MAME lookup: DAT name → XML description  
    /// This is the new primary method for MAME systems
    /// 1. First get ROM name from DAT by CRC32
    /// 2. Then use that name to lookup description in MAME XML
    /// 3. Return the XML description (more informative)
    pub fn get_mame_description_by_dat_name(&self, dat_name: &str) -> Option<String> {
        if let Some(ref mame_db) = self.mame_xml_db {
            if let Some(description) = mame_db.get_description(dat_name) {
                debug!("MAME DAT→XML lookup: {} -> {}", dat_name, description);
                return Some(description.to_string());
            }
        }
        None
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

    fn parse_multiline_rom_block(&self, lines: &[&str], game_name: &str, game_description: &str) -> Option<(DatEntry, usize)> {
        // Parse multiline ROM block format:
        // rom
        // (
        //     name "file.bin"
        //     size 32768
        //     crc 4b5b7d8d
        //     sha1 ...
        // )
        
        let mut i = 0;
        if i >= lines.len() {
            return None;
        }
        
        // First line should be "rom" or start with "rom"
        let first_line = lines[i].trim();
        if !first_line.starts_with("rom") {
            return None;
        }
        i += 1;
        
        // Skip to opening brace
        while i < lines.len() {
            let line = lines[i].trim();
            if line == "(" {
                i += 1;
                break;
            } else if line.starts_with("rom (") {
                // Handle "rom (" case
                break;
            }
            i += 1;
        }
        
        let mut crc32_str = None;
        let mut size_val = None;
        let mut rom_name = String::new();
        let mut brace_count = 1;
        
        // Parse ROM attributes until we close the block
        while i < lines.len() && brace_count > 0 {
            let line = lines[i].trim();
            
            // Count braces
            let open_count = line.matches('(').count();
            let close_count = line.matches(')').count();
            brace_count += open_count;
            brace_count -= close_count;
            
            // Parse attributes
            if line.starts_with("name ") && line.contains('"') {
                if let Some(name) = self.extract_quoted_value(line) {
                    rom_name = name;
                }
            } else if line.starts_with("size ") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() > 1 {
                    if let Ok(size) = parts[1].parse::<u64>() {
                        size_val = Some(size);
                    }
                }
            } else if line.starts_with("crc ") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() > 1 {
                    let crc_candidate = parts[1];
                    // Remove common prefixes and suffixes from CRC
                    let crc_clean = crc_candidate
                        .trim_matches(')')
                        .trim_matches('(')
                        .trim_matches('"');
                    crc32_str = Some(crc_clean);
                }
            }
            
            i += 1;
        }
        
        // Try to create DatEntry if we have a CRC
        if let Some(crc_str) = crc32_str {
            if let Ok(crc32_val) = u32::from_str_radix(crc_str, 16) {
                let name = if !game_description.is_empty() {
                    game_description.to_string()
                } else if !game_name.is_empty() {
                    game_name.to_string()
                } else {
                    rom_name.clone()
                };
                
                let entry = DatEntry {
                    name,
                    crc32: crc32_val,
                    size: size_val,
                    md5: None,
                    sha1: None,
                };
                
                return Some((entry, i));
            }
        }
        
        None
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
    use std::path::PathBuf;
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
