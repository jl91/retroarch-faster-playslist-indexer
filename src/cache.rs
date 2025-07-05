use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// Cache entry for CRC32 calculations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    pub crc32: u32,
    pub file_size: u64,
    pub modified_time: u64,
    pub last_accessed: u64,
}

/// Persistent CRC32 cache to avoid recalculating hashes
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CrcCache {
    entries: HashMap<PathBuf, CacheEntry>,
    #[serde(skip)]
    cache_file: Option<PathBuf>,
    #[serde(skip)]
    dirty: bool,
}

impl CrcCache {
    /// Create a new cache instance
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            cache_file: None,
            dirty: false,
        }
    }

    /// Create a new cache with default location
    pub fn with_default_location() -> Result<Self> {
        let cache_dir = dirs::cache_dir()
            .or_else(|| dirs::home_dir().map(|h| h.join(".cache")))
            .unwrap_or_else(|| PathBuf::from("."));
        
        let cache_dir = cache_dir.join("retroarch-indexer");
        std::fs::create_dir_all(&cache_dir)?;
        
        let cache_file = cache_dir.join("crc32_cache.json");
        
        if cache_file.exists() {
            Self::load_from_file(&cache_file)
        } else {
            let mut cache = Self::default();
            cache.cache_file = Some(cache_file);
            Ok(cache)
        }
    }

    /// Load cache from file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        
        if !path.exists() {
            return Ok(Self::new());
        }

        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read cache file: {}", path.display()))?;
        
        let mut cache: Self = serde_json::from_str(&content)
            .with_context(|| "Failed to parse cache file")?;
        
        cache.cache_file = Some(path.to_path_buf());
        
        Ok(cache)
    }

    /// Save cache to file
    pub fn save_to_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path = path.as_ref();
        
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create cache directory: {}", parent.display()))?;
        }

        let content = serde_json::to_string_pretty(self)
            .with_context(|| "Failed to serialize cache")?;
        
        fs::write(path, content)
            .with_context(|| format!("Failed to write cache file: {}", path.display()))?;
        
        self.cache_file = Some(path.to_path_buf());
        self.dirty = false;
        
        Ok(())
    }

    /// Auto-save if cache file is set and cache is dirty
    pub fn auto_save(&mut self) -> Result<()> {
        if self.dirty && self.cache_file.is_some() {
            let cache_file = self.cache_file.clone().unwrap();
            self.save_to_file(cache_file)?;
        }
        Ok(())
    }

    /// Get CRC32 from cache if entry is valid
    pub fn get_crc32(&mut self, file_path: &Path) -> Result<Option<u32>> {
        let metadata = fs::metadata(file_path)
            .with_context(|| format!("Failed to get metadata for: {}", file_path.display()))?;
        
        let file_size = metadata.len();
        let modified_time = metadata.modified()
            .with_context(|| "Failed to get file modification time")?
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        if let Some(entry) = self.entries.get_mut(file_path) {
            // Check if entry is still valid
            if entry.file_size == file_size && entry.modified_time == modified_time {
                // Update last accessed time
                entry.last_accessed = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                self.dirty = true;
                
                return Ok(Some(entry.crc32));
            } else {
                // Entry is stale, remove it
                self.entries.remove(file_path);
                self.dirty = true;
            }
        }

        Ok(None)
    }

    /// Store CRC32 in cache
    pub fn set_crc32(&mut self, file_path: &Path, crc32: u32) -> Result<()> {
        let metadata = fs::metadata(file_path)
            .with_context(|| format!("Failed to get metadata for: {}", file_path.display()))?;
        
        let file_size = metadata.len();
        let modified_time = metadata.modified()
            .with_context(|| "Failed to get file modification time")?
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let entry = CacheEntry {
            crc32,
            file_size,
            modified_time,
            last_accessed: now,
        };

        self.entries.insert(file_path.to_path_buf(), entry);
        self.dirty = true;
        
        Ok(())
    }

    /// Clear all cache entries
    pub fn clear(&mut self) -> Result<()> {
        self.entries.clear();
        self.dirty = true;
        self.auto_save()?;
        Ok(())
    }

    /// Get detailed statistics for CLI reporting
    pub fn get_stats(&self) -> Result<DetailedCacheStats> {
        let cache_size = if let Some(cache_file) = &self.cache_file {
            if cache_file.exists() {
                std::fs::metadata(cache_file)?.len()
            } else {
                0
            }
        } else {
            0
        };

        let last_updated = if let Some(cache_file) = &self.cache_file {
            if cache_file.exists() {
                let metadata = std::fs::metadata(cache_file)?;
                let modified = metadata.modified()?;
                let duration = modified.duration_since(UNIX_EPOCH).unwrap_or_default();
                chrono::DateTime::from_timestamp(duration.as_secs() as i64, 0)
                    .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                    .unwrap_or_else(|| "Unknown".to_string())
            } else {
                "Never".to_string()
            }
        } else {
            "No cache file".to_string()
        };

        Ok(DetailedCacheStats {
            total_entries: self.entries.len(),
            cache_size,
            cache_hits: 0, // Would need to track this in real implementation
            cache_misses: 0, // Would need to track this in real implementation
            total_requests: 0, // Would need to track this in real implementation
            last_updated,
        })
    }

    /// Clean old entries and return count of removed entries
    pub fn clean_old_entries(&mut self, max_age_days: u64) -> Result<usize> {
        let max_age_secs = max_age_days * 24 * 60 * 60;
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let initial_count = self.entries.len();
        
        self.entries.retain(|_path, entry| {
            now.saturating_sub(entry.last_accessed) < max_age_secs
        });
        
        let removed_count = initial_count - self.entries.len();
        if removed_count > 0 {
            log::info!("Cleaned {} old cache entries", removed_count);
            self.dirty = true;
            self.auto_save()?;
        }
        
        Ok(removed_count)
    }

    /// Auto-save on drop if possible
    pub fn drop(&mut self) {
        let _ = self.auto_save();
    }
}

/// Enhanced cache statistics for CLI reporting
#[derive(Debug, Default)]
pub struct DetailedCacheStats {
    pub total_entries: usize,
    pub cache_size: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub total_requests: u64,
    pub last_updated: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::io::Write;

    #[test]
    fn test_cache_operations() {
        let temp_dir = tempdir().unwrap();
        let cache_file = temp_dir.path().join("test_cache.json");
        
        // Create a test file
        let test_file = temp_dir.path().join("test.rom");
        std::fs::write(&test_file, b"test data").unwrap();
        
        let mut cache = CrcCache::new();
        
        // Initially no CRC32
        assert!(cache.get_crc32(&test_file).unwrap().is_none());
        
        // Set CRC32
        cache.set_crc32(&test_file, 0x12345678).unwrap();
        
        // Should now return the CRC32
        assert_eq!(cache.get_crc32(&test_file).unwrap(), Some(0x12345678));
        
        // Save and reload
        cache.save_to_file(&cache_file).unwrap();
        let mut reloaded_cache = CrcCache::load_from_file(&cache_file).unwrap();
        
        // Should still have the CRC32
        assert_eq!(reloaded_cache.get_crc32(&test_file).unwrap(), Some(0x12345678));
    }
}
