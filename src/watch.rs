use anyhow::{Result, Context};
use notify::{Watcher, RecursiveMode, Event, EventKind, RecommendedWatcher};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::collections::HashSet;
use log::{info, debug, warn, error};

use crate::scanner::RomScanner;
use crate::playlist::PlaylistGenerator;
use crate::config::Config;
use crate::platform::Platform;

/// File system event types we care about
#[derive(Debug, Clone)]
pub enum FileEvent {
    Created(PathBuf),
    Modified(PathBuf),
    Deleted(PathBuf),
    Renamed { from: PathBuf, to: PathBuf },
}

/// Watch mode configuration
#[derive(Debug, Clone)]
pub struct WatchConfig {
    pub debounce_duration: Duration,
    pub batch_size: usize,
    pub scan_delay: Duration,
    pub include_archives: bool,
}

impl Default for WatchConfig {
    fn default() -> Self {
        Self {
            debounce_duration: Duration::from_millis(500),
            batch_size: 100,
            scan_delay: Duration::from_secs(2),
            include_archives: true,
        }
    }
}

/// File system watcher for ROM directories
pub struct RomWatcher {
    watcher: RecommendedWatcher,
    event_receiver: Receiver<notify::Result<Event>>,
    config: WatchConfig,
    watched_paths: HashSet<PathBuf>,
    last_scan: Option<Instant>,
    pending_events: Vec<FileEvent>,
}

impl RomWatcher {
    /// Create a new ROM watcher
    pub fn new(config: WatchConfig) -> Result<Self> {
        let (tx, rx) = std::sync::mpsc::channel();
        
        let watcher = RecommendedWatcher::new(
            move |result| {
                if let Err(e) = tx.send(result) {
                    error!("Failed to send watch event: {}", e);
                }
            },
            notify::Config::default(),
        ).with_context(|| "Failed to create file system watcher")?;

        Ok(Self {
            watcher,
            event_receiver: rx,
            config,
            watched_paths: HashSet::new(),
            last_scan: None,
            pending_events: Vec::new(),
        })
    }

    /// Add a directory to watch
    pub fn watch_directory<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path = path.as_ref().to_path_buf();
        
        if !path.exists() {
            warn!("Directory does not exist: {}", path.display());
            return Ok(());
        }

        if !path.is_dir() {
            warn!("Path is not a directory: {}", path.display());
            return Ok(());
        }

        self.watcher.watch(&path, RecursiveMode::Recursive)
            .with_context(|| format!("Failed to watch directory: {}", path.display()))?;
        
        self.watched_paths.insert(path.clone());
        info!("Now watching directory: {}", path.display());
        
        Ok(())
    }

    /// Remove a directory from watch
    pub fn unwatch_directory<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path = path.as_ref().to_path_buf();
        
        self.watcher.unwatch(&path)
            .with_context(|| format!("Failed to unwatch directory: {}", path.display()))?;
        
        self.watched_paths.remove(&path);
        info!("Stopped watching directory: {}", path.display());
        
        Ok(())
    }

    /// Process file system events
    pub fn process_events(&mut self) -> Result<Vec<FileEvent>> {
        let mut new_events = Vec::new();
        
        // Collect all pending events without blocking
        while let Ok(result) = self.event_receiver.try_recv() {
            match result {
                Ok(event) => {
                    if let Some(file_event) = self.convert_event(event)? {
                        new_events.push(file_event);
                    }
                },
                Err(e) => {
                    warn!("File system watch error: {}", e);
                }
            }
        }

        // Add to pending events
        self.pending_events.extend(new_events);
        
        // Check if we should process the batch
        let should_process = self.should_process_batch();
        
        if should_process {
            let events = std::mem::take(&mut self.pending_events);
            self.last_scan = Some(Instant::now());
            Ok(self.deduplicate_events(events))
        } else {
            Ok(Vec::new())
        }
    }

    /// Convert notify event to our event type
    fn convert_event(&self, event: Event) -> Result<Option<FileEvent>> {
        use notify::EventKind;
        
        let paths = event.paths;
        if paths.is_empty() {
            return Ok(None);
        }

        // Only process ROM files and archives
        let first_path = &paths[0];
        if !self.is_relevant_file(first_path) {
            return Ok(None);
        }

        let file_event = match event.kind {
            EventKind::Create(_) => {
                debug!("File created: {}", first_path.display());
                Some(FileEvent::Created(first_path.clone()))
            },
            EventKind::Modify(_) => {
                debug!("File modified: {}", first_path.display());
                Some(FileEvent::Modified(first_path.clone()))
            },
            EventKind::Remove(_) => {
                debug!("File deleted: {}", first_path.display());
                Some(FileEvent::Deleted(first_path.clone()))
            },
            EventKind::Other => {
                // Check for rename events
                if paths.len() == 2 {
                    debug!("File renamed: {} -> {}", paths[0].display(), paths[1].display());
                    Some(FileEvent::Renamed {
                        from: paths[0].clone(),
                        to: paths[1].clone(),
                    })
                } else {
                    None
                }
            },
            _ => None,
        };

        Ok(file_event)
    }

    /// Check if file is relevant for ROM scanning
    fn is_relevant_file(&self, path: &Path) -> bool {
        if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
            let ext_lower = extension.to_lowercase();
            
            // ROM extensions
            let is_rom = matches!(ext_lower.as_str(),
                "nes" | "fds" | "unf" | "unif" |
                "smc" | "sfc" | "swc" | "fig" |
                "z64" | "n64" | "v64" |
                "gcm" | "iso" | "gcz" | "rvz" |
                "gb" | "gbc" | "gba" |
                "nds" | "dsi" | "ids" |
                "3ds" | "cci" | "cxi" |
                "sms" | "sg" |
                "md" | "smd" | "gen" | "bin" |
                "32x" | "gg" |
                "cdi" | "gdi" | "chd" |
                "pbp" | "cue" | "img" | "mdf" | "nrg" | "cso" |
                "a26" | "a78" |
                "pce" | "sgx" | "ws" | "wsc" | "neo"
            );
            
            // Archive extensions
            let is_archive = self.config.include_archives && matches!(ext_lower.as_str(),
                "zip" | "7z" | "rar"
            );
            
            is_rom || is_archive
        } else {
            false
        }
    }

    /// Check if we should process the current batch of events
    fn should_process_batch(&self) -> bool {
        if self.pending_events.is_empty() {
            return false;
        }

        // If we have too many events, process immediately
        if self.pending_events.len() >= self.config.batch_size {
            return true;
        }

        // If enough time has passed since last scan, process
        if let Some(last_scan) = self.last_scan {
            if last_scan.elapsed() >= self.config.scan_delay {
                return true;
            }
        } else {
            // First time, wait for debounce duration
            return true;
        }

        false
    }

    /// Remove duplicate events and merge related events
    fn deduplicate_events(&self, events: Vec<FileEvent>) -> Vec<FileEvent> {
        let mut unique_events = Vec::new();
        let mut processed_paths = HashSet::new();

        for event in events.into_iter().rev() { // Process in reverse to keep latest events
            let path = match &event {
                FileEvent::Created(p) | FileEvent::Modified(p) | FileEvent::Deleted(p) => p.clone(),
                FileEvent::Renamed { to, .. } => to.clone(),
            };

            if !processed_paths.contains(&path) {
                processed_paths.insert(path);
                unique_events.push(event);
            }
        }

        unique_events.reverse();
        unique_events
    }

    /// Get list of watched directories
    pub fn watched_directories(&self) -> Vec<PathBuf> {
        self.watched_paths.iter().cloned().collect()
    }

    /// Get watch statistics
    pub fn stats(&self) -> WatchStats {
        WatchStats {
            watched_directories: self.watched_paths.len(),
            pending_events: self.pending_events.len(),
            last_scan: self.last_scan,
        }
    }

    /// Run event loop
    pub fn run_event_loop(&mut self) -> Result<()> {
        info!("Starting watch event loop...");
        
        loop {
            // Process events with timeout
            match self.event_receiver.recv_timeout(self.config.debounce_duration) {
                Ok(Ok(event)) => {
                    if let Ok(Some(file_event)) = self.process_event(event) {
                        self.pending_events.push(file_event);
                        
                        // Process batch if we have enough events
                        if self.pending_events.len() >= self.config.batch_size {
                            self.process_pending_events()?;
                        }
                    }
                }
                Ok(Err(e)) => {
                    warn!("Watch error: {}", e);
                }
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                    // Timeout reached - process any pending events
                    if !self.pending_events.is_empty() {
                        self.process_pending_events()?;
                    }
                }
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                    warn!("Watch channel disconnected");
                    break;
                }
            }
        }
        
        Ok(())
    }

    /// Process accumulated pending events
    fn process_pending_events(&mut self) -> Result<()> {
        if self.pending_events.is_empty() {
            return Ok(());
        }

        // Debounce: wait a bit more if we just processed events
        if let Some(last_scan) = self.last_scan {
            if last_scan.elapsed() < self.config.scan_delay {
                debug!("Debouncing scan, waiting for scan delay");
                return Ok(());
            }
        }

        info!("Processing {} file events", self.pending_events.len());
        
        // Group events by directory to minimize scans
        let mut affected_dirs = HashSet::new();
        
        for event in &self.pending_events {
            match event {
                FileEvent::Created(path) | 
                FileEvent::Modified(path) | 
                FileEvent::Deleted(path) => {
                    if let Some(parent) = path.parent() {
                        affected_dirs.insert(parent.to_path_buf());
                    }
                }
                FileEvent::Renamed { from, to } => {
                    if let Some(parent) = from.parent() {
                        affected_dirs.insert(parent.to_path_buf());
                    }
                    if let Some(parent) = to.parent() {
                        affected_dirs.insert(parent.to_path_buf());
                    }
                }
            }
        }

        // Trigger rescan for affected directories
        for dir in affected_dirs {
            info!("Rescanning directory: {}", dir.display());
            // Here you would integrate with your scanner and playlist builder
            // For now, just log the action
            debug!("Would rescan directory: {}", dir.display());
        }

        // Clear processed events
        self.pending_events.clear();
        self.last_scan = Some(Instant::now());
        
        Ok(())
    }
}

/// Watch statistics
#[derive(Debug)]
pub struct WatchStats {
    pub watched_directories: usize,
    pub pending_events: usize,
    pub last_scan: Option<Instant>,
}

/// Watch mode runner
pub struct WatchRunner {
    watcher: RomWatcher,
    scanner: RomScanner,
    playlist_generator: PlaylistGenerator,
    config: Arc<Config>,
}

impl WatchRunner {
    /// Create a new watch runner
    pub fn new(
        watch_config: WatchConfig,
        scanner: RomScanner,
        playlist_generator: PlaylistGenerator,
        config: Arc<Config>,
    ) -> Result<Self> {
        let watcher = RomWatcher::new(watch_config)?;
        
        Ok(Self {
            watcher,
            scanner,
            playlist_generator,
            config,
        })
    }

    /// Start watching the configured ROM directories
    pub fn start_watching(&mut self) -> Result<()> {
        for rom_dir in &self.config.paths.roms_directories {
            self.watcher.watch_directory(rom_dir)?;
        }
        
        info!("Started watching {} directories", self.config.paths.roms_directories.len());
        Ok(())
    }

    /// Run the watch loop
    pub fn run(&mut self) -> Result<()> {
        info!("Starting ROM directory watch mode...");
        self.start_watching()?;

        loop {
            // Process file system events
            let events = self.watcher.process_events()?;
            
            if !events.is_empty() {
                info!("Processing {} file system events", events.len());
                self.handle_events(events)?;
            }

            // Sleep to avoid busy-waiting
            std::thread::sleep(Duration::from_millis(100));
        }
    }

    /// Handle file system events
    fn handle_events(&mut self, events: Vec<FileEvent>) -> Result<()> {
        let mut should_rescan = false;
        
        for event in events {
            match event {
                FileEvent::Created(path) => {
                    info!("New ROM detected: {}", path.display());
                    should_rescan = true;
                },
                FileEvent::Modified(path) => {
                    info!("ROM modified: {}", path.display());
                    should_rescan = true;
                },
                FileEvent::Deleted(path) => {
                    info!("ROM deleted: {}", path.display());
                    should_rescan = true;
                },
                FileEvent::Renamed { from, to } => {
                    info!("ROM renamed: {} -> {}", from.display(), to.display());
                    should_rescan = true;
                },
            }
        }

        if should_rescan {
            self.perform_incremental_scan()?;
        }

        Ok(())
    }

    /// Perform an incremental scan and update playlists
    fn perform_incremental_scan(&mut self) -> Result<()> {
        info!("Performing incremental ROM scan...");
        
        // TODO: Implement incremental scanning logic
        // For now, do a full rescan
        let start_time = Instant::now();
        
        // Scan all ROM directories
        let mut all_roms = Vec::new();
        for rom_dir in &self.config.paths.roms_directories {
            let roms = self.scanner.scan_directory(rom_dir)?;
            all_roms.extend(roms);
        }

        // Generate playlists
        let output_dir = &self.config.paths.output_directory;
        self.playlist_generator.generate_all_playlists(&all_roms, output_dir)?;
        
        let elapsed = start_time.elapsed();
        info!("Incremental scan completed in {:?} - found {} ROMs", elapsed, all_roms.len());
        
        Ok(())
    }
}

/// Watch service for monitoring ROM directories and auto-updating playlists
pub struct WatchService {
    config: WatchConfig,
    watched_directories: Vec<PathBuf>,
    source_platform: Platform,
    target_platform: Platform,
    output_dir: PathBuf,
}

impl WatchService {
    /// Create a new watch service
    pub fn new() -> Self {
        Self {
            config: WatchConfig::default(),
            watched_directories: Vec::new(),
            source_platform: Platform::Windows,
            target_platform: Platform::Windows,
            output_dir: PathBuf::from("./playlists"),
        }
    }

    /// Set debounce duration
    pub fn with_debounce_duration(mut self, duration: Duration) -> Self {
        self.config.debounce_duration = duration;
        self
    }

    /// Set batch size
    pub fn with_batch_size(mut self, size: usize) -> Self {
        self.config.batch_size = size;
        self
    }

    /// Set include archives flag
    pub fn with_include_archives(mut self, include: bool) -> Self {
        self.config.include_archives = include;
        self
    }

    /// Set platforms
    pub fn with_platforms(mut self, source: Platform, target: Platform) -> Self {
        self.source_platform = source;
        self.target_platform = target;
        self
    }

    /// Set output directory
    pub fn with_output_dir(mut self, dir: PathBuf) -> Self {
        self.output_dir = dir;
        self
    }

    /// Add directory to watch
    pub fn add_watch_directory(&mut self, path: PathBuf) -> Result<()> {
        if !path.exists() {
            return Err(anyhow::anyhow!("Directory does not exist: {}", path.display()));
        }
        
        self.watched_directories.push(path);
        Ok(())
    }

    /// Start watching directories
    pub fn start_watching(&self) -> Result<()> {
        let mut watcher = RomWatcher::new(self.config.clone())?;
        
        // Watch all directories
        for dir in &self.watched_directories {
            watcher.watch_directory(dir)?;
        }

        // Start event loop
        watcher.run_event_loop()?;
        
        Ok(())
    }
}

impl Default for WatchService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    #[test]
    fn test_watcher_creation() {
        let config = WatchConfig::default();
        let watcher = RomWatcher::new(config);
        assert!(watcher.is_ok());
    }

    #[test]
    fn test_file_relevance() {
        let config = WatchConfig::default();
        let watcher = RomWatcher::new(config).unwrap();
        
        assert!(watcher.is_relevant_file(Path::new("test.nes")));
        assert!(watcher.is_relevant_file(Path::new("test.zip")));
        assert!(!watcher.is_relevant_file(Path::new("test.txt")));
        assert!(!watcher.is_relevant_file(Path::new("test.jpg")));
    }

    #[test]
    fn test_watch_directory() {
        let temp_dir = tempdir().unwrap();
        let config = WatchConfig::default();
        let mut watcher = RomWatcher::new(config).unwrap();
        
        let result = watcher.watch_directory(temp_dir.path());
        assert!(result.is_ok());
        
        assert_eq!(watcher.watched_directories().len(), 1);
        assert!(watcher.watched_directories().contains(&temp_dir.path().to_path_buf()));
    }
}
