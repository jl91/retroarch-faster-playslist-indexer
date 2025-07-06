use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use crate::scanner::RomFile;
use crate::platform::{Platform, PlatformPathConverter};
use crate::dat_parser::DatCollection;
use crate::core_mapper::CoreMapper;
use crate::error::PlaylistError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_core_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_core_name: Option<String>,
    #[serde(default)]
    pub label_display_mode: u32,
    #[serde(default)]
    pub right_thumbnail_mode: u32,
    #[serde(default)]
    pub left_thumbnail_mode: u32,
    #[serde(default)]
    pub sort_mode: u32,
    pub items: Vec<PlaylistItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistItem {
    pub path: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crc32: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_name: Option<String>,
}

impl Playlist {
    pub fn new() -> Self {
        Self {
            version: "1.5".to_string(),
            default_core_path: None,
            default_core_name: None,
            label_display_mode: 0,
            right_thumbnail_mode: 0,
            left_thumbnail_mode: 0,
            sort_mode: 0,
            items: Vec::new(),
        }
    }

    pub fn with_default_core(mut self, core_path: String, core_name: String) -> Self {
        self.default_core_path = Some(core_path);
        self.default_core_name = Some(core_name);
        self
    }

    pub fn add_item(&mut self, item: PlaylistItem) {
        self.items.push(item);
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self)
            .map_err(PlaylistError::SerializationFailed)?;

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| PlaylistError::SaveFailed {
                    path: path.to_path_buf(),
                    source: e,
                })?;
        }

        std::fs::write(path, json)
            .map_err(|e| PlaylistError::SaveFailed {
                path: path.to_path_buf(),
                source: e,
            })?;

        Ok(())
    }

    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| PlaylistError::LoadFailed {
                path: path.to_path_buf(),
                source: e,
            })?;

        let playlist: Playlist = serde_json::from_str(&content)
            .map_err(PlaylistError::SerializationFailed)?;

        Ok(playlist)
    }

    pub fn sort_by_label(&mut self) {
        self.items.sort_by(|a, b| a.label.cmp(&b.label));
    }

    pub fn deduplicate(&mut self) {
        let mut seen = std::collections::HashSet::new();
        self.items.retain(|item| {
            let key = (item.path.clone(), item.crc32.clone());
            seen.insert(key)
        });
    }
}

impl Default for Playlist {
    fn default() -> Self {
        Self::new()
    }
}

impl PlaylistItem {
    pub fn new(path: String, label: String) -> Self {
        Self {
            path,
            label,
            core_path: None,
            core_name: None,
            crc32: None,
            db_name: None,
        }
    }

    pub fn with_core(mut self, core_path: String, core_name: String) -> Self {
        self.core_path = Some(core_path);
        self.core_name = Some(core_name);
        self
    }

    pub fn with_crc32(mut self, crc32: u32) -> Self {
        self.crc32 = Some(format!("{:08X}|crc", crc32));
        self
    }

    pub fn with_db_name(mut self, db_name: String) -> Self {
        self.db_name = Some(db_name);
        self
    }
}

pub struct PlaylistBuilder {
    source_platform: Platform,
    target_platform: Platform,
    path_converter: PlatformPathConverter,
    dat_collection: DatCollection,
    core_mapper: CoreMapper,
    verbose: bool,
}

impl PlaylistBuilder {
    pub fn new() -> Self {
        let source = Platform::Windows; // Default
        let target = Platform::Windows; // Default
        let path_converter = PlatformPathConverter::new(source, target);
        
        Self {
            source_platform: source,
            target_platform: target,
            path_converter,
            dat_collection: DatCollection::new(),
            core_mapper: CoreMapper::new(),
            verbose: false,
        }
    }

    pub fn with_platforms(mut self, source: Platform, target: Platform) -> Self {
        self.source_platform = source;
        self.target_platform = target;
        self.path_converter = PlatformPathConverter::new(source, target);
        self
    }

    pub fn with_dat_collection(mut self, dat_collection: DatCollection) -> Self {
        self.dat_collection = dat_collection;
        self
    }

    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    pub fn build_by_system(&self, roms: &[RomFile]) -> Result<HashMap<String, Playlist>> {
        let mut playlists: HashMap<String, Playlist> = HashMap::new();

        for rom in roms {
            if let Some(system) = &rom.system {
                let playlist = playlists.entry(system.clone()).or_insert_with(|| {
                    let mut playlist = Playlist::new();
                    
                    // Set default core for this system
                    if let Some((core_path, core_name)) = self.core_mapper.get_default_core(system, self.target_platform) {
                        playlist = playlist.with_default_core(core_path, core_name);
                    }
                    
                    playlist
                });

                let item = self.create_playlist_item(rom, system)?;
                playlist.add_item(item);
            }
        }

        // Sort all playlists
        for playlist in playlists.values_mut() {
            playlist.sort_by_label();
            playlist.deduplicate();
        }

        Ok(playlists)
    }

    pub fn build_master(&self, roms: &[RomFile]) -> Result<Playlist> {
        let mut playlist = Playlist::new();

        for rom in roms {
            if let Some(system) = &rom.system {
                let item = self.create_playlist_item(rom, system)?;
                playlist.add_item(item);
            }
        }

        playlist.sort_by_label();
        playlist.deduplicate();

        Ok(playlist)
    }

    pub fn build_single_system_playlist(&self, roms: &[RomFile], forced_system: &str) -> Result<Playlist> {
        let mut playlist = Playlist::new();
        
        // Set default core for this system
        if let Some((core_path, core_name)) = self.core_mapper.get_default_core(forced_system, self.target_platform) {
            playlist = playlist.with_default_core(core_path, core_name);
        }

        // Add all ROMs to the playlist as if they belong to the forced system
        for rom in roms {
            let item = self.create_playlist_item(rom, forced_system)?;
            playlist.add_item(item);
        }

        playlist.sort_by_label();
        playlist.deduplicate();

        Ok(playlist)
    }

    fn create_playlist_item(&self, rom: &RomFile, system: &str) -> Result<PlaylistItem> {
        // Convert path to target platform format
        let converted_path = self.path_converter.convert_rom_path(
            &rom.path.to_string_lossy()
        );

        // Get ROM label (try DAT first, then filename)
        let label = if let Some(crc32) = rom.crc32 {
            self.dat_collection.get_name_by_crc(crc32)
                .unwrap_or_else(|| self.clean_filename(&rom.filename))
        } else {
            self.clean_filename(&rom.filename)
        };

        // Get core for this system
        let (core_path, core_name) = self.core_mapper
            .get_default_core(system, self.target_platform)
            .unwrap_or_else(|| ("DETECT".to_string(), "DETECT".to_string()));

        let mut item = PlaylistItem::new(converted_path, label)
            .with_core(core_path, core_name)
            .with_db_name(format!("{}.lpl", system));

        if let Some(crc32) = rom.crc32 {
            item = item.with_crc32(crc32);
        }

        Ok(item)
    }

    fn clean_filename(&self, filename: &str) -> String {
        // Remove file extension
        let name = if let Some(dot_pos) = filename.rfind('.') {
            &filename[..dot_pos]
        } else {
            filename
        };

        // Basic cleaning - remove common ROM tags
        name.replace(['[', ']', '(', ')'], " ")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string()
    }
}

impl Default for PlaylistBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_playlist_creation() {
        let mut playlist = Playlist::new();
        assert_eq!(playlist.version, "1.5");
        assert_eq!(playlist.items.len(), 0);

        let item = PlaylistItem::new(
            "/path/to/game.rom".to_string(),
            "Test Game".to_string(),
        );
        playlist.add_item(item);

        assert_eq!(playlist.items.len(), 1);
        assert_eq!(playlist.items[0].label, "Test Game");
    }

    #[test]
    fn test_playlist_serialization() {
        let mut playlist = Playlist::new();
        let item = PlaylistItem::new(
            "/path/to/game.rom".to_string(),
            "Test Game".to_string(),
        ).with_crc32(0x12345678);
        
        playlist.add_item(item);

        let json = serde_json::to_string_pretty(&playlist).unwrap();
        assert!(json.contains("Test Game"));
        assert!(json.contains("12345678"));
    }

    #[test]
    fn test_filename_cleaning() {
        let builder = PlaylistBuilder::new();
        
        let cleaned = builder.clean_filename("Super Mario Bros (USA) [!].nes");
        assert_eq!(cleaned, "Super Mario Bros USA !");
        
        let cleaned = builder.clean_filename("Zelda [Rev A].z64");
        assert_eq!(cleaned, "Zelda Rev A");
    }
}
