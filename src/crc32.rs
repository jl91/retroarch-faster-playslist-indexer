use anyhow::{Result, Context};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

const BUFFER_SIZE: usize = 64 * 1024; // 64KB buffer

pub fn calculate_crc32(file_path: &Path) -> Result<u32> {
    let file = File::open(file_path)
        .with_context(|| format!("Falha ao abrir arquivo: {}", file_path.display()))?;

    let mut reader = BufReader::with_capacity(BUFFER_SIZE, file);
    let mut crc = crc32fast::Hasher::new();
    let mut buffer = vec![0u8; BUFFER_SIZE];

    loop {
        let bytes_read = reader.read(&mut buffer)
            .with_context(|| format!("Falha ao ler arquivo: {}", file_path.display()))?;
        
        if bytes_read == 0 {
            break;
        }
        
        crc.update(&buffer[..bytes_read]);
    }

    Ok(crc.finalize())
}

pub fn calculate_crc32_from_bytes(data: &[u8]) -> u32 {
    crc32fast::hash(data)
}

pub fn calculate_crc32_zip_entry(zip_data: &[u8], entry_name: &str) -> Result<u32> {
    #[cfg(feature = "archive-support")]
    {
        let cursor = std::io::Cursor::new(zip_data);
        let mut archive = zip::ZipArchive::new(cursor)
            .context("Falha ao abrir arquivo ZIP")?;

        let mut file = archive.by_name(entry_name)
            .with_context(|| format!("Arquivo {} não encontrado no ZIP", entry_name))?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .context("Falha ao ler entrada do ZIP")?;

        Ok(calculate_crc32_from_bytes(&buffer))
    }
    
    #[cfg(not(feature = "archive-support"))]
    {
        let _ = (zip_data, entry_name);
        Err(anyhow::anyhow!("Suporte a ZIP não habilitado (compile com --features archive-support)"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_crc32_calculation() {
        // Create a temporary file with known content
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"Hello, World!").unwrap();
        
        let crc = calculate_crc32(temp_file.path()).unwrap();
        
        // Known CRC32 for "Hello, World!"
        assert_eq!(crc, 0x65A8E27D);
    }

    #[test]
    fn test_crc32_from_bytes() {
        let data = b"Hello, World!";
        let crc = calculate_crc32_from_bytes(data);
        assert_eq!(crc, 0x65A8E27D);
    }

    #[test]
    fn test_empty_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        // Don't write anything - empty file
        
        let crc = calculate_crc32(temp_file.path()).unwrap();
        assert_eq!(crc, 0); // CRC32 of empty data is 0
    }
}
