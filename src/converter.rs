use anyhow::Result;
use log::{debug, info, warn};
use std::path::{Path, PathBuf};
use walkdir;

use crate::playlist::Playlist;
use crate::platform::{Platform, PlatformPathConverter};
use crate::error::ConverterError;

pub struct PlaylistConverter {
    validate_paths: bool,
}

impl PlaylistConverter {
    pub fn new() -> Self {
        Self {
            validate_paths: false,
        }
    }

    pub fn with_path_validation(mut self, validate: bool) -> Self {
        self.validate_paths = validate;
        self
    }

    pub fn load_playlist(&self, path: &Path) -> Result<Playlist> {
        info!("Carregando playlist: {}", path.display());
        Playlist::load(path)
    }

    pub fn detect_platform(&self, playlist: &Playlist) -> Result<Platform> {
        // Analyze paths in the playlist to detect the source platform
        for item in &playlist.items {
            if let Some(platform) = self.detect_platform_from_path(&item.path) {
                debug!("Plataforma detectada: {} (baseada no caminho: {})", 
                       platform.display_name(), item.path);
                return Ok(platform);
            }

            if let Some(ref core_path) = item.core_path {
                if let Some(platform) = self.detect_platform_from_core_path(core_path) {
                    debug!("Plataforma detectada: {} (baseada no core: {})", 
                           platform.display_name(), core_path);
                    return Ok(platform);
                }
            }
        }

        Err(ConverterError::SourcePlatformNotDetected.into())
    }

    fn detect_platform_from_path(&self, path: &str) -> Option<Platform> {
        let path_lower = path.to_lowercase();

        // Windows patterns
        if path.contains('\\') || path_lower.starts_with("c:") || path_lower.starts_with("d:") {
            return Some(Platform::Windows);
        }

        // Switch patterns
        if path_lower.contains("/switch/") || path_lower.contains("atmosphere") {
            return Some(Platform::Switch);
        }

        // Android patterns
        if path_lower.contains("/storage/emulated/") || path_lower.contains("/android_asset/") {
            return Some(Platform::Android);
        }

        // Raspberry Pi patterns
        if path_lower.contains("/home/pi/") || path_lower.contains("retropie") {
            return Some(Platform::Raspberry);
        }

        // Steam Deck patterns
        if path_lower.contains("/home/deck/") || path_lower.contains("steamdeck") {
            return Some(Platform::SteamDeck);
        }

        // macOS patterns
        if path_lower.contains("/users/") || path_lower.contains(".app/contents/") {
            return Some(Platform::MacOS);
        }

        // Generic Linux (fallback for Unix-like paths)
        if path.starts_with('/') {
            return Some(Platform::Linux);
        }

        None
    }

    fn detect_platform_from_core_path(&self, core_path: &str) -> Option<Platform> {
        let core_lower = core_path.to_lowercase();

        if core_lower.ends_with(".dll") {
            Some(Platform::Windows)
        } else if core_lower.ends_with(".dylib") {
            Some(Platform::MacOS)
        } else if core_lower.contains("_android.so") {
            Some(Platform::Android)
        } else if core_lower.contains("_libnx.a") {
            Some(Platform::Switch)
        } else if core_lower.ends_with(".so") {
            // Could be Linux, Raspberry Pi, or Steam Deck
            // Use path context to determine
            if core_lower.contains("/home/pi/") || core_lower.contains("retropie") {
                Some(Platform::Raspberry)
            } else if core_lower.contains("/home/deck/") {
                Some(Platform::SteamDeck)
            } else {
                Some(Platform::Linux)
            }
        } else {
            None
        }
    }

    pub fn convert_playlist(
        &self,
        playlist: &Playlist,
        source: Platform,
        target: Platform,
    ) -> Result<Playlist> {
        if source == target {
            info!("Plataformas são iguais, retornando playlist original");
            return Ok(playlist.clone());
        }

        info!("Convertendo playlist de {} para {}", 
              source.display_name(), target.display_name());

        let converter = PlatformPathConverter::new(source, target);
        let mut new_playlist = playlist.clone();

        // Convert default core if present
        if let Some(ref core_path) = playlist.default_core_path {
            new_playlist.default_core_path = Some(converter.convert_core_path(core_path));
        }

        // Convert all items
        for item in &mut new_playlist.items {
            // Convert ROM path
            item.path = converter.convert_rom_path(&item.path);

            // Convert core path if present
            if let Some(ref core_path) = item.core_path {
                item.core_path = Some(converter.convert_core_path(core_path));
            }

            // Validate converted paths if requested
            if self.validate_paths {
                if let Err(e) = self.validate_converted_paths(item, target) {
                    warn!("Validação de caminho falhou para {}: {}", item.label, e);
                }
            }
        }

        info!("Conversão concluída: {} itens processados", new_playlist.items.len());
        Ok(new_playlist)
    }

    fn validate_converted_paths(
        &self,
        item: &crate::playlist::PlaylistItem,
        _target: Platform,
    ) -> Result<()> {
        // TODO: Implement path validation
        // This could check if the converted paths exist, are accessible, etc.
        // For now, we'll just do basic format validation

        if item.path.is_empty() {
            return Err(ConverterError::PathConversionFailed {
                original: "empty".to_string(),
                target: item.path.clone(),
            }.into());
        }

        Ok(())
    }

    pub fn convert_all_playlists(
        &self,
        input_dir: &Path,
        source: Platform,
        target: Platform,
        output_dir: &Path,
    ) -> Result<ConversionReport> {
        let mut report = ConversionReport::new();

        // Find all .lpl files
        let lpl_files: Vec<PathBuf> = walkdir::WalkDir::new(input_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "lpl"))
            .map(|e| e.path().to_path_buf())
            .collect();

        info!("Encontrados {} arquivos de playlist para conversão", lpl_files.len());

        // Ensure output directory exists
        std::fs::create_dir_all(output_dir)?;

        for lpl_file in lpl_files {
            let filename = lpl_file.file_stem()
                .unwrap_or_default()
                .to_string_lossy();

            match self.convert_single_playlist(&lpl_file, source, target, output_dir) {
                Ok(_) => {
                    report.successful_conversions.push(filename.to_string());
                }
                Err(e) => {
                    warn!("Falha ao converter {}: {}", lpl_file.display(), e);
                    report.failed_conversions.push((filename.to_string(), e.to_string()));
                }
            }
        }

        Ok(report)
    }

    fn convert_single_playlist(
        &self,
        input_path: &Path,
        source: Platform,
        target: Platform,
        output_dir: &Path,
    ) -> Result<()> {
        let playlist = self.load_playlist(input_path)?;
        let converted = self.convert_playlist(&playlist, source, target)?;

        let filename = input_path.file_stem()
            .unwrap_or_default()
            .to_string_lossy();
        let output_path = output_dir.join(format!("{} [{}].lpl", filename, target.short_name()));

        converted.save(&output_path)?;
        info!("Playlist convertida salva em: {}", output_path.display());

        Ok(())
    }
}

impl Default for PlaylistConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Default)]
pub struct ConversionReport {
    pub successful_conversions: Vec<String>,
    pub failed_conversions: Vec<(String, String)>,
}

impl ConversionReport {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn total_processed(&self) -> usize {
        self.successful_conversions.len() + self.failed_conversions.len()
    }

    pub fn success_rate(&self) -> f64 {
        if self.total_processed() == 0 {
            0.0
        } else {
            self.successful_conversions.len() as f64 / self.total_processed() as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::playlist::{Playlist, PlaylistItem};

    #[test]
    fn test_platform_detection_from_windows_path() {
        let converter = PlaylistConverter::new();
        
        assert_eq!(
            converter.detect_platform_from_path("C:\\Games\\roms\\game.rom"),
            Some(Platform::Windows)
        );
        
        assert_eq!(
            converter.detect_platform_from_path("D:\\RetroArch\\cores\\core.dll"),
            Some(Platform::Windows)
        );
    }

    #[test]
    fn test_platform_detection_from_switch_path() {
        let converter = PlaylistConverter::new();
        
        assert_eq!(
            converter.detect_platform_from_path("/switch/roms/game.rom"),
            Some(Platform::Switch)
        );
    }

    #[test]
    fn test_platform_detection_from_core_extension() {
        let converter = PlaylistConverter::new();
        
        assert_eq!(
            converter.detect_platform_from_core_path("core_libretro.dll"),
            Some(Platform::Windows)
        );
        
        assert_eq!(
            converter.detect_platform_from_core_path("core_libretro_libnx.a"),
            Some(Platform::Switch)
        );
        
        assert_eq!(
            converter.detect_platform_from_core_path("core_libretro_android.so"),
            Some(Platform::Android)
        );
    }

    #[test]
    fn test_playlist_conversion() {
        let converter = PlaylistConverter::new();
        
        let mut playlist = Playlist::new();
        playlist.add_item(
            PlaylistItem::new(
                "C:\\Games\\roms\\game.rom".to_string(),
                "Test Game".to_string(),
            ).with_core(
                "C:\\RetroArch\\cores\\core_libretro.dll".to_string(),
                "Test Core".to_string(),
            )
        );

        let converted = converter.convert_playlist(
            &playlist,
            Platform::Windows,
            Platform::Switch,
        ).unwrap();

        assert!(converted.items[0].path.contains("/switch/"));
        assert!(converted.items[0].core_path.as_ref().unwrap().contains("_libnx.a"));
    }
}
