use clap::{Parser, Subcommand};
use std::path::PathBuf;
use crate::platform::Platform;

#[derive(Parser, Clone)]
#[command(
    name = "retroarch-indexer",
    about = "Um indexador universal de ROMs de alta performance para RetroArch",
    version = "1.3.3",
    long_about = "
🎮 RetroArch Fast Playlist Indexer

Um indexador universal de ROMs de alta performance escrito em Rust, projetado para 
gerar playlists RetroArch (.lpl) com velocidade superior ao scanner nativo.

Características principais:
• 15-17x mais rápido que o scanner nativo do RetroArch
• Conversão automática de caminhos entre plataformas
• Playlist master unificada com todas as ROMs
• Suporte a arquivos DAT para nomenclatura precisa
• Processamento paralelo otimizado

Exemplos de uso:
  # Indexar ROMs e criar playlists
  retroarch-indexer --roms-dir /path/to/roms

  # Converter playlist existente entre plataformas
  retroarch-indexer convert Nintendo\\ 64.lpl --target switch

  # Conversão em lote
  retroarch-indexer convert-all --input-dir ./playlists --source windows --target switch
"
)]
pub struct Args {
    /// Diretórios contendo as ROMs (pode ser usado múltiplas vezes)
    #[arg(long = "roms-dir", value_name = "PATH")]
    pub roms_dirs: Vec<PathBuf>,

    /// Plataforma de origem (onde está executando)
    #[arg(long, value_enum)]
    pub source_platform: Option<Platform>,

    /// Plataforma de destino (onde será usado)
    #[arg(long, value_enum)]
    pub target_platform: Option<Platform>,

    /// Diretório para salvar as playlists
    #[arg(long, default_value = "./playlists")]
    pub output_dir: PathBuf,

    /// Diretório contendo arquivos DAT
    #[arg(long)]
    pub dat_dir: Option<PathBuf>,

    /// Baixa DATs automaticamente do No-Intro/Redump
    #[arg(long)]
    pub auto_download_dats: bool,

    /// Força sistema específico (ignora auto-detecção)
    #[arg(long)]
    pub system: Option<String>,

    /// Número de threads paralelas
    #[arg(long)]
    pub threads: Option<usize>,

    /// Desabilita busca recursiva em subdiretórios
    #[arg(long)]
    pub no_recursive: bool,

    /// Extensões customizadas (separadas por vírgula)
    #[arg(long, value_delimiter = ',')]
    pub extensions: Option<Vec<String>>,

    /// Não criar playlist master roms.lpl
    #[arg(long)]
    pub skip_master: bool,

    /// Arquivo de configuração TOML
    #[arg(long)]
    pub config: Option<PathBuf>,

    /// Simula execução sem criar arquivos
    #[arg(long)]
    pub dry_run: bool,

    /// Sobrescreve playlists existentes
    #[arg(long)]
    pub force: bool,

    /// Modo silencioso (apenas erros)
    #[arg(long, short)]
    pub quiet: bool,

    /// Modo verboso
    #[arg(long, short, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Gera relatório de ROMs não identificadas
    #[arg(long)]
    pub report: Option<PathBuf>,

    /// Pula cálculo de CRC32 (mais rápido, menos preciso)
    #[arg(long)]
    pub no_crc: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    /// Converte uma playlist específica entre plataformas
    Convert {
        /// Arquivo de playlist para converter
        input: PathBuf,

        /// Plataforma de origem (auto-detectada se não especificada)
        #[arg(long, value_enum)]
        source: Option<Platform>,

        /// Plataforma de destino
        #[arg(long, value_enum)]
        target: Platform,

        /// Diretório para salvar a playlist convertida
        #[arg(long)]
        output_dir: Option<PathBuf>,

        /// Valida se os caminhos convertidos existem
        #[arg(long)]
        validate_paths: bool,
    },

    /// Converte todas as playlists de um diretório
    ConvertAll {
        /// Diretório contendo playlists para converter
        #[arg(long, default_value = ".")]
        input_dir: PathBuf,

        /// Plataforma de origem
        #[arg(long, value_enum)]
        source: Platform,

        /// Plataforma de destino
        #[arg(long, value_enum)]
        target: Platform,

        /// Diretório para salvar as playlists convertidas
        #[arg(long)]
        output_dir: Option<PathBuf>,

        /// Valida se os caminhos convertidos existem
        #[arg(long)]
        validate_paths: bool,
    },

    /// Monitora diretórios de ROMs e atualiza playlists automaticamente
    #[cfg(feature = "watch-mode")]
    Watch {
        /// Intervalo de debounce em milissegundos
        #[arg(long, default_value = "500")]
        debounce: u64,

        /// Número máximo de eventos por lote
        #[arg(long, default_value = "100")]
        batch_size: usize,

        /// Incluir arquivos de arquivo no monitoramento
        #[arg(long)]
        include_archives: bool,
    },

    /// Baixa arquivos DAT automaticamente
    #[cfg(feature = "dat-download")]
    DownloadDats {
        /// Diretório para salvar os DATs
        #[arg(long, default_value = "./dat_files")]
        output_dir: PathBuf,

        /// Sistemas específicos para baixar (todos se não especificado)
        #[arg(long, value_delimiter = ',')]
        systems: Option<Vec<String>>,

        /// Forçar re-download mesmo se o arquivo já existir
        #[arg(long)]
        force: bool,

        /// Timeout para downloads em segundos
        #[arg(long, default_value = "30")]
        timeout: u64,
    },

    /// Valida integridade de ROMs usando arquivos DAT
    Validate {
        /// Diretório contendo arquivos DAT
        #[arg(long)]
        dat_dir: PathBuf,

        /// Arquivo para salvar o relatório de validação
        #[arg(long)]
        report: Option<PathBuf>,

        /// Sistemas específicos para validar
        #[arg(long, value_delimiter = ',')]
        systems: Option<Vec<String>>,
    },

    /// Remove ROMs duplicados inteligentemente
    Deduplicate {
        /// Estratégia de deduplicação
        #[arg(long, value_enum, default_value = "filename-quality")]
        strategy: DeduplicationStrategy,

        /// Diretórios com prioridade (ordem crescente)
        #[arg(long, value_delimiter = ',')]
        priority_dirs: Option<Vec<PathBuf>>,

        /// Apenas simular (não remover arquivos)
        #[arg(long)]
        dry_run: bool,

        /// Criar backup antes de remover
        #[arg(long)]
        backup: bool,

        /// Diretório para backups
        #[arg(long)]
        backup_dir: Option<PathBuf>,

        /// Arquivo para salvar o relatório
        #[arg(long)]
        report: Option<PathBuf>,
    },

    /// Gerencia cache de CRC32
    Cache {
        #[command(subcommand)]
        action: CacheAction,
    },
}

/// Estratégias de deduplicação
#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum DeduplicationStrategy {
    /// Prioridade por região (USA > Europe > Japan)
    #[value(name = "region-priority")]
    RegionPriority,
    /// Maior tamanho de arquivo
    #[value(name = "file-size")]
    FileSize,
    /// Data de modificação mais recente
    #[value(name = "modification-date")]
    ModificationDate,
    /// Prioridade por diretório
    #[value(name = "directory-priority")]
    DirectoryPriority,
    /// Qualidade do nome do arquivo
    #[value(name = "filename-quality")]
    FilenameQuality,
}

/// Ações do cache
#[derive(Subcommand, Debug, Clone)]
pub enum CacheAction {
    /// Limpa o cache de CRC32
    Clear,
    /// Mostra estatísticas do cache
    Stats,
    /// Limpa entradas antigas do cache
    Clean {
        /// Idade máxima em dias
        #[arg(long, default_value = "30")]
        max_age: u64,
    },
}
