use anyhow::{Result, Context};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use log::{info, debug};

use crate::scanner::RomFile;
use crate::dat_parser::{DatCollection, DatEntry};
use crate::crc32::calculate_crc32;

/// Validation result for a single ROM
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationResult {
    /// ROM matches DAT entry perfectly
    Valid {
        dat_name: String,
        region: Option<String>,
        version: Option<String>,
    },
    /// ROM CRC32 matches but filename differs
    ValidButRenamed {
        dat_name: String,
        expected_name: String,
        region: Option<String>,
        version: Option<String>,
    },
    /// ROM not found in any DAT
    Unknown,
    /// ROM matches but is marked as bad dump
    BadDump {
        dat_name: String,
        reason: String,
    },
    /// ROM is a homebrew/unlicensed title
    Homebrew {
        detected_name: String,
    },
    /// ROM file is corrupted or unreadable
    Corrupted {
        error: String,
    },
}

/// Validation report for a collection of ROMs
#[derive(Debug, Default)]
pub struct ValidationReport {
    pub total_roms: usize,
    pub valid_roms: usize,
    pub renamed_roms: usize,
    pub unknown_roms: usize,
    pub bad_dumps: usize,
    pub homebrew_roms: usize,
    pub corrupted_roms: usize,
    pub validation_results: HashMap<PathBuf, ValidationResult>,
    pub missing_from_collection: Vec<DatEntry>,
}

impl ValidationReport {
    /// Calculate validation percentage
    pub fn validation_percentage(&self) -> f64 {
        if self.total_roms == 0 {
            return 100.0;
        }
        
        let validated = self.valid_roms + self.renamed_roms;
        (validated as f64 / self.total_roms as f64) * 100.0
    }

    /// Get unknown ROMs
    pub fn unknown_roms(&self) -> Vec<&PathBuf> {
        self.validation_results
            .iter()
            .filter_map(|(path, result)| {
                match result {
                    ValidationResult::Unknown => Some(path),
                    _ => None,
                }
            })
            .collect()
    }

    /// Get ROMs that need renaming
    pub fn roms_needing_rename(&self) -> Vec<(&PathBuf, &str)> {
        self.validation_results
            .iter()
            .filter_map(|(path, result)| {
                match result {
                    ValidationResult::ValidButRenamed { expected_name, .. } => {
                        Some((path, expected_name.as_str()))
                    },
                    _ => None,
                }
            })
            .collect()
    }

    /// Get bad dumps
    pub fn bad_dumps(&self) -> Vec<(&PathBuf, &str)> {
        self.validation_results
            .iter()
            .filter_map(|(path, result)| {
                match result {
                    ValidationResult::BadDump { reason, .. } => {
                        Some((path, reason.as_str()))
                    },
                    _ => None,
                }
            })
            .collect()
    }
}

/// ROM integrity validator
pub struct RomValidator {
    dat_collections: HashMap<String, DatCollection>,
    homebrew_patterns: Vec<String>,
}

impl RomValidator {
    /// Create a new ROM validator
    pub fn new() -> Self {
        Self {
            dat_collections: HashMap::new(),
            homebrew_patterns: Self::load_homebrew_patterns(),
        }
    }

    /// Load DAT collection for a system
    pub fn load_dat_collection<P: AsRef<Path>>(&mut self, system: &str, dat_path: P) -> Result<()> {
        let mut collection = DatCollection::new();
        collection.load_from_file(dat_path.as_ref())?;
        
        let total_entries = collection.total_entries();
        self.dat_collections.insert(system.to_string(), collection);
        info!("Loaded DAT collection for {}: {} entries", 
            system, total_entries);
        
        Ok(())
    }

    /// Load common homebrew/unlicensed patterns
    fn load_homebrew_patterns() -> Vec<String> {
        vec![
            "(Unl)".to_string(),
            "(Homebrew)".to_string(),
            "(PD)".to_string(), // Public Domain
            "[h1]".to_string(), // Hack
            "[h2]".to_string(),
            "[T+".to_string(), // Translation
            "[f1]".to_string(), // Fixed
            "[f2]".to_string(),
            "[o1]".to_string(), // Overdump
            "[b1]".to_string(), // Bad dump
            "[b2]".to_string(),
            "(Pirate)".to_string(),
            "(Aftermarket)".to_string(),
        ]
    }

    /// Validate a single ROM file
    pub fn validate_rom(&self, rom: &RomFile) -> Result<ValidationResult> {
        debug!("Validating ROM: {}", rom.path.display());
        
        // Calculate CRC32 if not already available
        let crc32 = match rom.crc32 {
            Some(crc) => crc,
            None => {
                match calculate_crc32(&rom.path) {
                    Ok(crc) => crc,
                    Err(e) => {
                        return Ok(ValidationResult::Corrupted {
                            error: e.to_string(),
                        });
                    }
                }
            }
        };

        // Check against DAT collections
        for (system, collection) in &self.dat_collections {
            if let Some(entry) = collection.find_by_crc32(crc32) {
                return Ok(self.analyze_dat_match(rom, entry, system));
            }
        }

        // Check if it's a known homebrew pattern
        let filename = rom.path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("");
        
        if self.is_homebrew_pattern(filename) {
            return Ok(ValidationResult::Homebrew {
                detected_name: filename.to_string(),
            });
        }

        Ok(ValidationResult::Unknown)
    }

    /// Analyze a DAT match
    fn analyze_dat_match(&self, rom: &RomFile, entry: &DatEntry, _system: &str) -> ValidationResult {
        let filename = rom.path.file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or("");

        // Check if it's marked as bad dump
        if self.is_bad_dump_entry(entry) {
            return ValidationResult::BadDump {
                dat_name: entry.name.clone(),
                reason: "Marked as bad dump in DAT".to_string(),
            };
        }

        // Extract region and version info
        let region = self.extract_region(&entry.name);
        let version = self.extract_version(&entry.name);

        // Check if filename matches expected name
        let expected_name = self.normalize_name(&entry.name);
        let actual_name = self.normalize_name(filename);

        if actual_name == expected_name {
            ValidationResult::Valid {
                dat_name: entry.name.clone(),
                region,
                version,
            }
        } else {
            ValidationResult::ValidButRenamed {
                dat_name: entry.name.clone(),
                expected_name: entry.name.clone(),
                region,
                version,
            }
        }
    }

    /// Check if DAT entry represents a bad dump
    fn is_bad_dump_entry(&self, entry: &DatEntry) -> bool {
        let name = entry.name.to_lowercase();
        
        name.contains("[b]") || 
        name.contains("[bad]") ||
        name.contains("(bad)") ||
        name.contains("[corrupt]") ||
        name.contains("(corrupt)")
    }

    /// Check if filename matches homebrew patterns
    fn is_homebrew_pattern(&self, filename: &str) -> bool {
        self.homebrew_patterns.iter()
            .any(|pattern| filename.contains(pattern))
    }

    /// Extract region information from ROM name
    fn extract_region(&self, name: &str) -> Option<String> {
        let regions = [
            ("(USA)", "USA"),
            ("(Europe)", "Europe"),
            ("(Japan)", "Japan"),
            ("(World)", "World"),
            ("(Asia)", "Asia"),
            ("(Korea)", "Korea"),
            ("(Brazil)", "Brazil"),
            ("(Australia)", "Australia"),
            ("(Germany)", "Germany"),
            ("(France)", "France"),
            ("(Spain)", "Spain"),
            ("(Italy)", "Italy"),
        ];

        for (pattern, region) in regions.iter() {
            if name.contains(pattern) {
                return Some(region.to_string());
            }
        }

        None
    }

    /// Extract version information from ROM name
    fn extract_version(&self, name: &str) -> Option<String> {
        // Look for version patterns like (Rev 1), (v1.1), etc.
        let version_patterns = [
            r"\(Rev \d+\)",
            r"\(v\d+\.\d+\)",
            r"\(Version \d+\)",
        ];

        for pattern in version_patterns.iter() {
            if let Ok(regex) = regex::Regex::new(pattern) {
                if let Some(captures) = regex.find(name) {
                    return Some(captures.as_str().to_string());
                }
            }
        }

        None
    }

    /// Normalize name for comparison
    fn normalize_name(&self, name: &str) -> String {
        name.to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Validate a collection of ROMs
    pub fn validate_collection(&self, roms: &[RomFile]) -> Result<ValidationReport> {
        info!("Validating collection of {} ROMs", roms.len());
        
        let mut report = ValidationReport {
            total_roms: roms.len(),
            ..Default::default()
        };

        for rom in roms {
            let result = self.validate_rom(rom)?;
            
            match &result {
                ValidationResult::Valid { .. } => report.valid_roms += 1,
                ValidationResult::ValidButRenamed { .. } => report.renamed_roms += 1,
                ValidationResult::Unknown => report.unknown_roms += 1,
                ValidationResult::BadDump { .. } => report.bad_dumps += 1,
                ValidationResult::Homebrew { .. } => report.homebrew_roms += 1,
                ValidationResult::Corrupted { .. } => report.corrupted_roms += 1,
            }

            report.validation_results.insert(rom.path.clone(), result);
        }

        // Find missing ROMs from DAT collections
        report.missing_from_collection = self.find_missing_roms(roms);

        info!("Validation complete: {:.1}% validated ({}/{})", 
            report.validation_percentage(), 
            report.valid_roms + report.renamed_roms,
            report.total_roms);

        Ok(report)
    }

    /// Find ROMs that are in DAT but missing from collection
    fn find_missing_roms(&self, roms: &[RomFile]) -> Vec<DatEntry> {
        let present_crc32s: std::collections::HashSet<u32> = roms
            .iter()
            .filter_map(|rom| rom.crc32.or_else(|| {
                // Try to calculate CRC32 for comparison
                calculate_crc32(&rom.path).ok()
            }))
            .collect();

        let mut missing = Vec::new();

        for collection in self.dat_collections.values() {
            for (_system, entries) in collection.entries.iter() {
                for entry in entries {
                    if !present_crc32s.contains(&entry.crc32) {
                        missing.push(entry.clone());
                    }
                }
            }
        }

        missing.sort_by(|a, b| a.name.cmp(&b.name));
        missing
    }

    /// Generate detailed validation report
    pub fn generate_report(&self, roms: &[RomFile], output_path: &Path) -> Result<()> {
        let report = self.validate_collection(roms)?;
        
        let mut content = String::new();
        content.push_str("# ROM Collection Validation Report\n\n");
        
        content.push_str(&format!("**Total ROMs**: {}\n", report.total_roms));
        content.push_str(&format!("**Validation Rate**: {:.1}%\n\n", report.validation_percentage()));
        
        content.push_str("## Summary\n\n");
        content.push_str(&format!("- ✅ Valid: {}\n", report.valid_roms));
        content.push_str(&format!("- 🔄 Need Rename: {}\n", report.renamed_roms));
        content.push_str(&format!("- ❓ Unknown: {}\n", report.unknown_roms));
        content.push_str(&format!("- 🏠 Homebrew: {}\n", report.homebrew_roms));
        content.push_str(&format!("- ❌ Bad Dumps: {}\n", report.bad_dumps));
        content.push_str(&format!("- 💥 Corrupted: {}\n\n", report.corrupted_roms));

        // Unknown ROMs section
        if !report.unknown_roms().is_empty() {
            content.push_str("## Unknown ROMs\n\n");
            for path in report.unknown_roms() {
                content.push_str(&format!("- `{}`\n", path.display()));
            }
            content.push('\n');
        }

        // ROMs needing rename
        if !report.roms_needing_rename().is_empty() {
            content.push_str("## ROMs Needing Rename\n\n");
            for (path, expected) in report.roms_needing_rename() {
                content.push_str(&format!("- `{}` → `{}`\n", path.display(), expected));
            }
            content.push('\n');
        }

        // Bad dumps
        if !report.bad_dumps().is_empty() {
            content.push_str("## Bad Dumps\n\n");
            for (path, reason) in report.bad_dumps() {
                content.push_str(&format!("- `{}`: {}\n", path.display(), reason));
            }
            content.push('\n');
        }

        // Missing ROMs
        if !report.missing_from_collection.is_empty() {
            content.push_str("## Missing from Collection\n\n");
            for entry in &report.missing_from_collection {
                content.push_str(&format!("- {} (CRC32: {:08X})\n", entry.name, entry.crc32));
            }
        }

        std::fs::write(output_path, content)
            .with_context(|| format!("Failed to write report: {}", output_path.display()))?;

        info!("Validation report saved to: {}", output_path.display());
        Ok(())
    }

    /// Get available systems
    pub fn available_systems(&self) -> Vec<String> {
        self.dat_collections.keys().cloned().collect()
    }
}

impl Default for RomValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_validator_creation() {
        let validator = RomValidator::new();
        assert!(validator.dat_collections.is_empty());
        assert!(!validator.homebrew_patterns.is_empty());
    }

    #[test]
    fn test_homebrew_detection() {
        let validator = RomValidator::new();
        
        assert!(validator.is_homebrew_pattern("Test Game (Homebrew).nes"));
        assert!(validator.is_homebrew_pattern("Hack [h1].sfc"));
        assert!(!validator.is_homebrew_pattern("Super Mario Bros.nes"));
    }

    #[test]
    fn test_region_extraction() {
        let validator = RomValidator::new();
        
        assert_eq!(validator.extract_region("Super Mario Bros. (USA)"), Some("USA".to_string()));
        assert_eq!(validator.extract_region("Final Fantasy (Japan)"), Some("Japan".to_string()));
        assert_eq!(validator.extract_region("Test Game"), None);
    }

    #[test]
    fn test_name_normalization() {
        let validator = RomValidator::new();
        
        assert_eq!(validator.normalize_name("Super Mario Bros."), "super mario bros");
        assert_eq!(validator.normalize_name("Test-Game_v1.0!"), "testgamev10");
    }
}
