use thiserror::Error;
use std::path::PathBuf;

#[derive(Error, Debug)]
pub enum ScannerError {
    #[error("Diretório não encontrado: {0}")]
    DirectoryNotFound(PathBuf),
    
    #[error("Arquivo não é uma ROM válida: {0}")]
    InvalidRomFile(PathBuf),
    
    #[error("Falha ao calcular CRC32 para {file}: {source}")]
    CrcCalculationFailed {
        file: PathBuf,
        source: std::io::Error,
    },
    
    #[error("Sistema não detectado para: {0}")]
    SystemNotDetected(PathBuf),
}

#[derive(Error, Debug)]
pub enum PlaylistError {
    #[error("Falha ao serializar playlist: {0}")]
    SerializationFailed(#[from] serde_json::Error),
    
    #[error("Falha ao salvar playlist em {path}: {source}")]
    SaveFailed {
        path: PathBuf,
        source: std::io::Error,
    },
    
    #[error("Falha ao carregar playlist de {path}: {source}")]
    LoadFailed {
        path: PathBuf,
        source: std::io::Error,
    },
    
    #[error("Formato de playlist inválido: {0}")]
    InvalidFormat(String),
}

#[derive(Error, Debug)]
pub enum ConverterError {
    #[error("Plataforma não suportada: {0}")]
    UnsupportedPlatform(String),
    
    #[error("Falha na conversão de caminho: {original} -> {target}")]
    PathConversionFailed {
        original: String,
        target: String,
    },
    
    #[error("Core não encontrado para sistema: {0}")]
    CoreNotFound(String),
    
    #[error("Plataforma de origem não detectada")]
    SourcePlatformNotDetected,
}

#[derive(Error, Debug)]
pub enum DatError {
    #[error("Falha ao carregar arquivo DAT {path}: {source}")]
    LoadFailed {
        path: PathBuf,
        source: std::io::Error,
    },
    
    #[error("Formato DAT inválido em {path}: {message}")]
    InvalidFormat {
        path: PathBuf,
        message: String,
    },
    
    #[error("Download DAT falhou para {url}: {source}")]
    DownloadFailed {
        url: String,
        #[cfg(feature = "dat-download")]
        #[source]
        source: reqwest::Error,
        #[cfg(not(feature = "dat-download"))]
        source: std::io::Error,
    },
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Falha ao carregar configuração de {path}: {source}")]
    LoadFailed {
        path: PathBuf,
        source: std::io::Error,
    },
    
    #[error("Falha ao salvar configuração em {path}: {source}")]
    SaveFailed {
        path: PathBuf,
        source: std::io::Error,
    },
    
    #[error("Formato de configuração inválido: {0}")]
    InvalidFormat(#[from] toml::de::Error),
    
    #[error("Falha na serialização da configuração: {0}")]
    SerializationFailed(#[from] toml::ser::Error),
}
