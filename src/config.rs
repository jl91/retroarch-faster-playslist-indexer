use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::platform::Platform;
use crate::error::ConfigError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub paths: PathsConfig,
    pub cores: CoresConfig,
    pub dat: DatConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub source_platform: Option<Platform>,
    pub target_platform: Option<Platform>,
    pub auto_download_dats: bool,
    pub create_master_playlist: bool,
    pub threads: Option<usize>,
    pub calculate_crc: bool,
    pub recursive_scan: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PathsConfig {
    pub roms_directories: Vec<PathBuf>,
    pub output_directory: PathBuf,
    pub dat_directory: Option<PathBuf>,
    pub custom_extensions: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoresConfig {
    pub override_system_cores: std::collections::HashMap<String, String>,
    pub custom_core_paths: std::collections::HashMap<Platform, PathBuf>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatConfig {
    pub auto_download: bool,
    pub download_sources: Vec<String>,
    pub update_interval_days: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig {
                source_platform: None,
                target_platform: None,
                auto_download_dats: false,
                create_master_playlist: true,
                threads: None,
                calculate_crc: true,
                recursive_scan: true,
            },
            paths: PathsConfig {
                roms_directories: Vec::new(),
                output_directory: PathBuf::from("./playlists"),
                dat_directory: Some(PathBuf::from("./dats")),
                custom_extensions: None,
            },
            cores: CoresConfig {
                override_system_cores: std::collections::HashMap::new(),
                custom_core_paths: std::collections::HashMap::new(),
            },
            dat: DatConfig {
                auto_download: false,
                download_sources: vec![
                    "https://datomatic.no-intro.org/".to_string(),
                    "http://redump.org/".to_string(),
                ],
                update_interval_days: 30,
            },
        }
    }
}

impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| ConfigError::LoadFailed {
                path: path.to_path_buf(),
                source: e,
            })?;

        let config: Config = toml::from_str(&content)
            .map_err(ConfigError::InvalidFormat)?;

        Ok(config)
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let toml_content = toml::to_string_pretty(self)
            .map_err(ConfigError::SerializationFailed)?;

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| ConfigError::SaveFailed {
                    path: path.to_path_buf(),
                    source: e,
                })?;
        }

        std::fs::write(path, toml_content)
            .map_err(|e| ConfigError::SaveFailed {
                path: path.to_path_buf(),
                source: e,
            })?;

        Ok(())
    }

    pub fn load_or_create(path: Option<&Path>) -> Result<Self> {
        let config_path = path.unwrap_or_else(|| {
            Path::new("config.toml")
        });

        if config_path.exists() {
            Self::load(config_path)
        } else {
            let config = Self::default();
            if let Err(e) = config.save(config_path) {
                log::warn!("Falha ao salvar configuração padrão: {}", e);
            }
            Ok(config)
        }
    }

    pub fn get_default_config_path() -> PathBuf {
        // Try to use system config directory
        if let Some(config_dir) = dirs::config_dir() {
            config_dir.join("retroarch-indexer").join("config.toml")
        } else {
            PathBuf::from("config.toml")
        }
    }

    pub fn merge_with_cli_args(&mut self, args: &crate::cli::Args) {
        // Override config with CLI arguments
        if let Some(source) = args.source_platform {
            self.general.source_platform = Some(source);
        }

        if let Some(target) = args.target_platform {
            self.general.target_platform = Some(target);
        }

        if !args.roms_dirs.is_empty() {
            self.paths.roms_directories = args.roms_dirs.clone();
        }

        self.paths.output_directory = args.output_dir.clone();

        if let Some(ref dat_dir) = args.dat_dir {
            self.paths.dat_directory = Some(dat_dir.clone());
        }

        if let Some(ref extensions) = args.extensions {
            self.paths.custom_extensions = Some(extensions.clone());
        }

        if let Some(threads) = args.threads {
            self.general.threads = Some(threads);
        }

        if args.auto_download_dats {
            self.dat.auto_download = true;
        }

        if args.skip_master {
            self.general.create_master_playlist = false;
        }

        if args.no_crc {
            self.general.calculate_crc = false;
        }

        if args.no_recursive {
            self.general.recursive_scan = false;
        }
    }

    pub fn validate(&self) -> Result<()> {
        // Validate configuration
        if self.paths.roms_directories.is_empty() {
            return Err(anyhow::anyhow!("Nenhum diretório de ROMs configurado"));
        }

        for roms_dir in &self.paths.roms_directories {
            if !roms_dir.exists() {
                log::warn!("Diretório de ROMs não existe: {}", roms_dir.display());
            }
        }

        if let Some(ref dat_dir) = self.paths.dat_directory {
            if !dat_dir.exists() {
                log::info!("Diretório DAT não existe (será criado): {}", dat_dir.display());
            }
        }

        Ok(())
    }
}

// Helper function to create a sample config file
pub fn create_sample_config() -> Config {
    let mut config = Config::default();

    // Add some example values
    config.general.source_platform = Some(Platform::Windows);
    config.general.target_platform = Some(Platform::Switch);
    config.general.threads = Some(8);

    config.paths.roms_directories = vec![
        PathBuf::from("D:/Games/ROMs"),
        PathBuf::from("E:/RetroGaming/ROMs"),
    ];

    config.paths.output_directory = PathBuf::from("./playlists");
    config.paths.dat_directory = Some(PathBuf::from("./dats"));

    // Add some system core overrides
    config.cores.override_system_cores.insert(
        "Nintendo - Nintendo 64".to_string(),
        "parallel_n64".to_string(),
    );

    config.dat.auto_download = true;

    config
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_config_serialization() {
        let config = create_sample_config();
        
        let toml_content = toml::to_string_pretty(&config).unwrap();
        
        assert!(toml_content.contains("[general]"));
        assert!(toml_content.contains("[paths]"));
        assert!(toml_content.contains("[cores]"));
        assert!(toml_content.contains("[dat]"));
    }

    #[test]
    fn test_config_save_and_load() {
        let config = create_sample_config();
        let temp_file = NamedTempFile::new().unwrap();
        
        // Save config
        config.save(temp_file.path()).unwrap();
        
        // Load config
        let loaded_config = Config::load(temp_file.path()).unwrap();
        
        assert_eq!(config.general.source_platform, loaded_config.general.source_platform);
        assert_eq!(config.general.target_platform, loaded_config.general.target_platform);
        assert_eq!(config.paths.roms_directories, loaded_config.paths.roms_directories);
    }

    #[test]
    fn test_default_config() {
        let config = Config::default();
        
        assert_eq!(config.general.create_master_playlist, true);
        assert_eq!(config.general.calculate_crc, true);
        assert_eq!(config.general.recursive_scan, true);
        assert_eq!(config.paths.output_directory, PathBuf::from("./playlists"));
    }

    #[test]
    fn test_config_validation() {
        let mut config = Config::default();
        
        // Empty roms directories should fail validation
        let result = config.validate();
        assert!(result.is_err());
        
        // Add a roms directory
        config.paths.roms_directories.push(PathBuf::from("/nonexistent"));
        let result = config.validate();
        assert!(result.is_ok()); // Should pass validation but log a warning
    }
}
