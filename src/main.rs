use anyhow::Result;
use clap::Parser;
use colored::*;
use dialoguer::{Input, Select, Confirm};
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
mod thread_monitor;
mod i18n;
mod mame_xml;
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

#[derive(Debug, Clone)]
enum ExecutionMode {
    Interactive,
    Automatic,
}

#[derive(Debug, Clone)]
struct ConsoleConfig {
    system_name: String,
    roms_dir: PathBuf,
    output_dir: PathBuf,
    force_system: bool, // Force all ROMs to be of the selected system
}

fn main() -> Result<()> {
    env_logger::init();
    
    // Initialize i18n system
    if let Err(e) = i18n::init() {
        eprintln!("{}", i18n::t_with_arg("initialization-warning", &e.to_string()));
    }
    
    let mut args = Args::parse();
    
    // Set language if specified
    if let Some(ref language) = args.language {
        i18n::set_locale(language);
    }
    
    // Print banner
    print_banner();
    
    match &args.command {
        Some(Commands::Convert { input, source, target, output_dir, validate_paths }) => {
            handle_convert_command(input.clone(), *source, *target, output_dir.clone(), *validate_paths)?;
        }
        Some(Commands::ConvertAll { input_dir, source, target, output_dir, validate_paths }) => {
            handle_convert_all_command(input_dir.clone(), *source, *target, output_dir.clone(), *validate_paths)?;
        }
        #[cfg(feature = "watch-mode")]
        Some(Commands::Watch { debounce, batch_size, include_archives }) => {
            handle_watch_command(args.clone(), *debounce, *batch_size, *include_archives)?;
        }
        #[cfg(feature = "dat-download")]
        Some(Commands::DownloadDats { output_dir, systems, force, timeout }) => {
            handle_download_dats_command(output_dir.clone(), systems.clone(), *force, *timeout)?;
        }
        Some(Commands::Validate { dat_dir, report, systems }) => {
            handle_validate_command(args.clone(), dat_dir.clone(), report.clone(), systems.clone())?;
        }
        Some(Commands::Deduplicate { strategy, priority_dirs, dry_run, backup, backup_dir, report }) => {
            handle_deduplicate_command(args.clone(), *strategy, priority_dirs.clone(), *dry_run, *backup, backup_dir.clone(), report.clone())?;
        }
        Some(Commands::Cache { action }) => {
            handle_cache_command(action.clone())?;
        }
        None => {
            // Ask which mode the user wants to use
            let mode = prompt_for_execution_mode()?;
            match mode {
                ExecutionMode::Interactive => {
                    handle_interactive_console_selection(&mut args)?;
                }
                ExecutionMode::Automatic => {
                    handle_index_command_with_prompts(&mut args)?;
                }
            }
        }
    }
    
    Ok(())
}

fn print_banner() {
    println!("{}", i18n::t("app-header").bright_cyan().bold());
    println!("{}", i18n::t("app-separator").cyan());
    println!();
}

/// Interactive prompt to get ROM directories from user
fn prompt_for_roms_dirs() -> Result<Vec<PathBuf>> {
    println!("{}", i18n::t("rom-directories-config").bright_cyan().bold());
    println!();
    
    let mut roms_dirs = Vec::new();
    
    loop {
        let prompt_text = if roms_dirs.is_empty() {
            i18n::t("roms-dir-prompt")
        } else {
            i18n::t("roms-dir-prompt-additional")
        };
        
        let input = Input::<String>::new()
            .with_prompt(prompt_text)
            .allow_empty(!roms_dirs.is_empty())
            .interact()?;
        
        if input.trim().is_empty() && !roms_dirs.is_empty() {
            break;
        }
        
        let path = PathBuf::from(input.trim());
        
        if !path.exists() {
            println!("{}", i18n::t_path("directory-not-found", &path.display().to_string()).yellow());
            continue;
        }
        
        if !path.is_dir() {
            println!("{}", i18n::t_path("not-a-directory", &path.display().to_string()).yellow());
            continue;
        }
        
        roms_dirs.push(path.clone());
        println!("{}", i18n::t_path("directory-added", &path.display().to_string()).green());
        
        if roms_dirs.len() >= 10 {
            println!("{}", i18n::t("max-directories-reached").yellow());
            break;
        }
    }
    
    Ok(roms_dirs)
}

/// Interactive prompt to get platforms
fn prompt_for_platforms() -> Result<(Platform, Platform)> {
    println!("{}", i18n::t("platforms-configuration").bright_cyan().bold());
    println!();
    
    let platforms = vec![
        Platform::Windows,
        Platform::Linux,
        Platform::MacOS,
        Platform::Switch,
        Platform::Android,
        Platform::SteamDeck,
        Platform::Raspberry,
    ];
    
    let platform_names: Vec<&str> = platforms.iter()
        .map(|p| p.display_name())
        .collect();
    
    println!("{}", i18n::t_with_arg("select-source-platform", &i18n::t("source")));
    let source_index = Select::new()
        .items(&platform_names)
        .default(0)
        .interact()?;
    
    println!();
    println!("{}", i18n::t_with_arg("select-target-platform", &i18n::t("target")));
    let target_index = Select::new()
        .items(&platform_names)
        .default(if source_index == 3 { 0 } else { 3 }) // Switch by default if not source
        .interact()?;
    
    Ok((platforms[source_index], platforms[target_index]))
}

/// Interactive prompt to get output directory
fn prompt_for_output_dir() -> Result<PathBuf> {
    println!("{}", i18n::t("output-directory-config").bright_cyan().bold());
    println!();
    
    let default_output = "./playlists";
    
    let output = Input::<String>::new()
        .with_prompt(i18n::t("output-dir-prompt"))
        .default(default_output.to_string())
        .interact()?;
    
    let output_path = PathBuf::from(output.trim());
    
    // Create directory if it doesn't exist
    if !output_path.exists() {
        let create = Confirm::new()
            .with_prompt(i18n::t_with_arg("create-directory-prompt", &output_path.display().to_string()))
            .default(true)
            .interact()?;
        
        if create {
            std::fs::create_dir_all(&output_path)?;
            println!("{}", i18n::t_path("directory-created", &output_path.display().to_string()).green());
        }
    }
    
    Ok(output_path)
}

fn handle_index_command(args: Args) -> Result<()> {
    if args.roms_dirs.is_empty() {
        eprintln!("{}", i18n::t("error-roms-dir-required").red());
        eprintln!("{}", i18n::t_with_arg("usage-instruction", "retroarch-indexer").cyan());
        std::process::exit(1);
    }

    // Load or create config
    let config = Config::load_or_create(args.config.as_deref())?;
    
    // Determine platforms
    let (source_platform, target_platform) = determine_platforms(&args, &config)?;
    
    println!("{}", i18n::t_path("scanning-directory", &format_paths(&args.roms_dirs)));
    println!("{}", i18n::t_conversion(&source_platform.display_name(), &target_platform.display_name()));
    println!("{}", i18n::t_count("threads-info", args.threads.unwrap_or_else(num_cpus::get) as i32));
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
    let mut total_scanned_files = 0;
    let mut archives_found = 0;
    
    for (index, roms_dir) in args.roms_dirs.iter().enumerate() {
        println!("{}", i18n::t_directory_progress(
            index + 1, 
            args.roms_dirs.len(), 
            &roms_dir.display().to_string()
        ).bright_yellow());
        
        let start_time = std::time::Instant::now();
        let roms = scanner.scan_directory(roms_dir)?;
        let scan_duration = start_time.elapsed();
        
        // Count archives
        let dir_archives = roms.iter().filter(|rom| rom.is_archive).count();
        archives_found += dir_archives;
        
        total_scanned_files += roms.len();
        
        if args.verbose > 0 {
            let mut args_map = std::collections::HashMap::new();
            args_map.insert("count".to_string(), roms.len().to_string());
            args_map.insert("time".to_string(), format!("{:.2}", scan_duration.as_secs_f32()));
            println!("   {}", i18n::t_with_args("roms-found-summary", &args_map).bright_green());
            if dir_archives > 0 {
                let mut args_map = std::collections::HashMap::new();
                args_map.insert("count".to_string(), dir_archives.to_string());
                println!("   {}", i18n::t_with_args("archives-detected", &args_map).bright_magenta());
            }
        }
        
        all_roms.extend(roms);
    }

    if all_roms.is_empty() {
        println!("{}", i18n::t("no-roms-found").yellow());
        return Ok(());
    }

    // Show scanning summary
    println!("\n📈 {} do Escaneamento:", "Resumo".bright_cyan().bold());
    println!("{}", i18n::t_count("total-roms-stat", total_scanned_files as i32).bright_green());
    println!("{}", i18n::t_conversion(&source_platform.display_name(), &target_platform.display_name()).bright_yellow());
    println!("🧵 Threads: {}", args.threads.unwrap_or_else(num_cpus::get));
    println!();

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
        println!("{}", i18n::t_count("master-playlist-info", total_roms as i32).replace(&total_roms.to_string(), &total_roms.to_string().bright_green().to_string()));
    }

    // Generate report if requested
    if let Some(report_path) = args.report {
        generate_report(&all_roms, &playlists_by_system, &report_path)?;
        println!("\n{}", i18n::t_path("report-generated", &report_path.display().to_string()).bright_blue());
    }

    println!("\n{}", i18n::t("indexing-complete").bright_green().bold());
    
    Ok(())
}

fn handle_convert_command(
    input: PathBuf,
    source: Option<Platform>,
    target: Platform,
    output_dir: Option<PathBuf>,
    validate_paths: bool,
) -> Result<()> {
    println!("{}", i18n::t("playlist-conversion-mode"));
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
    println!("{}", i18n::t_with_arg("converting-to", &target.display_name().to_string()).bright_green());
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
    println!("{}", i18n::t("batch-conversion-mode"));
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
        println!("{}", i18n::t("no-lpl-files-found").yellow());
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

    pb.finish_with_message("Conversion completed");

    println!("\n{}", i18n::t("batch-conversion-complete"));
    println!("   {} playlists convertidas com sucesso", converted_count.to_string().bright_green());
    if error_count > 0 {
        println!("{}", i18n::t_count("errors-found", error_count));
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
    println!("{}", i18n::t("watch-mode-active"));
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    if args.roms_dirs.is_empty() {
        eprintln!("{}", i18n::t("error-roms-dir-required").red());
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

    println!("\n{}", i18n::t("settings"));
    println!("├─ Debounce: {}ms", debounce);
    println!("├─ Batch Size: {} arquivos", batch_size);
    println!("{}", i18n::t_with_arg("include-archives", if include_archives { &i18n::t("yes") } else { &i18n::t("no") }));
    println!("└─ Output: {}", args.output_dir.display());

    println!("\n{}", i18n::t("watch-active-press-ctrl-c").bright_green());
    
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
    println!("{}", i18n::t("dat-download"));
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    std::fs::create_dir_all(&output_dir)?;

    let downloader = DatDownloader::new_default()?
        .with_timeout(std::time::Duration::from_secs(timeout))
        .with_output_directory(output_dir.clone());

    let systems_to_download = if let Some(systems) = systems {
        systems
    } else {
        println!("{}", i18n::t("getting-systems-list"));
        downloader.get_available_systems()?
    };

    println!("{}", i18n::t_with_arg("systems-for-download", &systems_to_download.join(", ")));
    println!("{}", i18n::t_path("destination-directory", &output_dir.display().to_string()));
    println!("{}", i18n::t_with_arg("force-redownload", if force { &i18n::t("yes") } else { &i18n::t("no") }));
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
                let mut args = std::collections::HashMap::new();
                args.insert("system".to_string(), system.to_string());
                args.insert("error".to_string(), e.to_string());
                eprintln!("{}", i18n::t_with_args("error-processing-failed", &args).red());
                error_count += 1;
            }
        }
        
        pb.inc(1);
    }

    pb.finish_with_message("Download completed");

    println!("\n📊 Resultado do Download:");
    println!("├─ ✅ Sucessos: {}", success_count.to_string().bright_green());
    if error_count > 0 {
        println!("{}", i18n::t_count("errors-count", error_count));
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
    println!("{}", i18n::t("integrity-validation-title"));
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    if args.roms_dirs.is_empty() {
        eprintln!("{}", i18n::t("error-roms-dir-required").red());
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
                println!("{}", i18n::t_path("dat-not-found-warning", &dat_path.display().to_string()));
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
    println!("{}", i18n::t("validation-results-title"));
    println!("{}", i18n::t_count("validation-total", validation_report.total_roms as i32));
    println!("{}", i18n::t_validation_valid(
        validation_report.valid_roms as i32,
        validation_report.valid_roms as f64 / validation_report.total_roms as f64 * 100.0
    ));
    println!("{}", i18n::t_count("validation-need-rename", validation_report.renamed_roms as i32));
    println!("{}", i18n::t_count("validation-unknown", validation_report.unknown_roms as i32));
    println!("{}", i18n::t_count("validation-homebrew", validation_report.homebrew_roms as i32));
    println!("{}", i18n::t_count("validation-bad-dumps", validation_report.bad_dumps as i32));
    println!("{}", i18n::t_count("validation-corrupted", validation_report.corrupted_roms as i32));

    // Generate detailed report if requested
    if let Some(report_path) = report {
        validator.generate_report(&all_roms, &report_path)?;
        println!("\n📄 Detailed report saved to: {}", report_path.display().to_string().bright_blue());
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
    println!("{}", i18n::t("intelligent-deduplication"));
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    if args.roms_dirs.is_empty() {
        eprintln!("{}", i18n::t("error-roms-dir-required").red());
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

    println!("🎯 Strategy: {:?}", strategy);
    println!("{}", i18n::t_count("total-roms-found", all_roms.len() as i32));
    if dry_run {
        println!("{}", i18n::t("simulation-mode-active"));
    }
    println!();

    // Find and remove duplicates
    let dedup_report = deduplicator.deduplicate(&all_roms)?;

    // Print results
    println!("{}", i18n::t("deduplication-results"));
    println!("├─ Grupos de duplicatas: {}", dedup_report.duplicate_groups);
    println!("├─ Duplicate ROMs found: {}", dedup_report.duplicates_found);
    println!("├─ ROMs removidas: {}", dedup_report.files_removed);
    println!("├─ Space freed: {}", format_size(dedup_report.space_freed));
    if backup && dedup_report.files_removed > 0 {
        println!("├─ Backup criado em: {}", dedup_report.backup_location.as_ref().unwrap_or(&"N/A".to_string()));
    }
    println!("└─ Unique ROMs remaining: {}", all_roms.len() - dedup_report.files_removed);

    // Save detailed report if requested
    if let Some(report_path) = report {
        deduplicator.generate_report(&dedup_report, &report_path)?;
        println!("\n📄 Detailed report saved to: {}", report_path.display().to_string().bright_blue());
    }

    if !dry_run && dedup_report.files_removed > 0 {
        println!("\n{}", i18n::t("deduplication-complete").bright_green());
    }

    Ok(())
}

fn handle_cache_command(action: CacheAction) -> Result<()> {
    println!("🗄️  Gerenciamento de Cache");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let mut cache = CrcCache::with_default_location()?;

    match action {
        CacheAction::Clear => {
            cache.clear()?;
            println!("✅ Cache limpo com sucesso!");
        }
        CacheAction::Stats => {
            let stats = cache.get_stats()?;
            println!("📊 Cache Statistics:");
            println!("{}", i18n::t_count("total-cache-entries", stats.total_entries as i32));
            println!("├─ Tamanho do cache: {}", format_size(stats.cache_size));
            println!("├─ Hits: {} ({:.1}%)", stats.cache_hits, 
                if stats.total_requests > 0 { 
                    (stats.cache_hits as f64 / stats.total_requests as f64) * 100.0 
                } else { 0.0 });
            println!("├─ Misses: {}", stats.cache_misses);
            println!("└─ Last updated: {}", stats.last_updated);
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
            select_platform("source (where it's running)", &[
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
            select_platform("target (where it will be used)", &[
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
        println!("{}", "❌ Invalid selection. Try again.".red());
    }
}

fn format_paths(paths: &[PathBuf]) -> String {
    if paths.len() == 1 {
        paths[0].display().to_string()
    } else {
        i18n::t_with_arg("directories-count", &paths.len().to_string())
    }
}

fn generate_report(
    _all_roms: &[scanner::RomFile], 
    _playlists: &std::collections::HashMap<String, playlist::Playlist>,
    report_path: &PathBuf
) -> Result<()> {
    // TODO: Implement detailed report generation
    std::fs::write(report_path, "# Indexing Report\n\nReport in development...")?;
    Ok(())
}

/// Main function to handle indexing with interactive prompts
fn handle_index_command_with_prompts(args: &mut Args) -> Result<()> {
    // If no ROM directories were provided via CLI, ask interactively
    if args.roms_dirs.is_empty() {
        match prompt_for_roms_dirs() {
            Ok(dirs) => args.roms_dirs = dirs,
            Err(e) => {
                eprintln!("{}", i18n::t_with_arg("error-getting-roms-dirs", &e.to_string()).red());
                std::process::exit(1);
            }
        }
    }
    
    // If no platforms were provided via CLI, ask interactively
    if args.source_platform.is_none() || args.target_platform.is_none() {
        match prompt_for_platforms() {
            Ok((source, target)) => {
                if args.source_platform.is_none() {
                    args.source_platform = Some(source);
                }
                if args.target_platform.is_none() {
                    args.target_platform = Some(target);
                }
            }
            Err(e) => {
                eprintln!("{}", i18n::t_with_arg("error-getting-platforms", &e.to_string()).red());
                std::process::exit(1);
            }
        }
    }
    
    // If output directory is default, ask if user wants to change it
    if args.output_dir == PathBuf::from("./playlists") {
        let use_default = Confirm::new()
            .with_prompt(&i18n::t("use-default-output-dir"))
            .default(true)
            .interact()?;
        
        if !use_default {
            match prompt_for_output_dir() {
                Ok(output_dir) => args.output_dir = output_dir,
                Err(e) => {
                    eprintln!("{}", i18n::t_with_arg("error-getting-output-dir", &e.to_string()).red());
                    std::process::exit(1);
                }
            }
        }
    }
    
    // Agora executar o comando index normal
    handle_index_command(args.clone())
}

/// Prompt to choose execution mode
fn prompt_for_execution_mode() -> Result<ExecutionMode> {
    println!("{}", i18n::t("execution-mode").bright_cyan().bold());
    println!();
    
    let modes = vec![
        i18n::t("interactive-mode-console-selection"),
        i18n::t("automatic-mode-scan-all")
    ];
    
    let mode_descriptions = vec![
        i18n::t("interactive-mode-desc"),
        i18n::t("automatic-mode-desc")
    ];
    
    println!("{}", i18n::t("choose-indexer-execution").bright_white());
    for (i, (mode, desc)) in modes.iter().zip(mode_descriptions.iter()).enumerate() {
        println!("  {}. {} - {}", (i + 1).to_string().bright_yellow(), mode.bright_green(), desc.bright_black());
    }
    println!();
    
    let selection = Select::new()
        .with_prompt(&i18n::t("select-mode"))
        .items(&modes)
        .default(0)
        .interact()?;
    
    match selection {
        0 => Ok(ExecutionMode::Interactive),
        1 => Ok(ExecutionMode::Automatic),
        _ => Ok(ExecutionMode::Automatic),
    }
}

/// Function to handle interactive console selection
fn handle_interactive_console_selection(args: &mut Args) -> Result<()> {
    println!("{}", i18n::t("console-cores-selection").bright_cyan().bold());
    println!();
    
    // Get list of available systems
    let core_mapper = core_mapper::CoreMapper::new();
    let available_systems = get_available_sistemas(&core_mapper);
    
    if available_systems.is_empty() {
        eprintln!("{}", i18n::t("no-available-systems").red());
        std::process::exit(1);
    }
    
    // Prompt for system selection
    let selected_systems = prompt_for_system_selection(&available_systems)?;
    
    if selected_systems.is_empty() {
        println!("{}", i18n::t("no-system-selected").yellow());
        return Ok(());
    }
    
    // For each selected system, configure directories
    let mut console_configs = Vec::new();
    for system_name in selected_systems {
        let config = prompt_for_console_config(&system_name)?;
        console_configs.push(config);
    }
    
    // Configure platforms if not specified
    if args.source_platform.is_none() || args.target_platform.is_none() {
        match prompt_for_platforms() {
            Ok((source, target)) => {
                if args.source_platform.is_none() {
                    args.source_platform = Some(source);
                }
                if args.target_platform.is_none() {
                    args.target_platform = Some(target);
                }
            }
            Err(e) => {
                eprintln!("{}", i18n::t_with_arg("error-getting-platforms", &e.to_string()).red());
                std::process::exit(1);
            }
        }
    }
    
    // Process each configured console
    println!("\n{} Processando {} consoles...", "🔄".bright_blue(), console_configs.len());
    
    // Determine if we should process consoles in parallel or sequentially
    let total_threads = args.threads.unwrap_or_else(num_cpus::get);
    let consoles_count = console_configs.len();
    
    if consoles_count == 1 {
        // Single console: use all available threads
        let config = &console_configs[0];
        println!("\n{} Processando: {} (usando {} threads)", "🔄".bright_blue(), config.system_name.bright_green(), total_threads);
        process_single_console_config(args, config)?;
    } else if consoles_count <= 4 && total_threads >= 8 {
        // Multiple consoles with sufficient threads: process in parallel
        println!("\n{} Processamento paralelo ativado para {} consoles", "⚡".bright_yellow(), consoles_count);
        process_consoles_parallel(args, console_configs)?;
    } else {
        // Multiple consoles but limited threads: process sequentially  
        println!("\n{} Processamento sequencial para {} consoles", "🔄".bright_blue(), consoles_count);
        process_consoles_sequential(args, console_configs)?;
    }
    
    println!("\n{}", i18n::t("processing-all-consoles-complete").bright_green().bold());
    Ok(())
}

/// Get list of available systems from core mapper
fn get_available_sistemas(_core_mapper: &core_mapper::CoreMapper) -> Vec<String> {
    // Since get_supported_systems() is not available, let's create manually
    vec![
        "Nintendo - Nintendo 64".to_string(),
        "Nintendo - Super Nintendo Entertainment System".to_string(),
        "Nintendo - Game Boy Advance".to_string(),
        "Nintendo - Game Boy Color".to_string(),
        "Nintendo - Game Boy".to_string(),
        "Nintendo - Nintendo Entertainment System".to_string(),
        "Sega - Mega Drive - Genesis".to_string(),
        "Sega - 32X".to_string(),
        "Sega - Master System - Mark III".to_string(),
        "Sega - Game Gear".to_string(),
        "Sony - PlayStation".to_string(),
        "MAME".to_string(),
        "Arcade".to_string(),
    ]
}

/// Prompt for system selection
fn prompt_for_system_selection(available_systems: &[String]) -> Result<Vec<String>> {
    println!("{}", i18n::t("available-systems-consoles").bright_white());
    
    // Group systems by manufacturer for better visualization
    let mut grouped_systems: std::collections::HashMap<String, Vec<&String>> = std::collections::HashMap::new();
    
    for system in available_systems {
        let manufacturer = if system.starts_with("Nintendo") {
            "Nintendo"
        } else if system.starts_with("Sega") {
            "Sega"
        } else if system.starts_with("Sony") {
            "Sony"
        } else {
            "Arcade/Outros"
        };
        
        grouped_systems.entry(manufacturer.to_string())
            .or_insert_with(Vec::new)
            .push(system);
    }
    
    // Mostrar sistemas agrupados
    for (manufacturer, systems) in &grouped_systems {
        println!("\n  {}:", manufacturer.bright_cyan().bold());
        for system in systems {
            println!("    • {}", system.bright_white());
        }
    }
    
    println!();
    
    let mut selected_systems = Vec::new();
    
    loop {
        let selection = Select::new()
            .with_prompt(&i18n::t("select-system-or-finish"))
            .items(&{
                let mut items = available_systems.to_vec();
                items.push(i18n::t("finish-selection"));
                items
            })
            .interact()?;
        
        if selection == available_systems.len() {
            // "Finish selection" was chosen
            break;
        }
        
        let selected_system = &available_systems[selection];
        
        if selected_systems.contains(selected_system) {
            println!("  {}", i18n::t_with_arg("system-already-selected", &selected_system).yellow());
        } else {
            selected_systems.push(selected_system.clone());
            println!("  {}", i18n::t_with_arg("system-added", &selected_system).green());
        }
        
        println!("\n{}", i18n::t_with_arg("systems-selected-so-far", &selected_systems.len().to_string()).bright_blue());
        for system in &selected_systems {
            println!("  • {}", system.bright_green());
        }
        println!();
        
        let continue_selection = Confirm::new()
            .with_prompt("Selecionar mais sistemas?")
            .default(true)
            .interact()?;
        
        if !continue_selection {
            break;
        }
    }
    
    Ok(selected_systems)
}

/// Prompt for specific console configuration
fn prompt_for_console_config(system_name: &str) -> Result<ConsoleConfig> {
    println!("\n{}", i18n::t_with_arg("configuration-for-system", system_name).bright_blue());
    println!();
    
    // Prompt for ROMs directory
    let roms_dir = Input::<String>::new()
        .with_prompt(&i18n::t_with_arg("roms-directory-for-system", system_name))
        .interact()?;
    
    let roms_path = PathBuf::from(roms_dir.trim());
    
    if !roms_path.exists() {
        eprintln!("{}", i18n::t_path("directory-not-exist-warning", &roms_path.display().to_string()).yellow());
        let continue_anyway = Confirm::new()
            .with_prompt("Continuar mesmo assim?")
            .default(false)
            .interact()?;
        
        if !continue_anyway {
            return prompt_for_console_config(system_name);
        }
    }
    
    // Prompt for output directory
    let default_output = format!("./playlists/{}", system_name.replace(" - ", "_").replace(" ", "_"));
    
    let output_dir = Input::<String>::new()
        .with_prompt(&i18n::t_with_arg("output-directory-for-system", system_name))
        .default(default_output.clone())
        .interact()?;
    
    let output_path = PathBuf::from(output_dir.trim());
    
    // Create output directory if it doesn't exist
    if !output_path.exists() {
        let create = Confirm::new()
            .with_prompt(&i18n::t_path("create-output-directory", &output_path.display().to_string()))
            .default(true)
            .interact()?;
        
        if create {
            std::fs::create_dir_all(&output_path)?;
            println!("  {}", i18n::t_path("directory-created", &output_path.display().to_string()).green());
        }
    }
    
    Ok(ConsoleConfig {
        system_name: system_name.to_string(),
        roms_dir: roms_path,
        output_dir: output_path,
        force_system: true, // In interactive mode, always force the selected system
    })
}

/// Function for indexing with forced system (interactive mode)
fn handle_index_command_forced_system(args: Args, forced_system: &str) -> Result<()> {
    if args.roms_dirs.is_empty() {
        eprintln!("{}", i18n::t("error-roms-dir-required").red());
        eprintln!("{}", i18n::t_with_arg("usage-instruction", "retroarch-indexer").cyan());
        std::process::exit(1);
    }

    // Load or create config
    let config = Config::load_or_create(args.config.as_deref())?;
    
    // Determine platforms
    let (source_platform, target_platform) = determine_platforms(&args, &config)?;
    
    println!("{}", i18n::t_path("scanning-directory", &format_paths(&args.roms_dirs)));
    println!("{}", i18n::t_with_arg("forced-system", forced_system).bright_cyan());    println!("{}", i18n::t_conversion(&source_platform.display_name(), &target_platform.display_name()).bright_yellow());
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
    let mut total_scanned_files = 0;
    let mut archives_found = 0;
    
    for (index, roms_dir) in args.roms_dirs.iter().enumerate() {
        let mut args_map = std::collections::HashMap::new();
        args_map.insert("current".to_string(), (index + 1).to_string());
        args_map.insert("total".to_string(), args.roms_dirs.len().to_string());
        args_map.insert("path".to_string(), roms_dir.display().to_string());
        println!("{}", i18n::t_with_args("scanning-directory-indexed", &args_map));
        
        let start_time = std::time::Instant::now();
        let mut roms = scanner.scan_directory(roms_dir)?;
        let scan_duration = start_time.elapsed();
        
        // FORCE ALL ROMS TO SELECTED SYSTEM
        for rom in &mut roms {
            rom.system = Some(forced_system.to_string());
            let filename = rom.path.file_name().unwrap_or_default().to_string_lossy().to_string();
            println!("  {}", i18n::t_rom_system("forcing-rom-to-system", &filename, forced_system));
        }
        
        // Count archives
        let dir_archives = roms.iter().filter(|rom| rom.is_archive).count();
        archives_found += dir_archives;
        
        total_scanned_files += roms.len();
        
        if args.verbose > 0 {
            let mut args_map = std::collections::HashMap::new();
            args_map.insert("count".to_string(), roms.len().to_string());
            args_map.insert("time".to_string(), format!("{:.2}", scan_duration.as_secs_f32()));
            println!("   {}", i18n::t_with_args("roms-found-summary", &args_map).bright_green());
            if dir_archives > 0 {
                let mut args_map = std::collections::HashMap::new();
                args_map.insert("count".to_string(), dir_archives.to_string());
                println!("   {}", i18n::t_with_args("archives-detected", &args_map).bright_magenta());
            }
        }
        
        all_roms.extend(roms);
    }

    if all_roms.is_empty() {
        println!("{}", i18n::t("no-roms-found").yellow());
        return Ok(());
    }

    // Show scanning summary
    println!("\n📈 {} do Escaneamento:", "Resumo".bright_cyan().bold());
    println!("{}", i18n::t_count("total-roms-stat", total_scanned_files as i32).bright_green());
    println!("{}", i18n::t_with_arg("forced-system", forced_system).bright_cyan());
    println!("{}", i18n::t_with_arg("directories-scanned", &args.roms_dirs.len().to_string()).bright_blue());
    if archives_found > 0 {
        println!("├─ Arquivos comprimidos: {}", archives_found.to_string().bright_magenta());
    }
    println!("└─ Threads utilizadas: {}", args.threads.unwrap_or_else(num_cpus::get).to_string().bright_yellow());
    println!();

    // Load DAT files if available
    let dat_collection = if let Some(dat_dir) = &args.dat_dir {
        dat_parser::DatCollection::load_directory(dat_dir)?
    } else {
        dat_parser::DatCollection::new()
    };

    // Build playlists - only for forced system
    let playlist_builder = PlaylistBuilder::new()
        .with_platforms(source_platform, target_platform)
        .with_dat_collection(dat_collection)
        .with_verbose(args.verbose > 0);

    // Create output directory
    std::fs::create_dir_all(&args.output_dir)?;

    // Generate playlist only for forced system
    let mut playlists_by_system = std::collections::HashMap::new();
    let playlist = playlist_builder.build_single_system_playlist(&all_roms, forced_system)?;
    playlists_by_system.insert(forced_system.to_string(), playlist);
    
    // Save playlist
    let filename = format!("{}.lpl", forced_system);
    let output_path = args.output_dir.join(&filename);
    if let Some(playlist) = playlists_by_system.get(forced_system) {
        playlist.save(&output_path)?;
        
        println!("📊 Sistema Processado:");
        println!("└─ {}: {} ROMs", forced_system.bright_white(), playlist.items.len().to_string().bright_green());
    }
    
    // Success summary
    println!("{}", i18n::t("indexing-completed-success").bright_green().bold());
    println!("{}", i18n::t_count("roms-processed", total_scanned_files as i32).bright_green());
    println!("├─ Sistema: {}", forced_system.bright_cyan());
    println!("├─ Playlists geradas: 1");
    println!("{}", i18n::t_path("output-directory", &args.output_dir.display().to_string()).bright_blue());
    
    // Generate report if requested
    if let Some(report_path) = &args.report {
        generate_report(&all_roms, &playlists_by_system, report_path)?;
    }
    
    Ok(())
}

/// Versão otimizada do handle_index_command_forced_system para modo paralelo
/// Retorna o número de ROMs processadas para estatísticas
fn handle_index_command_forced_system_optimized(args: Args, forced_system: &str) -> Result<usize> {
    if args.roms_dirs.is_empty() {
        return Err(anyhow::anyhow!("Diretório de ROMs não especificado"));
    }

    // Load or create config
    let config = Config::load_or_create(args.config.as_deref())?;
    
    // Determine platforms
    let (source_platform, target_platform) = determine_platforms(&args, &config)?;

    // Initialize scanner with simplified monitoring (no verbose progress bars)
    let scanner = Scanner::new()
        .with_threads(args.threads.unwrap_or_else(num_cpus::get))
        .with_recursive(!args.no_recursive)
        .with_calculate_crc(!args.no_crc)
        .with_extensions(args.extensions.as_deref())
        .with_verbose(false); // Sempre false para evitar contenção

    // Scan all ROM directories
    let mut all_roms = Vec::new();
    
    for roms_dir in &args.roms_dirs {
        let mut roms = scanner.scan_directory_simple(roms_dir)?;
        
        // FORCE ALL ROMS TO SELECTED SYSTEM
        for rom in &mut roms {
            rom.system = Some(forced_system.to_string());
        }
        
        all_roms.extend(roms);
    }

    if all_roms.is_empty() {
        return Ok(0);
    }

    // Load DAT files if available
    let dat_collection = if let Some(dat_dir) = &args.dat_dir {
        dat_parser::DatCollection::load_directory(dat_dir)?
    } else {
        dat_parser::DatCollection::new()
    };

    // Build playlists - only for forced system
    let playlist_builder = PlaylistBuilder::new()
        .with_platforms(source_platform, target_platform)
        .with_dat_collection(dat_collection)
        .with_verbose(false); // Desabilita verbose para performance

    // Create output directory
    std::fs::create_dir_all(&args.output_dir)?;

    // Generate playlist only for forced system
    let playlist = playlist_builder.build_single_system_playlist(&all_roms, forced_system)?;
    
    // Save playlist
    let filename = format!("{}.lpl", forced_system);
    let output_path = args.output_dir.join(&filename);
    playlist.save(&output_path)?;
    
    Ok(all_roms.len())
}

/// Process a single console configuration (uses all available threads)
fn process_single_console_config(args: &Args, config: &ConsoleConfig) -> Result<()> {
    // Configure args temporarily for this console
    let mut temp_args = args.clone();
    temp_args.roms_dirs = vec![config.roms_dir.clone()];
    temp_args.output_dir = config.output_dir.clone();
    
    // In interactive mode, force the specific system
    if config.force_system {
        temp_args.system = Some(config.system_name.clone());
    }
    
    // Execute indexing for this specific console
    match handle_index_command_forced_system(temp_args, &config.system_name) {
        Ok(_) => println!("  {} {}", "✅".green(), i18n::t("completed-successfully").bright_green()),
        Err(e) => {
            eprintln!("  {}", i18n::t_with_arg("error-processing-system", &format!("{}: {}", config.system_name, e)).red());
            return Err(e);
        }
    }
    Ok(())
}

/// Process multiple consoles in parallel (when sufficient threads available)
fn process_consoles_parallel(args: &Args, console_configs: Vec<ConsoleConfig>) -> Result<()> {
    use rayon::prelude::*;
    
    let total_threads = args.threads.unwrap_or_else(num_cpus::get);
    let threads_per_console = std::cmp::max(2, total_threads / console_configs.len()); // Mínimo 2 threads por console
    
    println!("  📊 {} threads por console (total: {})", threads_per_console, total_threads);
    println!("  ⚡ Processamento paralelo otimizado: progress simplificado");
    
    // Create a custom thread pool to avoid interference with individual console scanning
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(console_configs.len().min(total_threads))
        .build()
        .map_err(|e| anyhow::anyhow!("Falha ao criar thread pool: {}", e))?;
    
    // Process consoles in parallel using custom pool
    let results: Vec<Result<()>> = pool.install(|| {
        console_configs
            .par_iter()
            .map(|config| {
                let thread_id = rayon::current_thread_index().unwrap_or(0);
                println!("  🚀 [T{}] Iniciando: {}", thread_id, config.system_name.bright_green());
                
                // Configure args for this console with limited threads
                let mut temp_args = args.clone();
                temp_args.roms_dirs = vec![config.roms_dir.clone()];
                temp_args.output_dir = config.output_dir.clone();
                temp_args.threads = Some(threads_per_console);
                
                // IMPORTANTE: Desabilitar verbose para reduzir contenção de I/O
                temp_args.verbose = 0; // Desabilita progress detalhado individual
                
                // In interactive mode, force the specific system
                if config.force_system {
                    temp_args.system = Some(config.system_name.clone());
                }
                
                // Execute indexing for this specific console
                let start_time = std::time::Instant::now();
                match handle_index_command_forced_system_optimized(temp_args, &config.system_name) {
                    Ok(rom_count) => {
                        let duration = start_time.elapsed();
                        println!("  ✅ [T{}] {}: {} ROMs em {:.2}s", 
                                thread_id, 
                                config.system_name.bright_green(), 
                                rom_count, 
                                duration.as_secs_f32());
                        Ok(())
                    },
                    Err(e) => {
                        eprintln!("  ❌ [T{}] Erro em {}: {}", thread_id, config.system_name.bright_red(), e);
                        Err(e)
                    }
                }
            })
            .collect()
    });
    
    // Check for errors
    for result in results {
        result?;
    }
    
    Ok(())
}

/// Process multiple consoles sequentially (when limited threads)
fn process_consoles_sequential(args: &Args, console_configs: Vec<ConsoleConfig>) -> Result<()> {
    for config in console_configs {
        println!("\n{} Processando: {}", "🔄".bright_blue(), config.system_name.bright_green());
        
        // Configure args temporarily for this console
        let mut temp_args = args.clone();
        temp_args.roms_dirs = vec![config.roms_dir.clone()];
        temp_args.output_dir = config.output_dir.clone();
        
        // In interactive mode, force the specific system
        if config.force_system {
            temp_args.system = Some(config.system_name.clone());
        }
        
        // Execute indexing for this specific console
        match handle_index_command_forced_system(temp_args, &config.system_name) {
            Ok(_) => println!("  {} {}", "✅".green(), i18n::t("completed-successfully").bright_green()),
            Err(e) => {
                eprintln!("  {}", i18n::t_with_arg("error-processing-system", &format!("{}: {}", config.system_name, e)).red());
                continue;
            }
        }
    }
    Ok(())
}
