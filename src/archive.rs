use anyhow::Result;
use std::io::Read;
use std::path::{Path, PathBuf};

#[cfg(feature = "archive-support")]
use zip::ZipArchive;

#[cfg(feature = "archive-support")]
use sevenz_rust::SevenZReader;

/// Archive entry information
#[derive(Debug, Clone)]
pub struct ArchiveEntry {
    pub name: String,
    pub size: u64,
    pub is_rom: bool,
    pub extension: String,
}

/// Archive reader trait
pub trait ArchiveReader {
    fn list_entries(&mut self) -> Result<Vec<ArchiveEntry>>;
    fn extract_entry(&mut self, entry_name: &str) -> Result<Vec<u8>>;
    fn get_entry_reader(&mut self, entry_name: &str) -> Result<Box<dyn Read>>;
}

/// ZIP archive reader
#[cfg(feature = "archive-support")]
pub struct ZipReader<R: Read + Seek> {
    archive: ZipArchive<R>,
}

#[cfg(feature = "archive-support")]
impl<R: Read + Seek> ZipReader<R> {
    pub fn new(reader: R) -> Result<Self> {
        let archive = ZipArchive::new(reader)
            .with_context(|| "Failed to open ZIP archive")?;
        
        Ok(Self { archive })
    }
}

#[cfg(feature = "archive-support")]
impl<R: Read + Seek> ArchiveReader for ZipReader<R> {
    fn list_entries(&mut self) -> Result<Vec<ArchiveEntry>> {
        let mut entries = Vec::new();
        
        for i in 0..self.archive.len() {
            let file = self.archive.by_index(i)
                .with_context(|| format!("Failed to get ZIP entry at index {}", i))?;
            
            let name = file.name().to_string();
            let size = file.size();
            
            // Skip directories
            if name.ends_with('/') {
                continue;
            }
            
            let path = Path::new(&name);
            let extension = path.extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("")
                .to_lowercase();
            
            let is_rom = is_rom_extension(&extension);
            
            entries.push(ArchiveEntry {
                name,
                size,
                is_rom,
                extension,
            });
        }
        
        Ok(entries)
    }
    
    fn extract_entry(&mut self, entry_name: &str) -> Result<Vec<u8>> {
        let mut file = self.archive.by_name(entry_name)
            .with_context(|| format!("Failed to find entry: {}", entry_name))?;
        
        let mut data = Vec::with_capacity(file.size() as usize);
        file.read_to_end(&mut data)
            .with_context(|| format!("Failed to read entry: {}", entry_name))?;
        
        Ok(data)
    }
    
    fn get_entry_reader(&mut self, entry_name: &str) -> Result<Box<dyn Read>> {
        let file = self.archive.by_name(entry_name)
            .with_context(|| format!("Failed to find entry: {}", entry_name))?;
        
        Ok(Box::new(file))
    }
}

/// 7z archive reader
#[cfg(feature = "archive-support")]
pub struct SevenZipReader {
    reader: SevenZReader<std::fs::File>,
}

#[cfg(feature = "archive-support")]
impl SevenZipReader {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = std::fs::File::open(path.as_ref())
            .with_context(|| format!("Failed to open 7z file: {}", path.as_ref().display()))?;
        
        let reader = SevenZReader::new(file)
            .with_context(|| "Failed to create 7z reader")?;
        
        Ok(Self { reader })
    }
}

#[cfg(feature = "archive-support")]
impl ArchiveReader for SevenZipReader {
    fn list_entries(&mut self) -> Result<Vec<ArchiveEntry>> {
        let mut entries = Vec::new();
        
        let archive_entries = self.reader.archive().files;
        
        for (i, entry) in archive_entries.iter().enumerate() {
            if entry.is_directory() {
                continue;
            }
            
            let name = entry.name().to_string();
            let size = entry.size();
            
            let path = Path::new(&name);
            let extension = path.extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("")
                .to_lowercase();
            
            let is_rom = is_rom_extension(&extension);
            
            entries.push(ArchiveEntry {
                name,
                size,
                is_rom,
                extension,
            });
        }
        
        Ok(entries)
    }
    
    fn extract_entry(&mut self, entry_name: &str) -> Result<Vec<u8>> {
        // Find entry by name
        let entry_index = self.reader.archive().files
            .iter()
            .position(|entry| entry.name() == entry_name)
            .with_context(|| format!("Entry not found: {}", entry_name))?;
        
        let mut data = Vec::new();
        self.reader.extract_to_writer(entry_index, &mut data)
            .with_context(|| format!("Failed to extract entry: {}", entry_name))?;
        
        Ok(data)
    }
    
    fn get_entry_reader(&mut self, entry_name: &str) -> Result<Box<dyn Read>> {
        // For 7z, we need to extract to memory first
        let data = self.extract_entry(entry_name)?;
        Ok(Box::new(std::io::Cursor::new(data)))
    }
}

/// Check if extension is a ROM extension
fn is_rom_extension(extension: &str) -> bool {
    matches!(extension,
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
        "pce" | "sgx" | "ws" | "wsc" | "neo"
    )
}

/// Archive format detection
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArchiveFormat {
    Zip,
    SevenZip,
    Unknown,
}

impl ArchiveFormat {
    pub fn detect_from_path<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref();
        
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("zip") => ArchiveFormat::Zip,
            Some("7z") => ArchiveFormat::SevenZip,
            _ => ArchiveFormat::Unknown,
        }
    }
    
    pub fn detect_from_bytes(data: &[u8]) -> Self {
        if data.len() < 4 {
            return ArchiveFormat::Unknown;
        }
        
        // ZIP magic: PK\x03\x04 or PK\x05\x06 or PK\x07\x08
        if data.starts_with(b"PK\x03\x04") || 
           data.starts_with(b"PK\x05\x06") || 
           data.starts_with(b"PK\x07\x08") {
            return ArchiveFormat::Zip;
        }
        
        // 7z magic: 7z\xBC\xAF\x27\x1C
        if data.len() >= 6 && data.starts_with(b"7z\xBC\xAF\x27\x1C") {
            return ArchiveFormat::SevenZip;
        }
        
        ArchiveFormat::Unknown
    }
}

/// Archive reader factory
pub struct ArchiveReaderFactory;

impl ArchiveReaderFactory {
    #[cfg(feature = "archive-support")]
    pub fn create_from_file<P: AsRef<Path>>(path: P) -> Result<Box<dyn ArchiveReader>> {
        let path = path.as_ref();
        let format = ArchiveFormat::detect_from_path(path);
        
        match format {
            ArchiveFormat::Zip => {
                let file = std::fs::File::open(path)
                    .with_context(|| format!("Failed to open ZIP file: {}", path.display()))?;
                let reader = ZipReader::new(file)?;
                Ok(Box::new(reader))
            },
            ArchiveFormat::SevenZip => {
                let reader = SevenZipReader::new(path)?;
                Ok(Box::new(reader))
            },
            ArchiveFormat::Unknown => {
                anyhow::bail!("Unsupported archive format: {}", path.display());
            }
        }
    }
    
    #[cfg(not(feature = "archive-support"))]
    pub fn create_from_file<P: AsRef<Path>>(_path: P) -> Result<Box<dyn ArchiveReader>> {
        anyhow::bail!("Archive support not compiled in. Enable 'archive-support' feature.");
    }
}

/// Virtual ROM file from archive
#[derive(Debug, Clone)]
pub struct VirtualRomFile {
    pub archive_path: PathBuf,
    pub entry_name: String,
    pub size: u64,
    pub extension: String,
    pub virtual_path: PathBuf,
}

impl VirtualRomFile {
    pub fn new(archive_path: PathBuf, entry: ArchiveEntry) -> Self {
        let virtual_path = archive_path
            .with_file_name(format!("{}#{}", 
                archive_path.file_name().unwrap().to_string_lossy(),
                entry.name
            ));
        
        Self {
            archive_path,
            entry_name: entry.name,
            size: entry.size,
            extension: entry.extension,
            virtual_path,
        }
    }
    
    /// Extract ROM data from archive
    #[cfg(feature = "archive-support")]
    pub fn extract_data(&self) -> Result<Vec<u8>> {
        let mut reader = ArchiveReaderFactory::create_from_file(&self.archive_path)?;
        reader.extract_entry(&self.entry_name)
    }
    
    #[cfg(not(feature = "archive-support"))]
    pub fn extract_data(&self) -> Result<Vec<u8>> {
        anyhow::bail!("Archive support not compiled in");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_archive_format_detection() {
        assert_eq!(ArchiveFormat::detect_from_path("test.zip"), ArchiveFormat::Zip);
        assert_eq!(ArchiveFormat::detect_from_path("test.7z"), ArchiveFormat::SevenZip);
        assert_eq!(ArchiveFormat::detect_from_path("test.rar"), ArchiveFormat::Unknown);
    }
    
    #[test]
    fn test_rom_extension_detection() {
        assert!(is_rom_extension("nes"));
        assert!(is_rom_extension("sfc"));
        assert!(is_rom_extension("z64"));
        assert!(!is_rom_extension("txt"));
        assert!(!is_rom_extension("jpg"));
    }
}
