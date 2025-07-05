use std::collections::HashMap;
use crate::platform::Platform;

#[derive(Debug, Clone)]
pub struct CoreInfo {
    pub display_name: String,
    pub library_name: String,
    pub supported_systems: Vec<String>,
    pub platform_cores: HashMap<Platform, String>,
}

impl CoreInfo {
    pub fn new(display_name: String, library_name: String) -> Self {
        Self {
            display_name,
            library_name,
            supported_systems: Vec::new(),
            platform_cores: HashMap::new(),
        }
    }

    pub fn with_systems(mut self, systems: Vec<String>) -> Self {
        self.supported_systems = systems;
        self
    }

    pub fn with_platform_core(mut self, platform: Platform, core_path: String) -> Self {
        self.platform_cores.insert(platform, core_path);
        self
    }

    pub fn get_core_for_platform(&self, platform: Platform) -> Option<String> {
        self.platform_cores.get(&platform).cloned()
    }
}

#[derive(Debug, Default)]
pub struct CoreMapper {
    // System name -> preferred core mapping
    system_cores: HashMap<String, String>,
    // Core name -> CoreInfo mapping
    cores: HashMap<String, CoreInfo>,
}

impl CoreMapper {
    pub fn new() -> Self {
        let mut mapper = Self::default();
        mapper.initialize_default_mappings();
        mapper
    }

    fn initialize_default_mappings(&mut self) {
        // Nintendo 64
        let mupen64plus = CoreInfo::new(
            "Mupen64Plus-Next".to_string(),
            "mupen64plus_next".to_string(),
        )
        .with_systems(vec!["Nintendo - Nintendo 64".to_string()])
        .with_platform_core(Platform::Windows, "mupen64plus_next_libretro.dll".to_string())
        .with_platform_core(Platform::Linux, "mupen64plus_next_libretro.so".to_string())
        .with_platform_core(Platform::MacOS, "mupen64plus_next_libretro.dylib".to_string())
        .with_platform_core(Platform::Android, "mupen64plus_next_libretro_android.so".to_string())
        .with_platform_core(Platform::Switch, "mupen64plus_next_libretro_libnx.a".to_string());

        self.cores.insert("mupen64plus_next".to_string(), mupen64plus);
        self.system_cores.insert("Nintendo - Nintendo 64".to_string(), "mupen64plus_next".to_string());

        // SNES
        let snes9x = CoreInfo::new(
            "Snes9x".to_string(),
            "snes9x".to_string(),
        )
        .with_systems(vec!["Nintendo - Super Nintendo Entertainment System".to_string()])
        .with_platform_core(Platform::Windows, "snes9x_libretro.dll".to_string())
        .with_platform_core(Platform::Linux, "snes9x_libretro.so".to_string())
        .with_platform_core(Platform::MacOS, "snes9x_libretro.dylib".to_string())
        .with_platform_core(Platform::Android, "snes9x_libretro_android.so".to_string())
        .with_platform_core(Platform::Switch, "snes9x_libretro_libnx.a".to_string());

        self.cores.insert("snes9x".to_string(), snes9x);
        self.system_cores.insert("Nintendo - Super Nintendo Entertainment System".to_string(), "snes9x".to_string());

        // Game Boy Advance
        let mgba = CoreInfo::new(
            "mGBA".to_string(),
            "mgba".to_string(),
        )
        .with_systems(vec![
            "Nintendo - Game Boy Advance".to_string(),
            "Nintendo - Game Boy Color".to_string(),
            "Nintendo - Game Boy".to_string(),
        ])
        .with_platform_core(Platform::Windows, "mgba_libretro.dll".to_string())
        .with_platform_core(Platform::Linux, "mgba_libretro.so".to_string())
        .with_platform_core(Platform::MacOS, "mgba_libretro.dylib".to_string())
        .with_platform_core(Platform::Android, "mgba_libretro_android.so".to_string())
        .with_platform_core(Platform::Switch, "mgba_libretro_libnx.a".to_string());

        self.cores.insert("mgba".to_string(), mgba);
        self.system_cores.insert("Nintendo - Game Boy Advance".to_string(), "mgba".to_string());
        self.system_cores.insert("Nintendo - Game Boy Color".to_string(), "mgba".to_string());
        self.system_cores.insert("Nintendo - Game Boy".to_string(), "mgba".to_string());

        // Genesis/Mega Drive
        let genesis_plus_gx = CoreInfo::new(
            "Genesis Plus GX".to_string(),
            "genesis_plus_gx".to_string(),
        )
        .with_systems(vec![
            "Sega - Mega Drive - Genesis".to_string(),
            "Sega - Master System - Mark III".to_string(),
            "Sega - Game Gear".to_string(),
        ])
        .with_platform_core(Platform::Windows, "genesis_plus_gx_libretro.dll".to_string())
        .with_platform_core(Platform::Linux, "genesis_plus_gx_libretro.so".to_string())
        .with_platform_core(Platform::MacOS, "genesis_plus_gx_libretro.dylib".to_string())
        .with_platform_core(Platform::Android, "genesis_plus_gx_libretro_android.so".to_string())
        .with_platform_core(Platform::Switch, "genesis_plus_gx_libretro_libnx.a".to_string());

        self.cores.insert("genesis_plus_gx".to_string(), genesis_plus_gx);
        self.system_cores.insert("Sega - Mega Drive - Genesis".to_string(), "genesis_plus_gx".to_string());
        self.system_cores.insert("Sega - Master System - Mark III".to_string(), "genesis_plus_gx".to_string());
        self.system_cores.insert("Sega - Game Gear".to_string(), "genesis_plus_gx".to_string());

        // PlayStation
        let pcsx_rearmed = CoreInfo::new(
            "PCSX ReARMed".to_string(),
            "pcsx_rearmed".to_string(),
        )
        .with_systems(vec!["Sony - PlayStation".to_string()])
        .with_platform_core(Platform::Windows, "pcsx_rearmed_libretro.dll".to_string())
        .with_platform_core(Platform::Linux, "pcsx_rearmed_libretro.so".to_string())
        .with_platform_core(Platform::MacOS, "pcsx_rearmed_libretro.dylib".to_string())
        .with_platform_core(Platform::Android, "pcsx_rearmed_libretro_android.so".to_string())
        .with_platform_core(Platform::Switch, "pcsx_rearmed_libretro_libnx.a".to_string());

        self.cores.insert("pcsx_rearmed".to_string(), pcsx_rearmed);
        self.system_cores.insert("Sony - PlayStation".to_string(), "pcsx_rearmed".to_string());

        // NES
        let fceumm = CoreInfo::new(
            "FCEUmm".to_string(),
            "fceumm".to_string(),
        )
        .with_systems(vec!["Nintendo - Nintendo Entertainment System".to_string()])
        .with_platform_core(Platform::Windows, "fceumm_libretro.dll".to_string())
        .with_platform_core(Platform::Linux, "fceumm_libretro.so".to_string())
        .with_platform_core(Platform::MacOS, "fceumm_libretro.dylib".to_string())
        .with_platform_core(Platform::Android, "fceumm_libretro_android.so".to_string())
        .with_platform_core(Platform::Switch, "fceumm_libretro_libnx.a".to_string());

        self.cores.insert("fceumm".to_string(), fceumm);
        self.system_cores.insert("Nintendo - Nintendo Entertainment System".to_string(), "fceumm".to_string());

        // MAME
        let mame_current = CoreInfo::new(
            "MAME (Current)".to_string(),
            "mame".to_string(),
        )
        .with_systems(vec!["MAME".to_string()])
        .with_platform_core(Platform::Windows, "mame_libretro.dll".to_string())
        .with_platform_core(Platform::Linux, "mame_libretro.so".to_string())
        .with_platform_core(Platform::MacOS, "mame_libretro.dylib".to_string())
        .with_platform_core(Platform::Android, "mame_libretro_android.so".to_string());

        self.cores.insert("mame".to_string(), mame_current);
        self.system_cores.insert("MAME".to_string(), "mame".to_string());
    }

    pub fn get_default_core(&self, system: &str, platform: Platform) -> Option<(String, String)> {
        let core_name = self.system_cores.get(system)?;
        let core_info = self.cores.get(core_name)?;
        
        let core_path = if let Some(path) = core_info.get_core_for_platform(platform) {
            format!("{}/{}", platform.default_cores_path(), path)
        } else {
            // Fallback to generic detection
            "DETECT".to_string()
        };

        Some((core_path, core_info.display_name.clone()))
    }

    pub fn get_core_info(&self, core_name: &str) -> Option<&CoreInfo> {
        self.cores.get(core_name)
    }

    pub fn add_core(&mut self, core_name: String, core_info: CoreInfo) {
        for system in &core_info.supported_systems {
            self.system_cores.insert(system.clone(), core_name.clone());
        }
        self.cores.insert(core_name, core_info);
    }

    pub fn get_supported_systems(&self) -> Vec<String> {
        self.system_cores.keys().cloned().collect()
    }

    pub fn get_all_cores(&self) -> &HashMap<String, CoreInfo> {
        &self.cores
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_mapping() {
        let mapper = CoreMapper::new();
        
        let (core_path, core_name) = mapper
            .get_default_core("Nintendo - Nintendo 64", Platform::Windows)
            .unwrap();
        
        assert!(core_path.contains("mupen64plus_next_libretro.dll"));
        assert_eq!(core_name, "Mupen64Plus-Next");
    }

    #[test]
    fn test_platform_core_conversion() {
        let mapper = CoreMapper::new();
        
        let (windows_core, _) = mapper
            .get_default_core("Nintendo - Nintendo 64", Platform::Windows)
            .unwrap();
        
        let (switch_core, _) = mapper
            .get_default_core("Nintendo - Nintendo 64", Platform::Switch)
            .unwrap();
        
        assert!(windows_core.contains(".dll"));
        assert!(switch_core.contains("_libnx.a"));
    }

    #[test]
    fn test_system_support() {
        let mapper = CoreMapper::new();
        let systems = mapper.get_supported_systems();
        
        assert!(systems.contains(&"Nintendo - Nintendo 64".to_string()));
        assert!(systems.contains(&"Nintendo - Super Nintendo Entertainment System".to_string()));
        assert!(systems.contains(&"Sony - PlayStation".to_string()));
    }

    #[test]
    fn test_mgba_multi_system_support() {
        let mapper = CoreMapper::new();
        
        // mGBA should support multiple Game Boy systems
        let (gba_core, gba_name) = mapper
            .get_default_core("Nintendo - Game Boy Advance", Platform::Windows)
            .unwrap();
        
        let (gb_core, gb_name) = mapper
            .get_default_core("Nintendo - Game Boy", Platform::Windows)
            .unwrap();
        
        assert_eq!(gba_name, "mGBA");
        assert_eq!(gb_name, "mGBA");
        assert_eq!(gba_core, gb_core);
    }
}
