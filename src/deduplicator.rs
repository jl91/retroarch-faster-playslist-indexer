use anyhow::{Result, Context};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use log::{info, debug};

use crate::scanner::RomFile;
use crate::crc32::calculate_crc32;

/// Duplicate detection strategy
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DuplicationStrategy {
    /// Keep the ROM with the best filename (usually USA > Europe > Japan)
    ByRegionPriority,
    /// Keep the ROM with the largest file size
    ByFileSize,
    /// Keep the ROM with the most recent modification date
    ByModificationDate,
    /// Keep the ROM from the preferred directory path
    ByDirectoryPriority,
    /// Custom strategy based on filename patterns
    ByFilenameQuality,
}

/// Region priority for filename-based deduplication
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Region {
    USA,
    Europe,
    Japan,
    World,
    Asia,
    Korea,
    Brazil,
    Australia,
    Germany,
    France,
    Spain,
    Italy,
    Unknown,
}

impl Region {
    /// Get priority score (lower = higher priority)
    pub fn priority_score(&self) -> u32 {
        match self {
            Region::USA => 1,
            Region::World => 2,
            Region::Europe => 3,
            Region::Japan => 4,
            Region::Asia => 5,
            Region::Korea => 6,
            Region::Australia => 7,
            Region::Brazil => 8,
            Region::Germany => 9,
            Region::France => 10,
            Region::Spain => 11,
            Region::Italy => 12,
            Region::Unknown => 100,
        }
    }

    /// Parse region from filename
    pub fn from_filename(filename: &str) -> Self {
        let filename_lower = filename.to_lowercase();
        
        if filename_lower.contains("(usa)") || filename_lower.contains("(u)") {
            Region::USA
        } else if filename_lower.contains("(world)") || filename_lower.contains("(w)") {
            Region::World
        } else if filename_lower.contains("(europe)") || filename_lower.contains("(e)") {
            Region::Europe
        } else if filename_lower.contains("(japan)") || filename_lower.contains("(j)") {
            Region::Japan
        } else if filename_lower.contains("(asia)") {
            Region::Asia
        } else if filename_lower.contains("(korea)") {
            Region::Korea
        } else if filename_lower.contains("(brazil)") {
            Region::Brazil
        } else if filename_lower.contains("(australia)") {
            Region::Australia
        } else if filename_lower.contains("(germany)") {
            Region::Germany
        } else if filename_lower.contains("(france)") {
            Region::France
        } else if filename_lower.contains("(spain)") {
            Region::Spain
        } else if filename_lower.contains("(italy)") {
            Region::Italy
        } else {
            Region::Unknown
        }
    }
}

/// ROM quality indicators
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RomQuality {
    Excellent,  // Original, no modifications
    Good,       // Minor fixes or improvements
    Fair,       // Some modifications but playable
    Poor,       // Bad dumps, heavy modifications
}

impl RomQuality {
    /// Assess ROM quality from filename
    pub fn assess_from_filename(filename: &str) -> Self {
        let filename_lower = filename.to_lowercase();
        
        // Bad quality indicators
        if filename_lower.contains("[b]") || 
           filename_lower.contains("[bad]") ||
           filename_lower.contains("(bad)") ||
           filename_lower.contains("[corrupt]") {
            return RomQuality::Poor;
        }

        // Fair quality indicators (hacks, translations)
        if filename_lower.contains("[h") ||
           filename_lower.contains("[t") ||
           filename_lower.contains("(hack)") ||
           filename_lower.contains("(translation)") {
            return RomQuality::Fair;
        }

        // Good quality indicators (fixes)
        if filename_lower.contains("[f") ||
           filename_lower.contains("(fixed)") {
            return RomQuality::Good;
        }

        // Default to excellent if no quality indicators found
        RomQuality::Excellent
    }

    /// Get quality score (higher = better quality)
    pub fn score(&self) -> u32 {
        match self {
            RomQuality::Excellent => 4,
            RomQuality::Good => 3,
            RomQuality::Fair => 2,
            RomQuality::Poor => 1,
        }
    }
}

/// Duplicate group containing ROMs with the same content
#[derive(Debug)]
pub struct DuplicateGroup {
    pub crc32: u32,
    pub roms: Vec<RomFile>,
    pub best_rom: Option<usize>, // Index of the best ROM in the group
}

impl DuplicateGroup {
    /// Create a new duplicate group
    pub fn new(crc32: u32) -> Self {
        Self {
            crc32,
            roms: Vec::new(),
            best_rom: None,
        }
    }

    /// Add ROM to the group
    pub fn add_rom(&mut self, rom: RomFile) {
        self.roms.push(rom);
    }

    /// Select the best ROM using the given strategy
    pub fn select_best(&mut self, strategy: DuplicationStrategy, directory_priorities: &[PathBuf]) {
        if self.roms.is_empty() {
            return;
        }

        let best_index = match strategy {
            DuplicationStrategy::ByRegionPriority => self.select_by_region_priority(),
            DuplicationStrategy::ByFileSize => self.select_by_file_size(),
            DuplicationStrategy::ByModificationDate => self.select_by_modification_date(),
            DuplicationStrategy::ByDirectoryPriority => self.select_by_directory_priority(directory_priorities),
            DuplicationStrategy::ByFilenameQuality => self.select_by_filename_quality(),
        };

        self.best_rom = Some(best_index);
    }

    /// Select best ROM by region priority
    fn select_by_region_priority(&self) -> usize {
        let mut best_index = 0;
        let mut best_score = u32::MAX;

        for (i, rom) in self.roms.iter().enumerate() {
            let filename = rom.path.file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("");
            
            let region = Region::from_filename(filename);
            let quality = RomQuality::assess_from_filename(filename);
            
            // Combined score: region priority + quality bonus
            let score = region.priority_score() - quality.score();
            
            if score < best_score {
                best_score = score;
                best_index = i;
            }
        }

        best_index
    }

    /// Select best ROM by file size (larger is usually better)
    fn select_by_file_size(&self) -> usize {
        let mut best_index = 0;
        let mut best_size = 0;

        for (i, rom) in self.roms.iter().enumerate() {
            if rom.size > best_size {
                best_size = rom.size;
                best_index = i;
            }
        }

        best_index
    }

    /// Select best ROM by modification date (newer is better)
    fn select_by_modification_date(&self) -> usize {
        let mut best_index = 0;
        let mut best_modified = None;

        for (i, rom) in self.roms.iter().enumerate() {
            if let Ok(metadata) = fs::metadata(&rom.path) {
                if let Ok(modified) = metadata.modified() {
                    if best_modified.map_or(true, |best| modified > best) {
                        best_modified = Some(modified);
                        best_index = i;
                    }
                }
            }
        }

        best_index
    }

    /// Select best ROM by directory priority
    fn select_by_directory_priority(&self, directory_priorities: &[PathBuf]) -> usize {
        let mut best_index = 0;
        let mut best_priority = usize::MAX;

        for (i, rom) in self.roms.iter().enumerate() {
            for (priority, dir) in directory_priorities.iter().enumerate() {
                if rom.path.starts_with(dir) {
                    if priority < best_priority {
                        best_priority = priority;
                        best_index = i;
                    }
                    break;
                }
            }
        }

        best_index
    }

    /// Select best ROM by filename quality
    fn select_by_filename_quality(&self) -> usize {
        let mut best_index = 0;
        let mut best_score = 0;

        for (i, rom) in self.roms.iter().enumerate() {
            let filename = rom.path.file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("");
            
            let region = Region::from_filename(filename);
            let quality = RomQuality::assess_from_filename(filename);
            
            // Combined score favoring quality and region
            let score = quality.score() * 10 + (100 - region.priority_score());
            
            if score > best_score {
                best_score = score;
                best_index = i;
            }
        }

        best_index
    }

    /// Get the best ROM
    pub fn get_best_rom(&self) -> Option<&RomFile> {
        self.best_rom.and_then(|index| self.roms.get(index))
    }

    /// Get ROMs to be removed (duplicates)
    pub fn get_duplicates(&self) -> Vec<&RomFile> {
        if let Some(best_index) = self.best_rom {
            self.roms.iter()
                .enumerate()
                .filter(|(i, _)| *i != best_index)
                .map(|(_, rom)| rom)
                .collect()
        } else {
            Vec::new()
        }
    }
}

/// Deduplication configuration
#[derive(Debug, Clone)]
pub struct DeduplicationConfig {
    pub strategy: DuplicationStrategy,
    pub directory_priorities: Vec<PathBuf>,
    pub dry_run: bool,
    pub create_backup: bool,
    pub backup_directory: Option<PathBuf>,
}

impl Default for DeduplicationConfig {
    fn default() -> Self {
        Self {
            strategy: DuplicationStrategy::ByFilenameQuality,
            directory_priorities: Vec::new(),
            dry_run: false,
            create_backup: false,
            backup_directory: None,
        }
    }
}

/// Deduplication results
#[derive(Debug, Default)]
pub struct DeduplicationResult {
    pub total_roms: usize,
    pub unique_roms: usize,
    pub duplicate_groups: usize,
    pub duplicates_removed: usize,
    pub space_saved: u64,
    pub removed_files: Vec<PathBuf>,
    pub kept_files: Vec<PathBuf>,
}

impl DeduplicationResult {
    /// Calculate deduplication percentage
    pub fn deduplication_percentage(&self) -> f64 {
        if self.total_roms == 0 {
            return 0.0;
        }
        
        (self.duplicates_removed as f64 / self.total_roms as f64) * 100.0
    }
}

/// Deduplication strategy enum
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Strategy {
    RegionPriority,
    FileSize,
    ModificationDate,
    DirectoryPriority,
    FilenameQuality,
}

/// Deduplication report
#[derive(Debug, Default)]
pub struct DeduplicationReport {
    pub duplicate_groups: usize,
    pub duplicates_found: usize,
    pub files_removed: usize,
    pub space_freed: u64,
    pub backup_location: Option<String>,
    pub removed_files: Vec<PathBuf>,
    pub kept_files: Vec<PathBuf>,
}

/// ROM deduplicator
#[derive(Debug)]
pub struct RomDeduplicator {
    strategy: Strategy,
    directory_priorities: Vec<PathBuf>,
    dry_run: bool,
    backup: bool,
    backup_directory: Option<PathBuf>,
}

impl RomDeduplicator {
    /// Create a new deduplicator
    pub fn new() -> Self {
        Self {
            strategy: Strategy::FilenameQuality,
            directory_priorities: Vec::new(),
            dry_run: false,
            backup: false,
            backup_directory: None,
        }
    }

    /// Set deduplication strategy
    pub fn with_strategy(mut self, strategy: Strategy) -> Self {
        self.strategy = strategy;
        self
    }

    /// Set directory priorities
    pub fn with_priority_directories(mut self, directories: Vec<PathBuf>) -> Self {
        self.directory_priorities = directories;
        self
    }

    /// Enable dry run mode
    pub fn with_dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = dry_run;
        self
    }

    /// Enable backup mode
    pub fn with_backup(mut self, backup: bool) -> Self {
        self.backup = backup;
        self
    }

    /// Set backup directory
    pub fn with_backup_directory(mut self, backup_dir: PathBuf) -> Self {
        self.backup_directory = Some(backup_dir);
        self
    }

    /// Perform deduplication on ROM collection
    pub fn deduplicate(&self, roms: &[RomFile]) -> Result<DeduplicationReport> {
        info!("Starting deduplication of {} ROMs", roms.len());
        
        // Group ROMs by CRC32
        let mut crc_groups: HashMap<u32, Vec<RomFile>> = HashMap::new();
        
        for rom in roms {
            let crc32 = match rom.crc32 {
                Some(crc) => crc,
                None => {
                    // Calculate CRC32 if not available
                    calculate_crc32(&rom.path)?
                }
            };
            
            crc_groups.entry(crc32).or_default().push(rom.clone());
        }

        // Find duplicate groups
        let duplicate_groups: Vec<_> = crc_groups
            .into_iter()
            .filter(|(_, roms)| roms.len() > 1)
            .collect();

        info!("Found {} duplicate groups", duplicate_groups.len());

        let mut report = DeduplicationReport {
            duplicate_groups: duplicate_groups.len(),
            ..Default::default()
        };

        // Create backup directory if needed
        if self.backup && !self.dry_run {
            let backup_dir = self.backup_directory
                .clone()
                .unwrap_or_else(|| PathBuf::from("./backup"));
            
            std::fs::create_dir_all(&backup_dir)?;
            report.backup_location = Some(backup_dir.display().to_string());
        }

        // Process each duplicate group
        for (crc32, group_roms) in duplicate_groups {
            debug!("Processing duplicate group with CRC32: {:08X}", crc32);
            
            if group_roms.is_empty() {
                continue;
            }

            report.duplicates_found += group_roms.len();

            // Select the best ROM to keep
            let best_index = self.select_best_rom(&group_roms)?;
            let best_rom = &group_roms[best_index];
            
            debug!("Selected best ROM: {}", best_rom.path.display());
            report.kept_files.push(best_rom.path.clone());

            // Remove duplicates (all except the best one)
            for (i, rom) in group_roms.iter().enumerate() {
                if i != best_index {
                    let file_size = if rom.size > 0 {
                        rom.size
                    } else {
                        std::fs::metadata(&rom.path).map(|m| m.len()).unwrap_or(0)
                    };
                    
                    if !self.dry_run {
                        // Backup file if requested
                        if self.backup {
                            self.backup_file(&rom.path)?;
                        }
                        
                        // Remove the duplicate
                        std::fs::remove_file(&rom.path)
                            .with_context(|| format!("Failed to remove duplicate: {}", rom.path.display()))?;
                    }
                    
                    report.files_removed += 1;
                    report.space_freed += file_size;
                    report.removed_files.push(rom.path.clone());
                    
                    info!("Removed duplicate: {}", rom.path.display());
                }
            }
        }

        info!("Deduplication complete: {} files removed, {} bytes freed", 
              report.files_removed, report.space_freed);

        Ok(report)
    }

    /// Select the best ROM from a group using the configured strategy
    fn select_best_rom(&self, roms: &[RomFile]) -> Result<usize> {
        match self.strategy {
            Strategy::RegionPriority => Ok(self.select_by_region_priority(roms)),
            Strategy::FileSize => Ok(self.select_by_file_size(roms)),
            Strategy::ModificationDate => self.select_by_modification_date(roms),
            Strategy::DirectoryPriority => Ok(self.select_by_directory_priority(roms)),
            Strategy::FilenameQuality => Ok(self.select_by_filename_quality(roms)),
        }
    }

    /// Select ROM by region priority
    fn select_by_region_priority(&self, roms: &[RomFile]) -> usize {
        let mut best_index = 0;
        let mut best_score = u32::MAX;

        for (i, rom) in roms.iter().enumerate() {
            let filename = rom.path.file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("");
            
            let region = Region::from_filename(filename);
            let quality = RomQuality::assess_from_filename(filename);
            
            // Combined score: region priority + quality
            let score = region.priority_score() * 10 + (5 - quality.score());
            
            if score < best_score {
                best_score = score;
                best_index = i;
            }
        }

        best_index
    }

    /// Select ROM by file size (largest)
    fn select_by_file_size(&self, roms: &[RomFile]) -> usize {
        let mut best_index = 0;
        let mut best_size = 0u64;

        for (i, rom) in roms.iter().enumerate() {
            let size = if rom.size > 0 {
                rom.size
            } else {
                std::fs::metadata(&rom.path).map(|m| m.len()).unwrap_or(0)
            };
            
            if size > best_size {
                best_size = size;
                best_index = i;
            }
        }

        best_index
    }

    /// Select ROM by modification date (most recent)
    fn select_by_modification_date(&self, roms: &[RomFile]) -> Result<usize> {
        let mut best_index = 0;
        let mut best_time = std::time::SystemTime::UNIX_EPOCH;

        for (i, rom) in roms.iter().enumerate() {
            let metadata = std::fs::metadata(&rom.path)?;
            let modified = metadata.modified()?;
            
            if modified > best_time {
                best_time = modified;
                best_index = i;
            }
        }

        Ok(best_index)
    }

    /// Select ROM by directory priority
    fn select_by_directory_priority(&self, roms: &[RomFile]) -> usize {
        for (_priority, dir) in self.directory_priorities.iter().enumerate() {
            for (i, rom) in roms.iter().enumerate() {
                if rom.path.starts_with(dir) {
                    return i;
                }
            }
        }
        
        // If no priority directory matches, fall back to filename quality
        self.select_by_filename_quality(roms)
    }

    /// Select ROM by filename quality
    fn select_by_filename_quality(&self, roms: &[RomFile]) -> usize {
        let mut best_index = 0;
        let mut best_score = 0u32;

        for (i, rom) in roms.iter().enumerate() {
            let filename = rom.path.file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("");
            
            let region = Region::from_filename(filename);
            let quality = RomQuality::assess_from_filename(filename);
            
            // Combined score: quality first, then region
            let score = quality.score() * 100 + (200 - region.priority_score());
            
            if score > best_score {
                best_score = score;
                best_index = i;
            }
        }

        best_index
    }

    /// Backup a file before deletion
    fn backup_file(&self, file_path: &Path) -> Result<()> {
        if let Some(backup_dir) = &self.backup_directory {
            let backup_path = backup_dir.join(file_path.file_name().unwrap());
            std::fs::copy(file_path, backup_path)?;
        }
        Ok(())
    }

    /// Generate detailed deduplication report
    pub fn generate_report(&self, report: &DeduplicationReport, output_path: &Path) -> Result<()> {
        let mut content = String::new();
        content.push_str("# ROM Deduplication Report\n\n");
        
        content.push_str(&format!("**Strategy**: {:?}\n", self.strategy));
        content.push_str(&format!("**Dry Run**: {}\n", self.dry_run));
        content.push_str(&format!("**Backup Enabled**: {}\n\n", self.backup));
        
        content.push_str("## Summary\n\n");
        content.push_str(&format!("- Duplicate Groups: {}\n", report.duplicate_groups));
        content.push_str(&format!("- Total Duplicates Found: {}\n", report.duplicates_found));
        content.push_str(&format!("- Files Removed: {}\n", report.files_removed));
        content.push_str(&format!("- Space Freed: {} bytes\n\n", report.space_freed));

        if !report.removed_files.is_empty() {
            content.push_str("## Removed Files\n\n");
            for file in &report.removed_files {
                content.push_str(&format!("- `{}`\n", file.display()));
            }
            content.push('\n');
        }

        if !report.kept_files.is_empty() {
            content.push_str("## Kept Files\n\n");
            for file in &report.kept_files {
                content.push_str(&format!("- `{}`\n", file.display()));
            }
        }

        std::fs::write(output_path, content)
            .with_context(|| format!("Failed to write report: {}", output_path.display()))?;

        Ok(())
    }
}

impl Default for RomDeduplicator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_region_detection() {
        assert_eq!(Region::from_filename("Super Mario Bros. (USA).nes"), Region::USA);
        assert_eq!(Region::from_filename("Final Fantasy (Japan).sfc"), Region::Japan);
        assert_eq!(Region::from_filename("Sonic (Europe).md"), Region::Europe);
        assert_eq!(Region::from_filename("Test Game.nes"), Region::Unknown);
    }

    #[test]
    fn test_quality_assessment() {
        assert_eq!(RomQuality::assess_from_filename("Super Mario Bros.nes"), RomQuality::Excellent);
        assert_eq!(RomQuality::assess_from_filename("Game [b].nes"), RomQuality::Poor);
        assert_eq!(RomQuality::assess_from_filename("Game [h1].nes"), RomQuality::Fair);
        assert_eq!(RomQuality::assess_from_filename("Game [f1].nes"), RomQuality::Good);
    }

    #[test]
    fn test_duplicate_group() {
        let group = DuplicateGroup::new(0x12345678);
        assert_eq!(group.roms.len(), 0);
        assert!(group.best_rom.is_none());
    }

    #[test]
    fn test_deduplication_config() {
        let config = DeduplicationConfig::default();
        assert_eq!(config.strategy, DuplicationStrategy::ByFilenameQuality);
        assert!(config.dry_run == false);
    }
}
