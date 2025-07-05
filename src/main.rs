use anyhow::Result;
use clap::Parser;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use log::error;
use std::path::PathBuf;

mod cli;
mod scanner;
mod playlist;
mod crc32;
mod dat_parser;
mod core_mapper;
mod converter;
mod platform;
mod config;
mod error;
mod cache;
mod archive;
#[cfg(feature = "watch-mode")]
mod watch;
#[cfg(feature = "dat-download")]
mod dat_downloader;
mod validator;
mod deduplicator;

use cli::{Args, Commands, CacheAction, DeduplicationStrategy};
use scanner::Scanner;
use playlist::PlaylistBuilder;
use converter::PlaylistConverter;
use platform::Platform;
use config::Config;

#[cfg(feature = "watch-mode")]
use watch::WatchService;
#[cfg(feature = "dat-download")]
use dat_downloader::DatDownloader;
use validator::RomValidator;
use deduplicator::RomDeduplicator;
use cache::CrcCache;

fn main() -> Result<()> {
    env_logger::init();
    
    let args = Args::parse();
    
    // Print banner
    print_banner();
    
    match args.command {
        Some(Commands::Convert { input, source, target, output_dir, validate_paths }) => {
            handle_convert_command(input, source, target, output_dir, validate_paths)?;
        }
        Some(Commands::ConvertAll { input_dir, source, target, output_dir, validate_paths }) => {
            handle_convert_all_command(input_dir, source, target, output_dir, validate_paths)?;
        }
        #[cfg(feature = "watch-mode")]
        Some(Commands::Watch { debounce, batch_size, include_archives }) => {
            handle_watch_command(args, debounce, batch_size, include_archives)?;
        }
        #[cfg(feature = "dat-download")]
        Some(Commands::DownloadDats { output_dir, systems, force, timeout }) => {
            handle_download_dats_command(output_dir, systems, force, timeout)?;
        }
        Some(Commands::Validate { dat_dir, report, systems }) => {
            handle_validate_command(args, dat_dir, report, systems)?;
        }
        Some(Commands::Deduplicate { strategy, priority_dirs, dry_run, backup, backup_dir, report }) => {
            handle_deduplicate_command(args, strategy, priority_dirs, dry_run, backup, backup_dir, report)?;
        }
        Some(Commands::Cache { action }) => {
            handle_cache_command(action)?;
        }
        None => {
            handle_index_command(args)?;
        }
    }
    
    Ok(())
}

fn print_banner() {
    println!("{}", "🎮 RetroArch Fast Playlist Indexer v1.0".bright_cyan().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".cyan());
    println!();
}

fn handle_index_command(args: Args) -> Result<()> {
    if args.roms_dirs.is_empty() {
        eprintln!("{}", "❌ Erro: Pelo menos um diretório de ROMs deve ser especificado".red());
        eprintln!("Use: {} --roms-dir <PATH>", "retroarch-indexer".cyan());
        std::process::exit(1);
    }

    // Load or create config
    let config = Config::load_or_create(args.config.as_deref())?;
    
    // Determine platforms
    let (source_platform, target_platform) = determine_platforms(&args, &config)?;
    
    println!("📂 Escaneando: {}", format_paths(&args.roms_dirs));
    println!("🔄 Conversão: {} → {}", 
        source_platform.display_name().bright_yellow(),
        target_platform.display_name().bright_green()
    );
    println!("🧵 Threads: {}", args.threads.unwrap_or_else(num_cpus::get));
    println!();

    // Initialize scanner
    let scanner = Scanner::new()
        .with_threads(args.threads.unwrap_or_else(num_cpus::get))
        .with_recursive(!args.no_recursive)
        .with_calculate_crc(!args.no_crc)
        .with_extensions(args.extensions.as_deref())
        .with_verbose(args.verbose > 0);

    // Scan all ROM directories
    let mut all_roms = Vec::new();
    for roms_dir in &args.roms_dirs {
        println!("🔍 Escaneando diretório: {}", roms_dir.display().to_string().bright_blue());
        let roms = scanner.scan_directory(roms_dir)?;
        all_roms.extend(roms);
    }

    if all_roms.is_empty() {
        println!("{}", "⚠️  Nenhuma ROM encontrada nos diretórios especificados".yellow());
        return Ok(());
    }

    // Load DAT files if available
    let dat_collection = if let Some(dat_dir) = &args.dat_dir {
        dat_parser::DatCollection::load_directory(dat_dir)?
    } else {
        dat_parser::DatCollection::new()
    };

    // Build playlists
    let playlist_builder = PlaylistBuilder::new()
        .with_platforms(source_platform, target_platform)
        .with_dat_collection(dat_collection)
        .with_verbose(args.verbose > 0);

    // Create output directory
    std::fs::create_dir_all(&args.output_dir)?;

    // Generate individual playlists by system
    let playlists_by_system = playlist_builder.build_by_system(&all_roms)?;
    
    println!("\n📊 Sistemas Detectados:");
    let mut total_roms = 0;
    for (system_name, playlist) in &playlists_by_system {
        let count = playlist.items.len();
        total_roms += count;
        println!("├─ {}: {} ROMs", system_name.bright_white(), count.to_string().bright_green());
        
        // Save individual playlist
        let filename = format!("{}.lpl", system_name);
        let output_path = args.output_dir.join(&filename);
        playlist.save(&output_path)?;
    }

    // Generate master playlist if requested
    if !args.skip_master {
        let master_playlist = playlist_builder.build_master(&all_roms)?;
        let master_path = args.output_dir.join("roms.lpl");
        master_playlist.save(&master_path)?;
        
        println!("└─ {}: {} ROMs", 
            "Master Playlist".bright_cyan(), 
            master_playlist.items.len().to_string().bright_green()
        );
    }

    println!("\n✅ Playlists criadas em {}:", args.output_dir.display().to_string().bright_blue());
    for (system_name, _) in &playlists_by_system {
        println!("├─ {}.lpl", system_name);
    }
    if !args.skip_master {
        println!("└─ roms.lpl (playlist master com {} ROMs)", total_roms.to_string().bright_green());
    }

    // Generate report if requested
    if let Some(report_path) = args.report {
        generate_report(&all_roms, &playlists_by_system, &report_path)?;
        println!("\n📄 Relatório gerado: {}", report_path.display().to_string().bright_blue());
    }

    println!("\n{}", "🎉 Indexação concluída com sucesso!".bright_green().bold());
    
    Ok(())
}

fn handle_convert_command(
    input: PathBuf,
    source: Option<Platform>,
    target: Platform,
    output_dir: Option<PathBuf>,
    validate_paths: bool,
) -> Result<()> {
    println!("🔄 Modo Conversão de Playlist");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let converter = PlaylistConverter::new()
        .with_path_validation(validate_paths);

    // Load playlist
    println!("📄 Carregando: {}", input.display().to_string().bright_blue());
    let playlist = converter.load_playlist(&input)?;

    // Detect source platform if not specified
    let source_platform = if let Some(src) = source {
        src
    } else {
        println!("🔍 Detectando plataforma de origem...");
        converter.detect_platform(&playlist)?
    };

    println!("✅ Plataforma detectada: {}", source_platform.display_name().bright_yellow());
    
    // Convert playlist
    println!("🎯 Convertendo para: {}", target.display_name().bright_green());
    let converted_playlist = converter.convert_playlist(&playlist, source_platform, target)?;

    // Determine output path
    let output_path = if let Some(dir) = output_dir {
        std::fs::create_dir_all(&dir)?;
        let filename = input.file_stem().unwrap().to_string_lossy();
        dir.join(format!("{} [{}].lpl", filename, target.display_name()))
    } else {
        let filename = input.file_stem().unwrap().to_string_lossy();
        input.with_file_name(format!("{} [{}].lpl", filename, target.display_name()))
    };

    // Save converted playlist
    converted_playlist.save(&output_path)?;
    
    println!("✅ Playlist convertida: {}", output_path.display().to_string().bright_green());
    println!("   {} ROMs convertidas com sucesso!", converted_playlist.items.len().to_string().bright_cyan());

    Ok(())
}

fn handle_convert_all_command(
    input_dir: PathBuf,
    source: Platform,
    target: Platform,
    output_dir: Option<PathBuf>,
    validate_paths: bool,
) -> Result<()> {
    println!("🔄 Modo Conversão em Lote");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let converter = PlaylistConverter::new()
        .with_path_validation(validate_paths);

    // Find all .lpl files
    let lpl_files: Vec<_> = walkdir::WalkDir::new(&input_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "lpl"))
        .map(|e| e.path().to_path_buf())
        .collect();

    if lpl_files.is_empty() {
        println!("{}", "⚠️  Nenhum arquivo .lpl encontrado no diretório especificado".yellow());
        return Ok(());
    }

    println!("📁 Encontrados {} arquivos de playlist", lpl_files.len().to_string().bright_cyan());

    // Create output directory
    let out_dir = output_dir.unwrap_or_else(|| input_dir.join("converted"));
    std::fs::create_dir_all(&out_dir)?;

    // Progress bar
    let pb = ProgressBar::new(lpl_files.len() as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} {msg}")?
        .progress_chars("#>-"));

    let mut converted_count = 0;
    let mut error_count = 0;

    for lpl_file in lpl_files {
        let filename = lpl_file.file_stem().unwrap().to_string_lossy();
        pb.set_message(format!("Convertendo {}", filename));

        match converter.load_playlist(&lpl_file) {
            Ok(playlist) => {
                match converter.convert_playlist(&playlist, source, target) {
                    Ok(converted) => {
                        let output_path = out_dir.join(format!("{} [{}].lpl", filename, target.display_name()));
                        if let Err(e) = converted.save(&output_path) {
                            error!("Erro ao salvar {}: {}", output_path.display(), e);
                            error_count += 1;
                        } else {
                            converted_count += 1;
                        }
                    }
                    Err(e) => {
                        error!("Erro ao converter {}: {}", lpl_file.display(), e);
                        error_count += 1;
                    }
                }
            }
            Err(e) => {
                error!("Erro ao carregar {}: {}", lpl_file.display(), e);
                error_count += 1;
            }
        }

        pb.inc(1);
    }

    pb.finish_with_message("Conversão concluída");

    println!("\n✅ Conversão em lote concluída:");
    println!("   {} playlists convertidas com sucesso", converted_count.to_string().bright_green());
    if error_count > 0 {
        println!("   {} erros encontrados", error_count.to_string().bright_red());
    }
    println!("   Arquivos salvos em: {}", out_dir.display().to_string().bright_blue());

    Ok(())
}

#[cfg(feature = "watch-mode")]
fn handle_watch_command(
    args: Args,
    debounce: u64,
    batch_size: usize,
    include_archives: bool,
) -> Result<()> {
    println!("👀 Modo Watch Ativado");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    if args.roms_dirs.is_empty() {
        eprintln!("{}", "❌ Erro: Pelo menos um diretório de ROMs deve ser especificado".red());
        std::process::exit(1);
    }

    let config = Config::load_or_create(args.config.as_deref())?;
    let (source_platform, target_platform) = determine_platforms(&args, &config)?;

    let mut watch_service = WatchService::new()
        .with_debounce_duration(std::time::Duration::from_millis(debounce))
        .with_batch_size(batch_size)
        .with_include_archives(include_archives)
        .with_platforms(source_platform, target_platform)
        .with_output_dir(args.output_dir.clone());

    // Add watch directories
    for dir in &args.roms_dirs {
        watch_service.add_watch_directory(dir.clone())?;
        println!("👁️  Monitorando: {}", dir.display().to_string().bright_blue());
    }

    println!("\n🔧 Configurações:");
    println!("├─ Debounce: {}ms", debounce);
    println!("├─ Batch Size: {} arquivos", batch_size);
    println!("├─ Incluir Archives: {}", if include_archives { "Sim" } else { "Não" });
    println!("└─ Output: {}", args.output_dir.display());

    println!("\n{}", "✅ Watch ativo! Pressione Ctrl+C para parar...".bright_green());
    
    // Start watching
    watch_service.start_watching()?;

    Ok(())
}

#[cfg(feature = "dat-download")]
fn handle_download_dats_command(
    output_dir: PathBuf,
    systems: Option<Vec<String>>,
    force: bool,
    timeout: u64,
) -> Result<()> {
    println!("📥 Download Automático de DATs");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    std::fs::create_dir_all(&output_dir)?;

    let downloader = DatDownloader::new()
        .with_timeout(std::time::Duration::from_secs(timeout))
        .with_output_directory(output_dir.clone());

    let systems_to_download = if let Some(systems) = systems {
        systems
    } else {
        println!("📋 Obtendo lista de sistemas disponíveis...");
        downloader.get_available_systems()?
    };

    println!("🎯 Sistemas para download: {}", systems_to_download.join(", "));
    println!("📁 Diretório de destino: {}", output_dir.display());
    println!("🔄 Forçar re-download: {}", if force { "Sim" } else { "Não" });
    println!();

    let pb = ProgressBar::new(systems_to_download.len() as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} {msg}")?
        .progress_chars("#>-"));

    let mut success_count = 0;
    let mut error_count = 0;

    for system in &systems_to_download {
        pb.set_message(format!("Baixando {}", system));
        
        match downloader.download_dat_for_system(system, force) {
            Ok(path) => {
                println!("✅ {}: {}", system, path.display());
                success_count += 1;
            }
            Err(e) => {
                eprintln!("❌ {}: {}", system, e);
                error_count += 1;
            }
        }
        
        pb.inc(1);
    }

    pb.finish_with_message("Download concluído");

    println!("\n📊 Resultado do Download:");
    println!("├─ ✅ Sucessos: {}", success_count.to_string().bright_green());
    if error_count > 0 {
        println!("├─ ❌ Erros: {}", error_count.to_string().bright_red());
    }
    println!("└─ 📁 Salvos em: {}", output_dir.display().to_string().bright_blue());

    Ok(())
}

fn handle_validate_command(
    args: Args,
    dat_dir: PathBuf,
    report: Option<PathBuf>,
    systems: Option<Vec<String>>,
) -> Result<()> {
    println!("🔍 Validação de Integridade");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    if args.roms_dirs.is_empty() {
        eprintln!("{}", "❌ Erro: Pelo menos um diretório de ROMs deve ser especificado".red());
        std::process::exit(1);
    }

    // Load validator
    let mut validator = RomValidator::new();

    // Load DAT files
    println!("📚 Carregando arquivos DAT de: {}", dat_dir.display());
    if let Some(systems) = &systems {
        for system in systems {
            let dat_path = dat_dir.join(format!("{}.dat", system));
            if dat_path.exists() {
                validator.load_dat_collection(system, &dat_path)?;
                println!("✅ Carregado: {}", system);
            } else {
                println!("⚠️  DAT não encontrado: {}", dat_path.display());
            }
        }
    } else {
        // Load all DAT files in directory
        for entry in std::fs::read_dir(&dat_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "dat") {
                let system = path.file_stem().unwrap().to_string_lossy();
                validator.load_dat_collection(&system, &path)?;
                println!("✅ Carregado: {}", system);
            }
        }
    }

    // Scan ROMs
    let scanner = Scanner::new()
        .with_threads(args.threads.unwrap_or_else(num_cpus::get))
        .with_recursive(!args.no_recursive)
        .with_calculate_crc(true); // Always calculate CRC for validation

    let mut all_roms = Vec::new();
    for roms_dir in &args.roms_dirs {
        println!("🔍 Escaneando: {}", roms_dir.display());
        let roms = scanner.scan_directory(roms_dir)?;
        all_roms.extend(roms);
    }

    if all_roms.is_empty() {
        println!("{}", "⚠️  Nenhuma ROM encontrada".yellow());
        return Ok(());
    }

    // Validate collection
    println!("🔍 Validando {} ROMs...", all_roms.len());
    let validation_report = validator.validate_collection(&all_roms)?;

    // Print summary
    println!("\n📊 Resultado da Validação:");
    println!("├─ Total: {}", validation_report.total_roms);
    println!("├─ ✅ Válidas: {} ({:.1}%)", 
        validation_report.valid_roms, 
        (validation_report.valid_roms as f64 / validation_report.total_roms as f64) * 100.0);
    println!("├─ 🔄 Precisam Renomear: {}", validation_report.renamed_roms);
    println!("├─ ❓ Desconhecidas: {}", validation_report.unknown_roms);
    println!("├─ 🏠 Homebrew/Hack: {}", validation_report.homebrew_roms);
    println!("├─ ❌ Bad Dumps: {}", validation_report.bad_dumps);
    println!("└─ 💥 Corrompidas: {}", validation_report.corrupted_roms);

    // Generate detailed report if requested
    if let Some(report_path) = report {
        validator.generate_report(&all_roms, &report_path)?;
        println!("\n📄 Relatório detalhado salvo em: {}", report_path.display().to_string().bright_blue());
    }

    Ok(())
}

fn handle_deduplicate_command(
    args: Args,
    strategy: DeduplicationStrategy,
    priority_dirs: Option<Vec<PathBuf>>,
    dry_run: bool,
    backup: bool,
    backup_dir: Option<PathBuf>,
    report: Option<PathBuf>,
) -> Result<()> {
    println!("🗂️  Deduplicação Inteligente");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    if args.roms_dirs.is_empty() {
        eprintln!("{}", "❌ Erro: Pelo menos um diretório de ROMs deve ser especificado".red());
        std::process::exit(1);
    }

    // Convert strategy enum to deduplicator strategy
    let dedup_strategy = match strategy {
        DeduplicationStrategy::RegionPriority => deduplicator::Strategy::RegionPriority,
        DeduplicationStrategy::FileSize => deduplicator::Strategy::FileSize,
        DeduplicationStrategy::ModificationDate => deduplicator::Strategy::ModificationDate,
        DeduplicationStrategy::DirectoryPriority => deduplicator::Strategy::DirectoryPriority,
        DeduplicationStrategy::FilenameQuality => deduplicator::Strategy::FilenameQuality,
    };

    let mut deduplicator = RomDeduplicator::new()
        .with_strategy(dedup_strategy)
        .with_dry_run(dry_run)
        .with_backup(backup);

    if let Some(backup_dir) = backup_dir {
        deduplicator = deduplicator.with_backup_directory(backup_dir);
    }

    if let Some(priority_dirs) = priority_dirs {
        deduplicator = deduplicator.with_priority_directories(priority_dirs);
    }

    // Scan for ROMs
    let scanner = Scanner::new()
        .with_threads(args.threads.unwrap_or_else(num_cpus::get))
        .with_recursive(!args.no_recursive)
        .with_calculate_crc(true); // Need CRC for duplicate detection

    let mut all_roms = Vec::new();
    for roms_dir in &args.roms_dirs {
        println!("🔍 Escaneando: {}", roms_dir.display());
        let roms = scanner.scan_directory(roms_dir)?;
        all_roms.extend(roms);
    }

    if all_roms.is_empty() {
        println!("{}", "⚠️  Nenhuma ROM encontrada".yellow());
        return Ok(());
    }

    println!("🎯 Estratégia: {:?}", strategy);
    println!("🔍 Total de ROMs: {}", all_roms.len());
    if dry_run {
        println!("🚫 Modo simulação ativado - nenhum arquivo será removido");
    }
    println!();

    // Find and remove duplicates
    let dedup_report = deduplicator.deduplicate(&all_roms)?;

    // Print results
    println!("📊 Resultado da Deduplicação:");
    println!("├─ Grupos de duplicatas: {}", dedup_report.duplicate_groups);
    println!("├─ ROMs duplicadas encontradas: {}", dedup_report.duplicates_found);
    println!("├─ ROMs removidas: {}", dedup_report.files_removed);
    println!("├─ Espaço liberado: {}", format_size(dedup_report.space_freed));
    if backup && dedup_report.files_removed > 0 {
        println!("├─ Backup criado em: {}", dedup_report.backup_location.as_ref().unwrap_or(&"N/A".to_string()));
    }
    println!("└─ ROMs únicas restantes: {}", all_roms.len() - dedup_report.files_removed);

    // Save detailed report if requested
    if let Some(report_path) = report {
        deduplicator.generate_report(&dedup_report, &report_path)?;
        println!("\n📄 Relatório detalhado salvo em: {}", report_path.display().to_string().bright_blue());
    }

    if !dry_run && dedup_report.files_removed > 0 {
        println!("\n{}", "✅ Deduplicação concluída com sucesso!".bright_green());
    }

    Ok(())
}

fn handle_cache_command(action: CacheAction) -> Result<()> {
    println!("🗄️  Gerenciamento de Cache");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let cache = CrcCache::new()?;

    match action {
        CacheAction::Clear => {
            cache.clear()?;
            println!("✅ Cache limpo com sucesso!");
        }
        CacheAction::Stats => {
            let stats = cache.get_stats()?;
            println!("📊 Estatísticas do Cache:");
            println!("├─ Total de entradas: {}", stats.total_entries);
            println!("├─ Tamanho do cache: {}", format_size(stats.cache_size));
            println!("├─ Hits: {} ({:.1}%)", stats.cache_hits, 
                if stats.total_requests > 0 { 
                    (stats.cache_hits as f64 / stats.total_requests as f64) * 100.0 
                } else { 0.0 });
            println!("├─ Misses: {}", stats.cache_misses);
            println!("└─ Última atualização: {}", stats.last_updated);
        }
        CacheAction::Clean { max_age } => {
            let removed = cache.clean_old_entries(max_age)?;
            println!("✅ {} entradas antigas removidas (mais de {} dias)", removed, max_age);
        }
    }

    Ok(())
}

fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.1} {}", size, UNITS[unit_index])
}

fn determine_platforms(args: &Args, config: &Config) -> Result<(Platform, Platform)> {
    let source = args.source_platform
        .or(config.general.source_platform)
        .unwrap_or_else(|| {
            // Interactive platform selection for source
            select_platform("origem (onde está executando)", &[
                Platform::Windows,
                Platform::Linux,
                Platform::MacOS,
                Platform::SteamDeck,
                Platform::Raspberry,
            ])
        });

    let target = args.target_platform
        .or(config.general.target_platform)
        .unwrap_or_else(|| {
            // Interactive platform selection for target
            select_platform("destino (onde será usado)", &[
                Platform::Windows,
                Platform::Linux,
                Platform::MacOS,
                Platform::Android,
                Platform::Switch,
                Platform::Raspberry,
                Platform::SteamDeck,
            ])
        });

    Ok((source, target))
}

fn select_platform(description: &str, platforms: &[Platform]) -> Platform {
    println!("📍 Plataforma de {}:", description);
    for (i, platform) in platforms.iter().enumerate() {
        println!("{}) {}", i + 1, platform.display_name());
    }
    
    loop {
        print!("\nSelecione [1-{}]: ", platforms.len());
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        
        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_ok() {
            if let Ok(choice) = input.trim().parse::<usize>() {
                if choice > 0 && choice <= platforms.len() {
                    return platforms[choice - 1];
                }
            }
        }
        println!("{}", "❌ Seleção inválida. Tente novamente.".red());
    }
}

fn format_paths(paths: &[PathBuf]) -> String {
    if paths.len() == 1 {
        paths[0].display().to_string()
    } else {
        format!("{} diretórios", paths.len())
    }
}

fn generate_report(
    _all_roms: &[scanner::RomFile], 
    _playlists: &std::collections::HashMap<String, playlist::Playlist>,
    report_path: &PathBuf
) -> Result<()> {
    // TODO: Implement detailed report generation
    std::fs::write(report_path, "# Relatório de Indexação\n\nRelatório em desenvolvimento...")?;
    Ok(())
}
