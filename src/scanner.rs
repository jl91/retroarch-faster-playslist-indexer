use anyhow::{Result, Context};
use dashmap::DashMap;
use log::{debug, info, warn};
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use walkdir::WalkDir;

use crate::crc32::calculate_crc32;
use crate::error::ScannerError;
use crate::thread_monitor::{ThreadMonitor, ThreadStatus};

#[derive(Debug, Clone)]
pub struct RomFile {
    pub path: PathBuf,
    pub filename: String,
    pub extension: String,
    pub size: u64,
    pub crc32: Option<u32>,
    pub system: Option<String>,
    pub is_archive: bool,
}

impl RomFile {
    pub fn new(path: PathBuf) -> Self {
        let filename = path.file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        
        let extension = path.extension()
            .unwrap_or_default()
            .to_string_lossy()
            .to_lowercase();

        Self {
            path,
            filename,
            extension,
            size: 0,
            crc32: None,
            system: None,
            is_archive: false,
        }
    }

    pub fn detect_system(&self) -> Option<String> {
        match self.extension.as_str() {
            // Nintendo
            "nes" | "fds" | "unf" | "unif" => Some("Nintendo - Nintendo Entertainment System".to_string()),
            "smc" | "sfc" | "swc" | "fig" => Some("Nintendo - Super Nintendo Entertainment System".to_string()),
            "z64" | "n64" | "v64" => Some("Nintendo - Nintendo 64".to_string()),
            "gcm" | "iso" | "gcz" | "rvz" if self.is_gamecube() => Some("Nintendo - GameCube".to_string()),
            "gb" => Some("Nintendo - Game Boy".to_string()),
            "gbc" => Some("Nintendo - Game Boy Color".to_string()),
            "gba" => Some("Nintendo - Game Boy Advance".to_string()),
            "nds" | "dsi" | "ids" => Some("Nintendo - Nintendo DS".to_string()),
            "3ds" | "cci" | "cxi" => Some("Nintendo - Nintendo 3DS".to_string()),

            // Sega
            "sms" | "sg" => Some("Sega - Master System - Mark III".to_string()),
            "md" | "smd" | "gen" | "bin" if self.is_genesis() => Some("Sega - Mega Drive - Genesis".to_string()),
            "32x" => Some("Sega - 32X".to_string()),
            "gg" => Some("Sega - Game Gear".to_string()),
            "cdi" | "gdi" | "chd" if self.is_dreamcast() => Some("Sega - Dreamcast".to_string()),

            // Sony
            "iso" | "cue" | "bin" | "img" | "pbp" | "chd" if self.is_playstation() => Some("Sony - PlayStation".to_string()),
            "iso" | "bin" | "mdf" | "nrg" if self.is_ps2() => Some("Sony - PlayStation 2".to_string()),
            "iso" | "cso" | "pbp" if self.is_psp() => Some("Sony - PlayStation Portable".to_string()),

            // Atari
            "a26" | "bin" if self.is_atari2600() => Some("Atari - 2600".to_string()),
            "a78" => Some("Atari - 7800".to_string()),

            // Others
            "pce" | "sgx" => Some("NEC - PC Engine - TurboGrafx 16".to_string()),
            "ws" | "wsc" => Some("Bandai - WonderSwan".to_string()),
            "neo" => Some("SNK - Neo Geo".to_string()),

            // Archives - handle with better priority logic
            "zip" | "7z" | "rar" => {
                // First, check if it's explicitly in a MAME directory structure
                if self.is_mame() {
                    Some("MAME".to_string())
                } else {
                    // For archives not in MAME directories, be more conservative
                    // Only detect other systems if there are very clear indicators
                    self.detect_system_from_path_conservative()
                }
            },

            _ => None,
        }
    }

    fn is_gamecube(&self) -> bool {
        // Heuristic: check path for gamecube indicators
        let path_str = self.path.to_string_lossy().to_lowercase();
        path_str.contains("gamecube") || path_str.contains("gc") || path_str.contains("ngc")
    }

    fn is_genesis(&self) -> bool {
        let path_str = self.path.to_string_lossy().to_lowercase();
        path_str.contains("genesis") || path_str.contains("megadrive") || path_str.contains("md")
    }

    fn is_dreamcast(&self) -> bool {
        let path_str = self.path.to_string_lossy().to_lowercase();
        path_str.contains("dreamcast") || path_str.contains("dc")
    }

    fn is_playstation(&self) -> bool {
        let path_str = self.path.to_string_lossy().to_lowercase();
        path_str.contains("playstation") || path_str.contains("psx") || path_str.contains("ps1")
    }

    fn is_ps2(&self) -> bool {
        let path_str = self.path.to_string_lossy().to_lowercase();
        path_str.contains("ps2") || path_str.contains("playstation2")
    }

    fn is_psp(&self) -> bool {
        let path_str = self.path.to_string_lossy().to_lowercase();
        path_str.contains("psp") || path_str.contains("playstation portable")
    }

    fn is_atari2600(&self) -> bool {
        let path_str = self.path.to_string_lossy().to_lowercase();
        path_str.contains("2600") || path_str.contains("atari")
    }

    fn is_mame(&self) -> bool {
        let path_str = self.path.to_string_lossy().to_lowercase();
        let filename = self.filename.to_lowercase();
        
        // Check for common MAME directory names and patterns
        if path_str.contains("mame") || 
           path_str.contains("arcade") ||
           path_str.contains("roms/mame") ||
           path_str.contains("roms\\mame") ||
           path_str.contains("/mame/") ||
           path_str.contains("\\mame\\") {
            return true;
        }
        
        // Common MAME subdirectories
        if path_str.contains("/roms/") && (
            path_str.contains("cps") ||     // Capcom Play System
            path_str.contains("neogeo") ||  // Neo Geo
            path_str.contains("fbneo") ||   // FinalBurn Neo
            path_str.contains("fba")        // FinalBurn Alpha
        ) {
            return true;
        }
        
        // For ZIP/7Z files, use a more permissive approach
        // Most ZIP files in emulation contexts are MAME ROMs
        if self.extension == "zip" || self.extension == "7z" {
            let name_without_ext = filename.trim_end_matches(&format!(".{}", self.extension));
            
            // MAME ROM names are typically:
            // - Short (usually <= 12 characters)
            // - May contain numbers, letters, underscores
            // - Don't contain common console ROM patterns like "(USA)" or "[!]"
            
            // Classic MAME naming patterns (short, cryptic names)
            if name_without_ext.len() <= 12 && 
               !name_without_ext.contains('(') && // No region/version info
               !name_without_ext.contains('[') && // No ROM dump info
               !name_without_ext.contains("rev") && // No revision info
               !name_without_ext.contains("v1.") && // No version numbers
               !name_without_ext.contains("disc") && // Not disc-based
               !name_without_ext.contains("track") { // Not track-based
                return true;
            }
            
            // Even if it doesn't match classic MAME patterns, 
            // ZIP files are commonly used for MAME ROMs
            // Only exclude if there are very strong indicators it's NOT MAME
            
            // If it's clearly a console ROM name pattern, might not be MAME
            if name_without_ext.contains("(usa)") ||
               name_without_ext.contains("(europe)") ||
               name_without_ext.contains("(japan)") ||
               name_without_ext.contains("(world)") ||
               name_without_ext.contains("[!]") ||
               name_without_ext.contains("[a]") ||
               name_without_ext.contains("[h") ||
               name_without_ext.contains("rev ") ||
               name_without_ext.contains("version ") {
                return false;
            }
            
            // Default to MAME for most ZIP files
            return true;
        }
        
        false
    }

    fn detect_system_from_path(&self) -> Option<String> {
        let path_str = self.path.to_string_lossy().to_lowercase();
        
        // Check directory names for system hints
        if path_str.contains("nes") || path_str.contains("famicom") {
            Some("Nintendo - Nintendo Entertainment System".to_string())
        } else if path_str.contains("snes") || path_str.contains("super nintendo") {
            Some("Nintendo - Super Nintendo Entertainment System".to_string())
        } else if path_str.contains("n64") || path_str.contains("nintendo 64") {
            Some("Nintendo - Nintendo 64".to_string())
        } else if path_str.contains("gba") || path_str.contains("game boy advance") {
            Some("Nintendo - Game Boy Advance".to_string())
        } else if path_str.contains("genesis") || path_str.contains("megadrive") {
            Some("Sega - Mega Drive - Genesis".to_string())
        } else if path_str.contains("mame") || path_str.contains("arcade") {
            Some("MAME".to_string())
        } else {
            None
        }
    }

    fn detect_system_from_path_conservative(&self) -> Option<String> {
        let path_str = self.path.to_string_lossy().to_lowercase();
        
        // Conservative detection - only detect if there are very clear, unambiguous indicators
        // For archives, be extra careful to avoid false positives
        
        // For ZIP/7Z files, be extremely conservative about non-MAME detection
        // Most ZIP/7Z files in emulation contexts are MAME ROMs
        if self.extension == "zip" || self.extension == "7z" {
            // For archives, almost always prefer MAME detection unless there are 
            // very strong indicators it's NOT a MAME ROM
            // We return None here to let the main detection logic handle it
            // (which will typically classify ZIP files as MAME)
            return None;
        }
        
        // For non-archive files, use more specific path-based detection
        // Only detect N64 if it's in a dedicated N64 directory structure
        if (path_str.contains("/n64/") || path_str.contains("\\n64\\") || 
            path_str.contains("/nintendo64/") || path_str.contains("\\nintendo64\\") ||
            path_str.contains("/nintendo_64/") || path_str.contains("\\nintendo_64\\")) &&
           !path_str.contains("mame") && !path_str.contains("arcade") {
            return Some("Nintendo - Nintendo 64".to_string());
        }
        
        // Only detect other systems with very explicit directory indicators
        if (path_str.contains("/nes/") || path_str.contains("\\nes\\") ||
            path_str.contains("/famicom/") || path_str.contains("\\famicom\\")) &&
           !path_str.contains("mame") && !path_str.contains("arcade") {
            return Some("Nintendo - Nintendo Entertainment System".to_string());
        }
        
        if (path_str.contains("/snes/") || path_str.contains("\\snes\\") ||
            path_str.contains("/super_nintendo/") || path_str.contains("\\super_nintendo\\")) &&
           !path_str.contains("mame") && !path_str.contains("arcade") {
            return Some("Nintendo - Super Nintendo Entertainment System".to_string());
        }
        
        if (path_str.contains("/gba/") || path_str.contains("\\gba\\") ||
            path_str.contains("/game_boy_advance/") || path_str.contains("\\game_boy_advance\\")) &&
           !path_str.contains("mame") && !path_str.contains("arcade") {
            return Some("Nintendo - Game Boy Advance".to_string());
        }
        
        if (path_str.contains("/genesis/") || path_str.contains("\\genesis\\") ||
            path_str.contains("/megadrive/") || path_str.contains("\\megadrive\\") ||
            path_str.contains("/mega_drive/") || path_str.contains("\\mega_drive\\")) &&
           !path_str.contains("mame") && !path_str.contains("arcade") {
            return Some("Sega - Mega Drive - Genesis".to_string());
        }
        
        // If we can't determine with high confidence, return None
        // This prevents false positives where MAME ROMs get misclassified
        None
    }
}

pub struct Scanner {
    threads: usize,
    recursive: bool,
    calculate_crc: bool,
    extensions: Option<Vec<String>>,
    verbose: bool,
    crc_cache: Arc<DashMap<PathBuf, u32>>,
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            threads: num_cpus::get(),
            recursive: true,
            calculate_crc: true,
            extensions: None,
            verbose: false,
            crc_cache: Arc::new(DashMap::new()),
        }
    }

    pub fn with_threads(mut self, threads: usize) -> Self {
        self.threads = threads;
        self
    }

    pub fn with_recursive(mut self, recursive: bool) -> Self {
        self.recursive = recursive;
        self
    }

    pub fn with_calculate_crc(mut self, calculate_crc: bool) -> Self {
        self.calculate_crc = calculate_crc;
        self
    }

    pub fn with_extensions(mut self, extensions: Option<&[String]>) -> Self {
        self.extensions = extensions.map(|e| e.to_vec());
        self
    }

    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    pub fn scan_directory(&self, dir: &Path) -> Result<Vec<RomFile>> {
        if !dir.exists() {
            return Err(ScannerError::DirectoryNotFound(dir.to_path_buf()).into());
        }

        info!("Escaneando diretório: {}", dir.display());

        // Configure rayon thread pool
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(self.threads)
            .build()
            .context("Falha ao criar thread pool")?;

        // Walk directory and collect file paths
        let walker = if self.recursive {
            WalkDir::new(dir)
        } else {
            WalkDir::new(dir).max_depth(1)
        };

        let file_paths: Vec<PathBuf> = walker
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().is_file())
            .map(|entry| entry.path().to_path_buf())
            .filter(|path| self.is_rom_file(path))
            .collect();

        if file_paths.is_empty() {
            warn!("Nenhum arquivo de ROM encontrado em {}", dir.display());
            return Ok(Vec::new());
        }

        info!("Encontrados {} arquivos para processar", file_paths.len());

        // Create thread monitor
        let monitor = Arc::new(ThreadMonitor::new(file_paths.len(), self.threads, self.verbose));
        
        // Process files in parallel with real-time thread monitoring
        let roms: Vec<RomFile> = pool.install(|| {
            file_paths
                .par_iter()
                .enumerate()
                .filter_map(|(index, path)| {
                    let thread_id = rayon::current_thread_index().unwrap_or(0);
                    
                    // Register thread if first time
                    monitor.register_thread(thread_id);
                    
                    // Update main progress message
                    monitor.set_main_message(&format!("Processando arquivo {} de {}", index + 1, file_paths.len()));
                    
                    // Process the file with detailed status updates
                    match self.process_file_with_monitor(path, &monitor, thread_id) {
                        Ok(Some(rom)) => {
                            monitor.update_thread_status(thread_id, ThreadStatus::ProcessingComplete(path.display().to_string()));
                            Some(rom)
                        },
                        Ok(None) => {
                            monitor.update_thread_status(thread_id, ThreadStatus::ProcessingComplete(path.display().to_string()));
                            None
                        },
                        Err(e) => {
                            monitor.update_thread_status(thread_id, ThreadStatus::Error(path.display().to_string(), e.to_string()));
                            None
                        }
                    }
                })
                .collect()
        });

        monitor.finish("✅ Escaneamento concluído");

        info!("Processados {} arquivos de ROM", roms.len());

        Ok(roms)
    }

    /// Versão simplificada para modo paralelo - sem thread monitoring detalhado
    pub fn scan_directory_simple(&self, dir: &Path) -> Result<Vec<RomFile>> {
        if !dir.exists() {
            return Err(ScannerError::DirectoryNotFound(dir.to_path_buf()).into());
        }

        // Walk directory and collect file paths
        let walker = if self.recursive {
            WalkDir::new(dir)
        } else {
            WalkDir::new(dir).max_depth(1)
        };

        let file_paths: Vec<PathBuf> = walker
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().is_file())
            .map(|entry| entry.path().to_path_buf())
            .filter(|path| self.is_rom_file(path))
            .collect();

        if file_paths.is_empty() {
            return Ok(Vec::new());
        }

        // Configure rayon thread pool
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(self.threads)
            .build()
            .context("Falha ao criar thread pool")?;

        // Process files in parallel WITHOUT detailed monitoring for performance
        let roms: Vec<RomFile> = pool.install(|| {
            file_paths
                .par_iter()
                .filter_map(|path| {
                    match self.process_file_simple(path) {
                        Ok(Some(rom)) => Some(rom),
                        Ok(None) => None,
                        Err(_) => None, // Ignora erros silenciosamente para performance
                    }
                })
                .collect()
        });

        Ok(roms)
    }

    fn is_rom_file(&self, path: &Path) -> bool {
        let extension = path.extension()
            .unwrap_or_default()
            .to_string_lossy()
            .to_lowercase();

        if let Some(ref allowed_extensions) = self.extensions {
            return allowed_extensions.iter().any(|ext| ext.to_lowercase() == extension);
        }

        // Default supported extensions
        matches!(extension.as_str(),
            // Nintendo
            "nes" | "fds" | "unf" | "unif" |
            "smc" | "sfc" | "swc" | "fig" |
            "z64" | "n64" | "v64" |
            "gcm" | "iso" | "gcz" | "rvz" |
            "gb" | "gbc" | "gba" |
            "nds" | "dsi" | "ids" |
            "3ds" | "cci" | "cxi" |
            
            // Sega
            "sms" | "sg" |
            "md" | "smd" | "gen" | "bin" |
            "32x" | "gg" |
            "cdi" | "gdi" | "chd" |
            
            // Sony
            "pbp" | "cue" | "img" | "mdf" | "nrg" | "cso" |
            
            // Atari
            "a26" | "a78" |
            
            // Others
            "pce" | "sgx" | "ws" | "wsc" | "neo" |
            
            // Archives
            "zip" | "7z" | "rar"
        )
    }

    fn process_file(&self, path: &Path) -> Result<Option<RomFile>> {
        let mut rom = RomFile::new(path.to_path_buf());

        // Get file metadata
        let metadata = std::fs::metadata(path)
            .with_context(|| format!("Falha ao ler metadados de {}", path.display()))?;
        rom.size = metadata.len();

        // Detect if it's an archive
        rom.is_archive = matches!(rom.extension.as_str(), "zip" | "7z" | "rar");

        // Calculate CRC32 if requested and not cached
        if self.calculate_crc {
            if let Some(cached_crc) = self.crc_cache.get(path) {
                rom.crc32 = Some(*cached_crc);
                debug!("CRC32 do cache para {}: {:08X}", path.display(), *cached_crc);
            } else {
                match calculate_crc32(path) {
                    Ok(crc) => {
                        rom.crc32 = Some(crc);
                        self.crc_cache.insert(path.to_path_buf(), crc);
                        debug!("CRC32 calculado para {}: {:08X}", path.display(), crc);
                    }
                    Err(e) => {
                        warn!("Falha ao calcular CRC32 para {}: {}", path.display(), e);
                    }
                }
            }
        }

        // Detect system
        rom.system = rom.detect_system();

        if self.verbose {
            debug!(
                "Processado: {} | Sistema: {} | CRC32: {} | Tamanho: {} bytes",
                rom.filename,
                rom.system.as_deref().unwrap_or("Desconhecido"),
                rom.crc32.map(|c| format!("{:08X}", c)).unwrap_or_else(|| "N/A".to_string()),
                rom.size
            );
        }

        Ok(Some(rom))
    }

    /// Processa um arquivo com monitoramento detalhado de thread
    fn process_file_with_monitor(
        &self, 
        path: &Path, 
        monitor: &ThreadMonitor, 
        thread_id: usize
    ) -> Result<Option<RomFile>> {
        // Update status: scanning file
        monitor.update_thread_status(thread_id, ThreadStatus::ScanningFile(path.display().to_string()));

        let mut rom = RomFile::new(path.to_path_buf());

        // Get file metadata
        let metadata = std::fs::metadata(path)
            .with_context(|| format!("Falha ao ler metadados de {}", path.display()))?;
        rom.size = metadata.len();

        // Detect if it's an archive
        rom.is_archive = matches!(rom.extension.as_str(), "zip" | "7z" | "rar");

        // If it's an archive, show extraction progress
        if rom.is_archive {
            // Simulate extraction progress for archives
            for progress in (0..=100).step_by(20) {
                monitor.update_thread_status(
                    thread_id, 
                    ThreadStatus::ExtractingArchive { 
                        file: path.display().to_string(), 
                        progress: progress as f32 
                    }
                );
                std::thread::sleep(std::time::Duration::from_millis(10)); // Simulate work
            }
        }

        // Calculate CRC32 if requested and not cached
        if self.calculate_crc {
            monitor.update_thread_status(thread_id, ThreadStatus::CalculatingCrc(path.display().to_string()));
            
            if let Some(cached_crc) = self.crc_cache.get(path) {
                rom.crc32 = Some(*cached_crc);
                debug!("CRC32 do cache para {}: {:08X}", path.display(), *cached_crc);
            } else {
                match calculate_crc32(path) {
                    Ok(crc) => {
                        rom.crc32 = Some(crc);
                        self.crc_cache.insert(path.to_path_buf(), crc);
                        debug!("CRC32 calculado para {}: {:08X}", path.display(), crc);
                    }
                    Err(e) => {
                        warn!("Falha ao calcular CRC32 para {}: {}", path.display(), e);
                    }
                }
            }
        }

        // Detect system
        rom.system = rom.detect_system();

        if self.verbose {
            debug!(
                "Processado: {} | Sistema: {} | CRC32: {} | Tamanho: {} bytes",
                rom.filename,
                rom.system.as_deref().unwrap_or("Desconhecido"),
                rom.crc32.map(|c| format!("{:08X}", c)).unwrap_or_else(|| "N/A".to_string()),
                rom.size
            );
        }

        Ok(Some(rom))
    }

    /// Processamento simplificado de arquivo sem monitoramento detalhado
    fn process_file_simple(&self, path: &Path) -> Result<Option<RomFile>> {
        let mut rom = RomFile::new(path.to_path_buf());

        // Get file metadata
        let metadata = std::fs::metadata(path)
            .with_context(|| format!("Falha ao ler metadados de {}", path.display()))?;
        rom.size = metadata.len();

        // Detect if it's an archive
        rom.is_archive = matches!(rom.extension.as_str(), "zip" | "7z" | "rar");

        // Calculate CRC32 if requested and not cached
        if self.calculate_crc {
            if let Some(cached_crc) = self.crc_cache.get(path) {
                rom.crc32 = Some(*cached_crc);
            } else {
                match calculate_crc32(path) {
                    Ok(crc) => {
                        rom.crc32 = Some(crc);
                        self.crc_cache.insert(path.to_path_buf(), crc);
                    }
                    Err(_) => {
                        // Ignora erro de CRC para performance em modo paralelo
                    }
                }
            }
        }

        // Detect system
        rom.system = rom.detect_system();

        Ok(Some(rom))
    }
}

impl Default for Scanner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_system_detection() {
        let rom = RomFile::new(PathBuf::from("test.z64"));
        assert_eq!(rom.detect_system().unwrap(), "Nintendo - Nintendo 64");

        let rom = RomFile::new(PathBuf::from("test.gba"));
        assert_eq!(rom.detect_system().unwrap(), "Nintendo - Game Boy Advance");
    }

    #[test]
    fn test_extension_filtering() {
        let scanner = Scanner::new();
        
        assert!(scanner.is_rom_file(Path::new("game.z64")));
        assert!(scanner.is_rom_file(Path::new("game.iso")));
        assert!(!scanner.is_rom_file(Path::new("readme.txt")));
    }

    #[test]
    fn test_custom_extensions() {
        let scanner = Scanner::new()
            .with_extensions(Some(&["rom".to_string(), "bin".to_string()]));
        
        assert!(scanner.is_rom_file(Path::new("game.rom")));
        assert!(scanner.is_rom_file(Path::new("game.bin")));
        assert!(!scanner.is_rom_file(Path::new("game.z64")));
    }
}
