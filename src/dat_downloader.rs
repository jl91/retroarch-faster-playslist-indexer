use anyhow::{Result, Context};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use log::{info, debug, warn, error};

/// DAT source configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatSource {
    pub name: String,
    pub url: String,
    pub system: String,
    pub description: String,
    pub priority: u32,
}

/// DAT download configuration
#[derive(Debug, Clone)]
pub struct DatDownloadConfig {
    pub download_directory: PathBuf,
    pub update_interval_days: u32,
    pub timeout_seconds: u64,
    pub retry_attempts: u32,
    pub concurrent_downloads: usize,
}

impl Default for DatDownloadConfig {
    fn default() -> Self {
        Self {
            download_directory: PathBuf::from("./dat_files"),
            update_interval_days: 7,
            timeout_seconds: 30,
            retry_attempts: 3,
            concurrent_downloads: 4,
        }
    }
}

/// DAT file metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatMetadata {
    pub source_url: String,
    pub downloaded_at: u64,
    pub file_size: u64,
    pub checksum: String,
    pub system: String,
}

/// DAT downloader
pub struct DatDownloader {
    client: Client,
    config: DatDownloadConfig,
    sources: Vec<DatSource>,
}

impl DatDownloader {
    /// Create a new DAT downloader
    pub fn new(config: DatDownloadConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .build()
            .with_context(|| "Failed to create HTTP client")?;

        let sources = Self::load_default_sources();

        Ok(Self {
            client,
            config,
            sources,
        })
    }

    /// Create a new DAT downloader with default config
    pub fn new() -> Result<Self> {
        Self::new(DatDownloadConfig::default())
    }

    /// Set timeout duration
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout_seconds = timeout.as_secs();
        self
    }

    /// Set output directory
    pub fn with_output_directory(mut self, dir: PathBuf) -> Self {
        self.config.download_directory = dir;
        self
    }

    /// Load default DAT sources
    fn load_default_sources() -> Vec<DatSource> {
        vec![
            // No-Intro sources
            DatSource {
                name: "No-Intro Nintendo Entertainment System".to_string(),
                url: "https://datomatic.no-intro.org/stuff/Nintendo%20-%20Nintendo%20Entertainment%20System%20(20231225-000000).dat".to_string(),
                system: "nes".to_string(),
                description: "Official No-Intro NES database".to_string(),
                priority: 1,
            },
            DatSource {
                name: "No-Intro Super Nintendo Entertainment System".to_string(),
                url: "https://datomatic.no-intro.org/stuff/Nintendo%20-%20Super%20Nintendo%20Entertainment%20System%20(20231225-000000).dat".to_string(),
                system: "snes".to_string(),
                description: "Official No-Intro SNES database".to_string(),
                priority: 1,
            },
            DatSource {
                name: "No-Intro Nintendo 64".to_string(),
                url: "https://datomatic.no-intro.org/stuff/Nintendo%20-%20Nintendo%2064%20(20231225-000000).dat".to_string(),
                system: "n64".to_string(),
                description: "Official No-Intro N64 database".to_string(),
                priority: 1,
            },
            DatSource {
                name: "No-Intro Game Boy".to_string(),
                url: "https://datomatic.no-intro.org/stuff/Nintendo%20-%20Game%20Boy%20(20231225-000000).dat".to_string(),
                system: "gb".to_string(),
                description: "Official No-Intro Game Boy database".to_string(),
                priority: 1,
            },
            DatSource {
                name: "No-Intro Game Boy Advance".to_string(),
                url: "https://datomatic.no-intro.org/stuff/Nintendo%20-%20Game%20Boy%20Advance%20(20231225-000000).dat".to_string(),
                system: "gba".to_string(),
                description: "Official No-Intro GBA database".to_string(),
                priority: 1,
            },
            DatSource {
                name: "No-Intro Sega Genesis".to_string(),
                url: "https://datomatic.no-intro.org/stuff/Sega%20-%20Mega%20Drive%20-%20Genesis%20(20231225-000000).dat".to_string(),
                system: "genesis".to_string(),
                description: "Official No-Intro Genesis database".to_string(),
                priority: 1,
            },
            
            // Redump sources (for disc-based systems)
            DatSource {
                name: "Redump Sony PlayStation".to_string(),
                url: "http://redump.org/datfile/psx/".to_string(),
                system: "psx".to_string(),
                description: "Redump PlayStation database".to_string(),
                priority: 2,
            },
            DatSource {
                name: "Redump Nintendo GameCube".to_string(),
                url: "http://redump.org/datfile/gc/".to_string(),
                system: "gamecube".to_string(),
                description: "Redump GameCube database".to_string(),
                priority: 2,
            },
        ]
    }

    /// Add custom DAT source
    pub fn add_source(&mut self, source: DatSource) {
        self.sources.push(source);
        self.sources.sort_by_key(|s| s.priority);
    }

    /// Get list of available systems
    pub fn get_available_systems(&self) -> Result<Vec<String>> {
        let systems: Vec<String> = self.sources
            .iter()
            .map(|source| source.system.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        
        Ok(systems)
    }

    /// Download DAT file for a specific system
    pub fn download_dat_for_system(&self, system: &str, force: bool) -> Result<PathBuf> {
        // Use tokio runtime to handle async download
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(self.download_dat_for_system_async(system, force))
    }

    /// Download DAT file for a specific system (async version)
    async fn download_dat_for_system_async(&self, system: &str, force: bool) -> Result<PathBuf> {
        let source = self.sources
            .iter()
            .find(|s| s.system == system)
            .ok_or_else(|| anyhow::anyhow!("No DAT source found for system: {}", system))?;

        let output_path = self.config.download_directory.join(format!("{}.dat", system));

        // Check if we need to download
        if !force && output_path.exists() {
            if let Ok(metadata) = self.load_metadata(&output_path) {
                let age_days = (SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs() - metadata.downloaded_at) / (24 * 60 * 60);
                
                if age_days < self.config.update_interval_days as u64 {
                    debug!("DAT for {} is up to date, skipping download", system);
                    return Ok(output_path);
                }
            }
        }

        // Ensure output directory exists
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }

        info!("Downloading DAT for {}: {}", system, source.url);

        // Download with retries
        let mut last_error = None;
        for attempt in 1..=self.config.retry_attempts {
            match self.download_file(&source.url, &output_path).await {
                Ok(()) => {
                    // Save metadata
                    let metadata = DatMetadata {
                        source_url: source.url.clone(),
                        downloaded_at: SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs(),
                        file_size: fs::metadata(&output_path)?.len(),
                        checksum: self.calculate_checksum(&output_path)?,
                        system: system.to_string(),
                    };
                    
                    self.save_metadata(&output_path, &metadata)?;
                    
                    info!("Successfully downloaded DAT for {}", system);
                    return Ok(output_path);
                }
                Err(e) => {
                    warn!("Download attempt {} failed for {}: {}", attempt, system, e);
                    last_error = Some(e);
                }
            }
        }

        Err(last_error.unwrap_or_else(|| anyhow::anyhow!("All download attempts failed")))
    }

    /// Download all available DAT files
    pub async fn download_all_dats(&self) -> Result<Vec<DatDownloadResult>> {
        info!("Starting download of {} DAT files", self.sources.len());
        
        // Ensure download directory exists
        fs::create_dir_all(&self.config.download_directory)
            .with_context(|| format!("Failed to create download directory: {}", 
                self.config.download_directory.display()))?;

        let mut results = Vec::new();
        
        // Process downloads with concurrency limit
        let semaphore = tokio::sync::Semaphore::new(self.config.concurrent_downloads);
        let mut tasks = Vec::new();

        for source in &self.sources {
            let permit = semaphore.clone().acquire_owned().await.unwrap();
            let client = self.client.clone();
            let config = self.config.clone();
            let source = source.clone();

            let task = tokio::spawn(async move {
                let result = Self::download_single_dat(client, config, source).await;
                drop(permit);
                result
            });

            tasks.push(task);
        }

        // Wait for all downloads to complete
        for task in tasks {
            match task.await {
                Ok(result) => results.push(result),
                Err(e) => error!("Task failed: {}", e),
            }
        }

        let successful = results.iter().filter(|r| r.success).count();
        info!("DAT download completed: {}/{} successful", successful, results.len());

        Ok(results)
    }

    /// Download a single DAT file
    async fn download_single_dat(
        client: Client,
        config: DatDownloadConfig,
        source: DatSource,
    ) -> DatDownloadResult {
        let start_time = std::time::Instant::now();
        let file_path = config.download_directory.join(format!("{}.dat", source.system));
        
        // Check if file exists and is recent enough
        if let Ok(metadata) = Self::load_dat_metadata(&file_path) {
            let age_days = (SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() - metadata.downloaded_at) / (24 * 60 * 60);
            
            if age_days < config.update_interval_days as u64 {
                debug!("DAT file is recent, skipping: {}", source.name);
                return DatDownloadResult {
                    source_name: source.name,
                    system: source.system,
                    success: true,
                    file_path: Some(file_path),
                    file_size: metadata.file_size,
                    download_time: Duration::ZERO,
                    error: None,
                    skipped: true,
                };
            }
        }

        info!("Downloading DAT: {}", source.name);
        
        let mut last_error = None;
        
        // Retry logic
        for attempt in 1..=config.retry_attempts {
            match Self::perform_download(&client, &source, &file_path).await {
                Ok(download_info) => {
                    let download_time = start_time.elapsed();
                    info!("Downloaded {}: {} bytes in {:?}", 
                        source.name, download_info.file_size, download_time);
                    
                    // Save metadata
                    if let Err(e) = Self::save_dat_metadata(&file_path, &source, &download_info) {
                        warn!("Failed to save DAT metadata: {}", e);
                    }
                    
                    return DatDownloadResult {
                        source_name: source.name,
                        system: source.system,
                        success: true,
                        file_path: Some(file_path),
                        file_size: download_info.file_size,
                        download_time,
                        error: None,
                        skipped: false,
                    };
                },
                Err(e) => {
                    warn!("Download attempt {} failed for {}: {}", attempt, source.name, e);
                    last_error = Some(e.to_string());
                    
                    if attempt < config.retry_attempts {
                        tokio::time::sleep(Duration::from_secs(2_u64.pow(attempt - 1))).await;
                    }
                }
            }
        }

        error!("Failed to download {} after {} attempts", source.name, config.retry_attempts);
        
        DatDownloadResult {
            source_name: source.name,
            system: source.system,
            success: false,
            file_path: None,
            file_size: 0,
            download_time: start_time.elapsed(),
            error: last_error,
            skipped: false,
        }
    }

    /// Perform the actual download
    async fn perform_download(
        client: &Client,
        source: &DatSource,
        file_path: &Path,
    ) -> Result<DownloadInfo> {
        let response = client.get(&source.url)
            .send()
            .await
            .with_context(|| format!("Failed to request URL: {}", source.url))?;

        if !response.status().is_success() {
            anyhow::bail!("HTTP error {}: {}", response.status(), source.url);
        }

        let content = response.bytes()
            .await
            .with_context(|| "Failed to download content")?;

        let file_size = content.len() as u64;
        
        // Calculate checksum
        let checksum = format!("{:x}", md5::compute(&content));

        fs::write(file_path, &content)
            .with_context(|| format!("Failed to write file: {}", file_path.display()))?;

        Ok(DownloadInfo {
            file_size,
            checksum,
        })
    }

    /// Load DAT metadata
    fn load_dat_metadata(dat_path: &Path) -> Result<DatMetadata> {
        let metadata_path = dat_path.with_extension("dat.meta");
        
        if !metadata_path.exists() {
            anyhow::bail!("Metadata file not found");
        }

        let content = fs::read_to_string(&metadata_path)
            .with_context(|| format!("Failed to read metadata: {}", metadata_path.display()))?;

        let metadata: DatMetadata = serde_json::from_str(&content)
            .with_context(|| "Failed to parse metadata")?;

        Ok(metadata)
    }

    /// Save DAT metadata
    fn save_dat_metadata(
        dat_path: &Path,
        source: &DatSource,
        download_info: &DownloadInfo,
    ) -> Result<()> {
        let metadata_path = dat_path.with_extension("dat.meta");
        
        let metadata = DatMetadata {
            source_url: source.url.clone(),
            downloaded_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            file_size: download_info.file_size,
            checksum: download_info.checksum.clone(),
            system: source.system.clone(),
        };

        let content = serde_json::to_string_pretty(&metadata)
            .with_context(|| "Failed to serialize metadata")?;

        fs::write(&metadata_path, content)
            .with_context(|| format!("Failed to write metadata: {}", metadata_path.display()))?;

        Ok(())
    }

    /// Download file from URL
    async fn download_file(&self, url: &str, output_path: &Path) -> Result<()> {
        let response = self.client
            .get(url)
            .send()
            .await
            .with_context(|| format!("Failed to download from: {}", url))?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("HTTP error: {}", response.status()));
        }

        let content = response.bytes().await
            .with_context(|| "Failed to read response body")?;

        fs::write(output_path, content)
            .with_context(|| format!("Failed to write file: {}", output_path.display()))?;

        Ok(())
    }

    /// Calculate file checksum
    fn calculate_checksum(&self, path: &Path) -> Result<String> {
        use md5::{Md5, Digest};
        
        let content = fs::read(path)
            .with_context(|| format!("Failed to read file for checksum: {}", path.display()))?;
        
        let mut hasher = Md5::new();
        hasher.update(&content);
        let hash = hasher.finalize();
        
        Ok(format!("{:x}", hash))
    }

    /// Save metadata for downloaded DAT
    fn save_metadata(&self, dat_path: &Path, metadata: &DatMetadata) -> Result<()> {
        let metadata_path = dat_path.with_extension("dat.meta");
        let content = serde_json::to_string_pretty(metadata)
            .with_context(|| "Failed to serialize metadata")?;
        
        fs::write(&metadata_path, content)
            .with_context(|| format!("Failed to write metadata: {}", metadata_path.display()))?;
        
        Ok(())
    }

    /// Load metadata for a DAT file
    fn load_metadata(&self, dat_path: &Path) -> Result<DatMetadata> {
        let metadata_path = dat_path.with_extension("dat.meta");
        let content = fs::read_to_string(&metadata_path)
            .with_context(|| format!("Failed to read metadata: {}", metadata_path.display()))?;
        
        let metadata: DatMetadata = serde_json::from_str(&content)
            .with_context(|| "Failed to parse metadata")?;
        
        Ok(metadata)
    }

    /// Get DAT file path for system
    pub fn get_dat_path(&self, system: &str) -> PathBuf {
        self.config.download_directory.join(format!("{}.dat", system))
    }
}

/// Download result information
#[derive(Debug)]
pub struct DatDownloadResult {
    pub source_name: String,
    pub system: String,
    pub success: bool,
    pub file_path: Option<PathBuf>,
    pub file_size: u64,
    pub download_time: Duration,
    pub error: Option<String>,
    pub skipped: bool,
}

/// Internal download information
#[derive(Debug)]
struct DownloadInfo {
    file_size: u64,
    checksum: String,
}

/// DAT manager for automatic updates
pub struct DatManager {
    downloader: DatDownloader,
}

impl DatManager {
    /// Create a new DAT manager
    pub fn new(config: DatDownloadConfig) -> Result<Self> {
        let downloader = DatDownloader::new(config)?;
        
        Ok(Self {
            downloader,
        })
    }

    /// Update all DAT files
    pub async fn update_all_dats(&self) -> Result<()> {
        let results = self.downloader.download_all_dats().await?;
        
        let mut successful = 0;
        let mut skipped = 0;
        let mut failed = 0;

        for result in results {
            if result.success {
                if result.skipped {
                    skipped += 1;
                } else {
                    successful += 1;
                }
            } else {
                failed += 1;
                if let Some(error) = result.error {
                    error!("Failed to download {}: {}", result.source_name, error);
                }
            }
        }

        info!("DAT update summary: {} successful, {} skipped, {} failed", 
            successful, skipped, failed);

        if failed > 0 {
            anyhow::bail!("{} DAT downloads failed", failed);
        }

        Ok(())
    }

    /// Check if DAT file exists for system
    pub fn has_dat_for_system(&self, system: &str) -> bool {
        self.downloader.get_dat_path(system).exists()
    }

    /// Get available systems
    pub fn available_systems(&self) -> Vec<String> {
        self.downloader.available_systems()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_dat_downloader_creation() {
        let config = DatDownloadConfig::default();
        let downloader = DatDownloader::new(config);
        assert!(downloader.is_ok());
    }

    #[test]
    fn test_default_sources() {
        let sources = DatDownloader::load_default_sources();
        assert!(!sources.is_empty());
        assert!(sources.iter().any(|s| s.system == "nes"));
        assert!(sources.iter().any(|s| s.system == "snes"));
    }

    #[test]
    fn test_add_custom_source() {
        let config = DatDownloadConfig::default();
        let mut downloader = DatDownloader::new(config).unwrap();
        
        let custom_source = DatSource {
            name: "Custom Test".to_string(),
            url: "http://example.com/test.dat".to_string(),
            system: "test".to_string(),
            description: "Test DAT".to_string(),
            priority: 1,
        };

        downloader.add_source(custom_source);
        assert!(downloader.available_systems().contains(&"test".to_string()));
    }
}
