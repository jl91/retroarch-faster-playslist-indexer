use std::collections::HashMap;
use std::fs;
use std::path::Path;
use anyhow::{Result, Context};
use quick_xml::Reader;
use quick_xml::events::Event;

/// MAME XML database parser for MAME 2003-Plus
/// Used as fallback when DAT files don't contain the ROM information
#[derive(Debug, Clone)]
pub struct MameXmlDatabase {
    /// Maps ROM name -> game description (label)
    pub rom_to_description: HashMap<String, String>,
    /// Maps ROM name -> additional metadata
    pub rom_metadata: HashMap<String, MameGameInfo>,
}

#[derive(Debug, Clone)]
pub struct MameGameInfo {
    pub name: String,
    pub description: String,
    pub year: Option<String>,
    pub manufacturer: Option<String>,
    pub clone_of: Option<String>,
    pub rom_of: Option<String>,
}

impl MameXmlDatabase {
    /// Create a new empty MAME XML database
    pub fn new() -> Self {
        Self {
            rom_to_description: HashMap::new(),
            rom_metadata: HashMap::new(),
        }
    }

    /// Load MAME XML database from file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read MAME XML file: {}", path.display()))?;
        
        Self::parse_xml_content(&content)
    }

    /// Load embedded MAME XML database
    pub fn load_embedded() -> Result<Self> {
        let content = include_str!("../assets/mame-2003-plus-database.xml");
        Self::parse_xml_content(content)
    }

    /// Parse XML content and extract game information
    fn parse_xml_content(content: &str) -> Result<Self> {
        let mut database = Self::new();
        let mut reader = Reader::from_str(content);
        reader.config_mut().trim_text(true);

        let mut buf = Vec::new();
        let mut current_game: Option<MameGameInfo> = None;
        let mut current_text = String::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    let element_name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    
                    if element_name == "game" {
                        // Parse game attributes
                        let mut game_info = MameGameInfo {
                            name: String::new(),
                            description: String::new(),
                            year: None,
                            manufacturer: None,
                            clone_of: None,
                            rom_of: None,
                        };

                        for attr in e.attributes() {
                            if let Ok(attr) = attr {
                                let key = String::from_utf8_lossy(attr.key.as_ref());
                                let value = String::from_utf8_lossy(&attr.value);
                                
                                match key.as_ref() {
                                    "name" => game_info.name = value.to_string(),
                                    "cloneof" => game_info.clone_of = Some(value.to_string()),
                                    "romof" => game_info.rom_of = Some(value.to_string()),
                                    _ => {}
                                }
                            }
                        }

                        current_game = Some(game_info);
                    }
                }
                Ok(Event::Text(e)) => {
                    current_text = e.unescape()?.to_string();
                }
                Ok(Event::End(ref e)) => {
                    let element_name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    
                    if let Some(ref mut game) = current_game {
                        match element_name.as_str() {
                            "description" => {
                                game.description = current_text.clone();
                            }
                            "year" => {
                                game.year = Some(current_text.clone());
                            }
                            "manufacturer" => {
                                game.manufacturer = Some(current_text.clone());
                            }
                            "game" => {
                                // Finished parsing this game, add to database
                                if !game.name.is_empty() && !game.description.is_empty() {
                                    database.rom_to_description.insert(
                                        game.name.clone(),
                                        game.description.clone()
                                    );
                                    database.rom_metadata.insert(
                                        game.name.clone(),
                                        game.clone()
                                    );
                                }
                                current_game = None;
                            }
                            _ => {}
                        }
                    }
                    current_text.clear();
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(anyhow::anyhow!("Error parsing MAME XML: {}", e)),
                _ => {}
            }
            buf.clear();
        }

        Ok(database)
    }

    /// Get game description (label) by ROM name
    pub fn get_description(&self, rom_name: &str) -> Option<&str> {
        self.rom_to_description.get(rom_name).map(|s| s.as_str())
    }

    /// Get full game metadata by ROM name
    pub fn get_game_info(&self, rom_name: &str) -> Option<&MameGameInfo> {
        self.rom_metadata.get(rom_name)
    }

    /// Get all ROM names in the database
    pub fn get_all_rom_names(&self) -> Vec<&str> {
        self.rom_to_description.keys().map(|s| s.as_str()).collect()
    }

    /// Check if ROM exists in database
    pub fn contains_rom(&self, rom_name: &str) -> bool {
        self.rom_to_description.contains_key(rom_name)
    }

    /// Get database statistics
    pub fn stats(&self) -> (usize, usize) {
        (self.rom_to_description.len(), self.rom_metadata.len())
    }
}

impl Default for MameXmlDatabase {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mame_xml_parsing() {
        let xml_content = r#"
        <mame>
            <game name="puckman" sourcefile="pacman.c">
                <description>PuckMan (Japan set 1)</description>
                <year>1980</year>
                <manufacturer>Namco</manufacturer>
            </game>
            <game name="pacman" cloneof="puckman" romof="puckman">
                <description>Pac-Man (Midway)</description>
                <year>1980</year>
                <manufacturer>Midway</manufacturer>
            </game>
        </mame>
        "#;

        let database = MameXmlDatabase::parse_xml_content(xml_content).unwrap();
        
        assert_eq!(database.get_description("puckman"), Some("PuckMan (Japan set 1)"));
        assert_eq!(database.get_description("pacman"), Some("Pac-Man (Midway)"));
        assert!(database.contains_rom("puckman"));
        assert!(database.contains_rom("pacman"));
        assert!(!database.contains_rom("nonexistent"));
        
        let game_info = database.get_game_info("pacman").unwrap();
        assert_eq!(game_info.year, Some("1980".to_string()));
        assert_eq!(game_info.manufacturer, Some("Midway".to_string()));
        assert_eq!(game_info.clone_of, Some("puckman".to_string()));
    }
}
