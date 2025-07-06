pub mod cli;
pub mod scanner;
pub mod playlist;
pub mod crc32;
pub mod dat_parser;
pub mod core_mapper;
pub mod converter;
pub mod platform;
pub mod config;
pub mod error;
pub mod cache;
pub mod archive;
pub mod thread_monitor;

#[cfg(feature = "watch-mode")]
pub mod watch;

#[cfg(feature = "dat-download")]
pub mod dat_downloader;

pub mod validator;
pub mod deduplicator;

// Re-export main types for convenience
pub use scanner::Scanner;
pub use playlist::{PlaylistBuilder, Playlist, PlaylistItem};
pub use platform::Platform;
pub use config::Config;
pub use error::*;
pub use cache::CrcCache;
pub use validator::RomValidator;
pub use deduplicator::RomDeduplicator;
