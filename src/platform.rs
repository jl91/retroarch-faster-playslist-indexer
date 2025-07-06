use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum Platform {
    Windows,
    Linux,
    MacOS,
    Android,
    Switch,
    Raspberry,
    SteamDeck,
}

impl Platform {
    pub fn display_name(&self) -> &'static str {
        match self {
            Platform::Windows => "Windows (PC)",
            Platform::Linux => "Linux (PC)",
            Platform::MacOS => "macOS",
            Platform::Android => "Android",
            Platform::Switch => "Nintendo Switch",
            Platform::Raspberry => "Raspberry Pi",
            Platform::SteamDeck => "Steam Deck",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            Platform::Windows => "Windows",
            Platform::Linux => "Linux",
            Platform::MacOS => "macOS",
            Platform::Android => "Android",
            Platform::Switch => "Switch",
            Platform::Raspberry => "Raspberry",
            Platform::SteamDeck => "SteamDeck",
        }
    }

    pub fn default_roms_path(&self) -> &'static str {
        match self {
            Platform::Windows => "D:/Games/ROMs",
            Platform::Linux => "/home/user/ROMs",
            Platform::MacOS => "/Users/user/ROMs",
            Platform::Android => "/storage/emulated/0/RetroArch/roms",
            Platform::Switch => "/retroarch/roms",
            Platform::Raspberry => "/home/pi/RetroPie/roms",
            Platform::SteamDeck => "/home/deck/ROMs",
        }
    }

    pub fn default_cores_path(&self) -> &'static str {
        match self {
            Platform::Windows => "C:/RetroArch/cores",
            Platform::Linux => "/usr/lib/libretro",
            Platform::MacOS => "/Applications/RetroArch.app/Contents/Resources/cores",
            Platform::Android => "/data/data/com.retroarch/cores",
            Platform::Switch => "/switch/retroarch/cores",
            Platform::Raspberry => "/opt/retropie/libretrocores",
            Platform::SteamDeck => "/home/deck/.var/app/org.libretro.RetroArch/config/retroarch/cores",
        }
    }

    pub fn core_extension(&self) -> &'static str {
        match self {
            Platform::Windows => ".dll",
            Platform::Linux | Platform::Raspberry | Platform::SteamDeck => ".so",
            Platform::MacOS => ".dylib",
            Platform::Android => "_android.so",
            Platform::Switch => "_libnx.a",
        }
    }

    pub fn path_separator(&self) -> char {
        match self {
            Platform::Windows => '\\',
            _ => '/',
        }
    }

    pub fn is_unix_like(&self) -> bool {
        !matches!(self, Platform::Windows)
    }

    pub fn supports_archives(&self) -> bool {
        // All platforms support basic ZIP archives
        true
    }

    pub fn has_case_sensitive_filesystem(&self) -> bool {
        match self {
            Platform::Windows => false,
            Platform::MacOS => false, // Usually case-insensitive by default
            _ => true,
        }
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.short_name())
    }
}

#[derive(Debug, Clone)]
pub struct PlatformPathConverter {
    source: Platform,
    target: Platform,
}

impl PlatformPathConverter {
    pub fn new(source: Platform, target: Platform) -> Self {
        Self { source, target }
    }

    pub fn convert_rom_path(&self, path: &str) -> String {
        if self.source == self.target {
            return path.to_string();
        }

        // Extract relative path from source
        let relative_path = self.extract_relative_path(path);
        
        // Convert to target platform format
        self.build_target_path(&relative_path, true)
    }

    pub fn convert_core_path(&self, path: &str) -> String {
        if self.source == self.target {
            return path.to_string();
        }

        // Extract core name from path
        let core_name = self.extract_core_name(path);
        
        // Build target core path
        format!("{}/{}{}",
            self.target.default_cores_path(),
            core_name,
            self.target.core_extension()
        )
    }

    fn extract_relative_path(&self, path: &str) -> String {
        // Try to find common ROM directory patterns and extract relative path
        let normalized = path.replace('\\', "/");
        
        // Common patterns to strip
        let patterns = [
            "roms/", "ROMs/", "games/", "Games/", 
            "/roms/", "/ROMs/", "/games/", "/Games/",
            "RetroPie/roms/", "RetroArch/roms/",
            "/retroarch/roms/", "/storage/emulated/0/RetroArch/roms/",
        ];

        for pattern in &patterns {
            if let Some(pos) = normalized.find(pattern) {
                return normalized[pos + pattern.len()..].to_string();
            }
        }

        // If no pattern found, try to extract from the last few path components
        let parts: Vec<&str> = normalized.split('/').collect();
        if parts.len() >= 2 {
            // Assume last 2 components are system/rom
            return format!("{}/{}", parts[parts.len() - 2], parts[parts.len() - 1]);
        }

        normalized
    }

    fn extract_core_name(&self, path: &str) -> String {
        let filename = std::path::Path::new(path)
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy();

        // Remove platform-specific suffixes
        let core_name = filename
            .replace("_libretro", "")
            .replace("_android", "")
            .replace("_libnx", "");

        core_name
    }

    fn build_target_path(&self, relative_path: &str, is_rom: bool) -> String {
        let base_path = if is_rom {
            self.target.default_roms_path()
        } else {
            self.target.default_cores_path()
        };

        let separator = self.target.path_separator();
        let converted_path = relative_path.replace(['/', '\\'], &separator.to_string());

        format!("{}{}{}", base_path, separator, converted_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_display_names() {
        assert_eq!(Platform::Windows.display_name(), "Windows (PC)");
        assert_eq!(Platform::Switch.display_name(), "Nintendo Switch");
        assert_eq!(Platform::SteamDeck.display_name(), "Steam Deck");
    }

    #[test]
    fn test_path_conversion() {
        let converter = PlatformPathConverter::new(Platform::Windows, Platform::Switch);
        
        let windows_path = "D:\\ROMs\\n64\\Super Mario 64.z64";
        let switch_path = converter.convert_rom_path(windows_path);
        assert!(switch_path.contains("/retroarch/roms"));
        assert!(switch_path.contains("Super Mario 64.z64"));
    }

    #[test]
    fn test_core_conversion() {
        let converter = PlatformPathConverter::new(Platform::Windows, Platform::Switch);
        
        let windows_core = "C:\\RetroArch\\cores\\mupen64plus_next_libretro.dll";
        let switch_core = converter.convert_core_path(windows_core);
        assert!(switch_core.contains("mupen64plus_next"));
        assert!(switch_core.ends_with("_libnx.a"));
    }
}
