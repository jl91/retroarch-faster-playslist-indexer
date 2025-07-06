use std::collections::HashMap;
use anyhow::Result;
use lazy_static::lazy_static;
use std::sync::RwLock;

pub struct I18nEntry {
    pub key: String,
    pub value: String,
}

pub struct Locale {
    entries: HashMap<String, String>,
}

impl Locale {
    fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }
    
    fn add_entry(&mut self, key: String, value: String) {
        self.entries.insert(key, value);
    }
    
    fn get(&self, key: &str) -> Option<&String> {
        self.entries.get(key)
    }
}

pub struct I18n {
    locales: HashMap<String, Locale>,
    current_locale: String,
    fallback_locale: String,
}

impl I18n {
    pub fn new() -> Self {
        let mut i18n = Self {
            locales: HashMap::new(),
            current_locale: "en".to_string(),
            fallback_locale: "en".to_string(),
        };
        
        // Load basic English locale as fallback
        i18n.load_basic_locales();
        i18n
    }
    
    fn load_basic_locales(&mut self) {
        // Load English (fallback)
        let mut en_locale = Locale::new();
        en_locale.add_entry("app-name".to_string(), "RetroArch Fast Playlist Indexer".to_string());
        en_locale.add_entry("scanning-roms".to_string(), "Scanning ROMs...".to_string());
        en_locale.add_entry("found-roms".to_string(), "Found {} ROMs".to_string());
        en_locale.add_entry("processing-file".to_string(), "Processing: {}".to_string());
        en_locale.add_entry("scan-complete".to_string(), "Scan complete".to_string());
        en_locale.add_entry("systems-detected".to_string(), "Systems detected:".to_string());
        en_locale.add_entry("playlists-created".to_string(), "Playlists created successfully".to_string());
        
        // Main.rs strings
        en_locale.add_entry("rom-directories-config".to_string(), "📂 ROM Directories Configuration".to_string());
        en_locale.add_entry("roms-dir-prompt".to_string(), "Enter the ROM directory path".to_string());
        en_locale.add_entry("roms-dir-prompt-additional".to_string(), "Enter another ROM directory (or Enter to continue)".to_string());
        en_locale.add_entry("directory-not-found".to_string(), "⚠️ Directory not found: {}".to_string());
        en_locale.add_entry("not-a-directory".to_string(), "⚠️ Not a directory: {}".to_string());
        en_locale.add_entry("directory-added".to_string(), "✅ Added: {}".to_string());
        en_locale.add_entry("max-directories-reached".to_string(), "⚠️ Maximum directory limit reached".to_string());
        en_locale.add_entry("platforms-configuration".to_string(), "🔄 Platforms Configuration".to_string());
        en_locale.add_entry("select-source-platform".to_string(), "Select the {} platform (where you are running):".to_string());
        en_locale.add_entry("select-target-platform".to_string(), "Select the {} platform (where it will be used):".to_string());
        en_locale.add_entry("source".to_string(), "source".to_string());
        en_locale.add_entry("target".to_string(), "target".to_string());
        en_locale.add_entry("output-directory-config".to_string(), "📁 Output Directory Configuration".to_string());
        en_locale.add_entry("output-dir-prompt".to_string(), "Enter the output directory for playlists".to_string());
        en_locale.add_entry("create-directory-prompt".to_string(), "Directory doesn't exist. Create '{}'?".to_string());
        en_locale.add_entry("directory-created".to_string(), "✅ Directory created: {}".to_string());
        en_locale.add_entry("error-roms-dir-required".to_string(), "❌ Error: At least one ROM directory must be specified".to_string());
        en_locale.add_entry("scanning-directory".to_string(), "📂 Scanning: {}".to_string());
        en_locale.add_entry("conversion".to_string(), "🔄 Conversion: {} → {}".to_string());
        en_locale.add_entry("scanning-directory-progress".to_string(), "🔍 Scanning directory {} of {}: {}".to_string());
        en_locale.add_entry("no-roms-found".to_string(), "⚠️ No ROMs found in specified directories".to_string());
        en_locale.add_entry("scan-summary".to_string(), "📈 Scan Summary:".to_string());
        en_locale.add_entry("directories-scanned".to_string(), "├─ Directories scanned: {}".to_string());
        en_locale.add_entry("directories-count".to_string(), "{} directories".to_string());
        en_locale.add_entry("total-roms".to_string(), "├─ Total ROMs: {}".to_string());
        en_locale.add_entry("archives-found".to_string(), "├─ Archives found: {}".to_string());
        en_locale.add_entry("systems-detected".to_string(), "├─ Systems detected: {}".to_string());
        en_locale.add_entry("scan-time".to_string(), "└─ Scan time: {:.2}s".to_string());
        en_locale.add_entry("report-generated".to_string(), "📄 Report generated: {}".to_string());
        en_locale.add_entry("indexing-complete".to_string(), "🎉 Indexing completed successfully!".to_string());
        en_locale.add_entry("playlist-conversion-mode".to_string(), "🔄 Playlist Conversion Mode".to_string());
        en_locale.add_entry("loading-playlist".to_string(), "📄 Loading: {}".to_string());
        en_locale.add_entry("detecting-source-platform".to_string(), "🔍 Detecting source platform...".to_string());
        en_locale.add_entry("platform-detected".to_string(), "✅ Platform detected: {}".to_string());
        en_locale.add_entry("batch-conversion-mode".to_string(), "🔄 Batch Conversion Mode".to_string());
        en_locale.add_entry("no-lpl-files-found".to_string(), "⚠️ No .lpl files found in specified directory".to_string());
        en_locale.add_entry("batch-conversion-complete".to_string(), "✅ Batch conversion completed:".to_string());
        en_locale.add_entry("successful-conversions".to_string(), "├─ Successful: {}".to_string());
        en_locale.add_entry("failed-conversions".to_string(), "├─ Failed: {}".to_string());
        en_locale.add_entry("output-directory".to_string(), "└─ Output directory: {}".to_string());
        en_locale.add_entry("watch-mode".to_string(), "👀 Watch Mode".to_string());
        en_locale.add_entry("settings".to_string(), "🔧 Settings:".to_string());
        en_locale.add_entry("debounce-interval".to_string(), "├─ Debounce interval: {}ms".to_string());
        en_locale.add_entry("batch-size".to_string(), "├─ Batch size: {}".to_string());
        en_locale.add_entry("include-archives".to_string(), "├─ Include Archives: {}".to_string());
        en_locale.add_entry("yes".to_string(), "Yes".to_string());
        en_locale.add_entry("no".to_string(), "No".to_string());
        en_locale.add_entry("dat-download".to_string(), "📥 Automatic DAT Download".to_string());
        en_locale.add_entry("getting-systems-list".to_string(), "📋 Getting available systems list...".to_string());
        en_locale.add_entry("destination-directory".to_string(), "📁 Destination directory: {}".to_string());
        en_locale.add_entry("force-redownload".to_string(), "🔄 Force re-download: {}".to_string());
        en_locale.add_entry("integrity-validation".to_string(), "🔍 Integrity Validation".to_string());
        en_locale.add_entry("dat-not-found".to_string(), "⚠️ DAT not found: {}".to_string());
        en_locale.add_entry("scanning-roms-for-validation".to_string(), "🔍 Scanning: {}".to_string());
        en_locale.add_entry("validating-roms".to_string(), "🔍 Validating {} ROMs...".to_string());
        en_locale.add_entry("validation-results".to_string(), "📊 Validation Results:".to_string());
        en_locale.add_entry("valid-roms".to_string(), "├─ ✅ Valid: {}".to_string());
        en_locale.add_entry("invalid-roms".to_string(), "├─ ❌ Invalid: {}".to_string());
        en_locale.add_entry("unknown-roms".to_string(), "├─ ❓ Unknown: {}".to_string());
        en_locale.add_entry("validation-time".to_string(), "└─ Validation time: {:.2}s".to_string());
        
        // Additional missing strings
        en_locale.add_entry("use-command-format".to_string(), "Use: {} --roms-dir <PATH>".to_string());
        en_locale.add_entry("conversion-completed".to_string(), "Conversion completed".to_string());
        en_locale.add_entry("download-completed".to_string(), "Download completed".to_string());
        en_locale.add_entry("errors-found".to_string(), "   {} errors found".to_string());
        en_locale.add_entry("errors-count".to_string(), "├─ ❌ Errors: {}".to_string());
        en_locale.add_entry("deduplication-complete".to_string(), "✅ Deduplication completed successfully!".to_string());
        en_locale.add_entry("completed-successfully".to_string(), "Completed successfully".to_string());
        en_locale.add_entry("processing-all-consoles-complete".to_string(), "🎉 Processing of all consoles completed!".to_string());
        en_locale.add_entry("indexing-completed-success".to_string(), "🎉 Indexing Completed Successfully!".to_string());
        en_locale.add_entry("directory-not-exist-warning".to_string(), "⚠️ Warning: Directory {} does not exist".to_string());
        en_locale.add_entry("error-getting-roms-dirs".to_string(), "❌ Error getting ROM directories: {}".to_string());
        en_locale.add_entry("error-getting-platforms".to_string(), "❌ Error getting platforms: {}".to_string());
        en_locale.add_entry("error-getting-output-dir".to_string(), "❌ Error getting output directory: {}".to_string());
        en_locale.add_entry("no-available-systems".to_string(), "❌ No available systems found".to_string());
        en_locale.add_entry("error-processing-system".to_string(), "  ❌ Error processing {}: {}".to_string());
        
        // Deduplication strings
        en_locale.add_entry("intelligent-deduplication".to_string(), "🗂️ Intelligent Deduplication".to_string());
        en_locale.add_entry("deduplication-results".to_string(), "📊 Deduplication Results:".to_string());
        en_locale.add_entry("duplicates-removed".to_string(), "├─ Duplicates removed: {}".to_string());
        en_locale.add_entry("files-kept".to_string(), "├─ Files kept: {}".to_string());
        en_locale.add_entry("space-saved".to_string(), "└─ Space saved: {}".to_string());
        
        // Mode selection strings
        en_locale.add_entry("watch-mode-active".to_string(), "👀 Watch Mode Active".to_string());
        en_locale.add_entry("simulation-mode-active".to_string(), "🚫 Simulation mode active - no files will be removed".to_string());
        en_locale.add_entry("execution-mode".to_string(), "🎮 Execution Mode".to_string());
        en_locale.add_entry("interactive-mode-console-selection".to_string(), "Interactive Mode - Console Selection".to_string());
        en_locale.add_entry("automatic-mode-scan-all".to_string(), "Automatic Mode - Scan all directories".to_string());
        en_locale.add_entry("scan-all-directories-current".to_string(), "Automatically scan all provided directories (current mode)".to_string());
        en_locale.add_entry("select-mode".to_string(), "Select mode".to_string());
        
        // Mode descriptions
        en_locale.add_entry("interactive-mode-desc".to_string(), "Choose which consoles/cores to sync and configure specific folders".to_string());
        en_locale.add_entry("automatic-mode-desc".to_string(), "Automatically scan all provided directories (current mode)".to_string());
        en_locale.add_entry("choose-indexer-execution".to_string(), "Choose how to run the indexer:".to_string());
        
        // Validation and report strings
        en_locale.add_entry("integrity-validation-title".to_string(), "🔍 Integrity Validation".to_string());
        en_locale.add_entry("dat-not-found-warning".to_string(), "⚠️ DAT not found: {}".to_string());
        en_locale.add_entry("validation-results-title".to_string(), "📊 Validation Results:".to_string());
        en_locale.add_entry("valid-roms-percent".to_string(), "├─ ✅ Valid: {} ({:.1}%)".to_string());
        en_locale.add_entry("detailed-report-saved".to_string(), "📄 Detailed report saved to: {}".to_string());
        en_locale.add_entry("strategy-info".to_string(), "🎯 Strategy: {:?}".to_string());
        en_locale.add_entry("space-freed".to_string(), "├─ Space freed: {}".to_string());
        en_locale.add_entry("unique-roms-remaining".to_string(), "└─ Unique ROMs remaining: {}".to_string());
        en_locale.add_entry("cache-statistics".to_string(), "📊 Cache Statistics:".to_string());
        en_locale.add_entry("last-updated".to_string(), "└─ Last updated: {}".to_string());
        en_locale.add_entry("source-platform-prompt".to_string(), "source (where it's running)".to_string());
        en_locale.add_entry("target-platform-prompt".to_string(), "target (where it will be used)".to_string());
        en_locale.add_entry("invalid-selection".to_string(), "❌ Invalid selection. Try again.".to_string());
        en_locale.add_entry("indexing-report-content".to_string(), "# Indexing Report\n\nReport in development...".to_string());
        
        // Additional statistics and summary strings
        en_locale.add_entry("total-roms-found".to_string(), "🔍 Total ROMs: {}".to_string());
        en_locale.add_entry("total-cache-entries".to_string(), "├─ Total entries: {}".to_string());
        en_locale.add_entry("total-roms-stat".to_string(), "├─ Total ROMs: {}".to_string());
        en_locale.add_entry("roms-processed".to_string(), "├─ ROMs processed: {}".to_string());
        en_locale.add_entry("master-playlist-info".to_string(), "└─ roms.lpl (master playlist with {} ROMs)".to_string());
        
        // Validation result details
        en_locale.add_entry("validation-total".to_string(), "├─ Total: {}".to_string());
        en_locale.add_entry("validation-valid".to_string(), "├─ ✅ Valid: {} ({:.1}%)".to_string());
        en_locale.add_entry("validation-need-rename".to_string(), "├─ 🔄 Need Rename: {}".to_string());
        en_locale.add_entry("validation-unknown".to_string(), "├─ ❓ Unknown: {}".to_string());
        en_locale.add_entry("validation-homebrew".to_string(), "├─ 🏠 Homebrew/Hack: {}".to_string());
        en_locale.add_entry("validation-bad-dumps".to_string(), "├─ ❌ Bad Dumps: {}".to_string());
        en_locale.add_entry("validation-corrupted".to_string(), "└─ 💥 Corrupted: {}".to_string());
        
        // Additional prompt strings
        en_locale.add_entry("use-default-output-dir".to_string(), "Use default output directory './playlists'?".to_string());
        en_locale.add_entry("console-cores-selection".to_string(), "🎯 Consoles/Cores Selection".to_string());
        en_locale.add_entry("systems-selection-prompt".to_string(), "Systems selection prompt".to_string());
        en_locale.add_entry("select-system-or-finish".to_string(), "Select a system to configure (or 'Finish selection' to finalize)".to_string());
        en_locale.add_entry("finish-selection".to_string(), "🏁 Finish selection".to_string());
        en_locale.add_entry("system-already-selected".to_string(), "⚠️ System {} already selected!".to_string());
        en_locale.add_entry("systems-selected-so-far".to_string(), "Systems selected so far: {}".to_string());
        en_locale.add_entry("configuration-for-system".to_string(), "⚙️ Configuration for: {}".to_string());
        en_locale.add_entry("roms-directory-for-system".to_string(), "ROMs directory for {}".to_string());
        en_locale.add_entry("output-directory-for-system".to_string(), "Output directory for {} playlists".to_string());
        en_locale.add_entry("create-output-directory".to_string(), "Create output directory '{}'?".to_string());
        en_locale.add_entry("directory-created".to_string(), "✅ Directory created: {}".to_string());
        en_locale.add_entry("system-added".to_string(), "✅ System {} added!".to_string());
        en_locale.add_entry("forced-system".to_string(), "🎯 Forced System: {}".to_string());
        en_locale.add_entry("forcing-rom-to-system".to_string(), "🎯 Forcing {} to system: {}".to_string());
        en_locale.add_entry("forced-system-scan".to_string(), "├─ Forced System: {}".to_string());
        en_locale.add_entry("scanning-directory-indexed".to_string(), "🔍 Scanning directory {} of {}: {}".to_string());
        en_locale.add_entry("force-all-roms-comment".to_string(), "FORCE ALL ROMS TO SELECTED SYSTEM".to_string());
        en_locale.add_entry("build-playlists-forced-only".to_string(), "Build playlists - only for forced system".to_string());
        en_locale.add_entry("generate-playlist-forced-only".to_string(), "Generate playlist only for forced system".to_string());
        en_locale.add_entry("converting-to".to_string(), "🎯 Converting to: {}".to_string());
        en_locale.add_entry("watch-active-press-ctrl-c".to_string(), "✅ Watch active! Press Ctrl+C to stop...".to_string());
        en_locale.add_entry("systems-for-download".to_string(), "🎯 Systems for download: {}".to_string());
        en_locale.add_entry("configure-args-temporarily".to_string(), "Configure args temporarily for this console".to_string());
        en_locale.add_entry("available-systems-consoles".to_string(), "Available Systems/Consoles:".to_string());
        
        self.locales.insert("en".to_string(), en_locale);
        
        // Load Portuguese
        let mut pt_locale = Locale::new();
        pt_locale.add_entry("app-name".to_string(), "RetroArch Fast Playlist Indexer".to_string());
        pt_locale.add_entry("scanning-roms".to_string(), "Escaneando ROMs...".to_string());
        pt_locale.add_entry("found-roms".to_string(), "Encontrados {} ROMs".to_string());
        pt_locale.add_entry("processing-file".to_string(), "Processando: {}".to_string());
        pt_locale.add_entry("scan-complete".to_string(), "Escaneamento completo".to_string());
        pt_locale.add_entry("systems-detected".to_string(), "Sistemas detectados:".to_string());
        pt_locale.add_entry("playlists-created".to_string(), "Playlists criadas com sucesso".to_string());
        
        // Main.rs strings in Portuguese
        pt_locale.add_entry("rom-directories-config".to_string(), "📂 Configuração de Diretórios de ROMs".to_string());
        pt_locale.add_entry("roms-dir-prompt".to_string(), "Digite o caminho do diretório de ROMs".to_string());
        pt_locale.add_entry("roms-dir-prompt-additional".to_string(), "Digite outro diretório de ROMs (ou Enter para continuar)".to_string());
        pt_locale.add_entry("directory-not-found".to_string(), "⚠️ Diretório não encontrado: {}".to_string());
        pt_locale.add_entry("not-a-directory".to_string(), "⚠️ Não é um diretório: {}".to_string());
        pt_locale.add_entry("directory-added".to_string(), "✅ Adicionado: {}".to_string());
        pt_locale.add_entry("max-directories-reached".to_string(), "⚠️ Limite máximo de diretórios atingido".to_string());
        pt_locale.add_entry("platforms-configuration".to_string(), "🔄 Configuração de Plataformas".to_string());
        pt_locale.add_entry("select-source-platform".to_string(), "Selecione a plataforma de {} (onde você está executando):".to_string());
        pt_locale.add_entry("select-target-platform".to_string(), "Selecione a plataforma de {} (onde será usado):".to_string());
        pt_locale.add_entry("source".to_string(), "origem".to_string());
        pt_locale.add_entry("target".to_string(), "destino".to_string());
        pt_locale.add_entry("output-directory-config".to_string(), "📁 Configuração de Diretório de Saída".to_string());
        pt_locale.add_entry("output-dir-prompt".to_string(), "Digite o diretório de saída para playlists".to_string());
        pt_locale.add_entry("create-directory-prompt".to_string(), "Diretório não existe. Criar '{}'?".to_string());
        pt_locale.add_entry("directory-created".to_string(), "✅ Diretório criado: {}".to_string());
        pt_locale.add_entry("error-roms-dir-required".to_string(), "❌ Erro: Pelo menos um diretório de ROMs deve ser especificado".to_string());
        pt_locale.add_entry("scanning-directory".to_string(), "📂 Escaneando: {}".to_string());
        pt_locale.add_entry("conversion".to_string(), "🔄 Conversão: {} → {}".to_string());
        pt_locale.add_entry("scanning-directory-progress".to_string(), "🔍 Escaneando diretório {} de {}: {}".to_string());
        pt_locale.add_entry("no-roms-found".to_string(), "⚠️ Nenhuma ROM encontrada nos diretórios especificados".to_string());
        pt_locale.add_entry("scan-summary".to_string(), "📈 Resumo do Escaneamento:".to_string());
        pt_locale.add_entry("directories-scanned".to_string(), "├─ Diretórios escaneados: {}".to_string());
        pt_locale.add_entry("directories-count".to_string(), "{} diretórios".to_string());
        pt_locale.add_entry("total-roms".to_string(), "├─ Total de ROMs: {}".to_string());
        pt_locale.add_entry("archives-found".to_string(), "├─ Arquivos encontrados: {}".to_string());
        pt_locale.add_entry("systems-detected".to_string(), "├─ Sistemas detectados: {}".to_string());
        pt_locale.add_entry("scan-time".to_string(), "└─ Tempo de escaneamento: {:.2}s".to_string());
        pt_locale.add_entry("report-generated".to_string(), "📄 Relatório gerado: {}".to_string());
        pt_locale.add_entry("indexing-complete".to_string(), "🎉 Indexação concluída com sucesso!".to_string());
        pt_locale.add_entry("playlist-conversion-mode".to_string(), "🔄 Modo Conversão de Playlist".to_string());
        pt_locale.add_entry("loading-playlist".to_string(), "📄 Carregando: {}".to_string());
        pt_locale.add_entry("detecting-source-platform".to_string(), "🔍 Detectando plataforma de origem...".to_string());
        pt_locale.add_entry("platform-detected".to_string(), "✅ Plataforma detectada: {}".to_string());
        pt_locale.add_entry("batch-conversion-mode".to_string(), "🔄 Modo Conversão em Lote".to_string());
        pt_locale.add_entry("no-lpl-files-found".to_string(), "⚠️ Nenhum arquivo .lpl encontrado no diretório especificado".to_string());
        pt_locale.add_entry("batch-conversion-complete".to_string(), "✅ Conversão em lote concluída:".to_string());
        pt_locale.add_entry("successful-conversions".to_string(), "├─ Sucessos: {}".to_string());
        pt_locale.add_entry("failed-conversions".to_string(), "├─ Falhas: {}".to_string());
        pt_locale.add_entry("output-directory".to_string(), "└─ Diretório de saída: {}".to_string());
        pt_locale.add_entry("watch-mode".to_string(), "👀 Modo Observação".to_string());
        pt_locale.add_entry("settings".to_string(), "🔧 Configurações:".to_string());
        pt_locale.add_entry("debounce-interval".to_string(), "├─ Intervalo de debounce: {}ms".to_string());
        pt_locale.add_entry("batch-size".to_string(), "├─ Tamanho do lote: {}".to_string());
        pt_locale.add_entry("include-archives".to_string(), "├─ Incluir Archives: {}".to_string());
        pt_locale.add_entry("yes".to_string(), "Sim".to_string());
        pt_locale.add_entry("no".to_string(), "Não".to_string());
        pt_locale.add_entry("dat-download".to_string(), "📥 Download Automático de DATs".to_string());
        pt_locale.add_entry("getting-systems-list".to_string(), "📋 Obtendo lista de sistemas disponíveis...".to_string());
        pt_locale.add_entry("destination-directory".to_string(), "📁 Diretório de destino: {}".to_string());
        pt_locale.add_entry("force-redownload".to_string(), "🔄 Forçar re-download: {}".to_string());
        pt_locale.add_entry("integrity-validation".to_string(), "🔍 Validação de Integridade".to_string());
        pt_locale.add_entry("dat-not-found".to_string(), "⚠️ DAT não encontrado: {}".to_string());
        pt_locale.add_entry("scanning-roms-for-validation".to_string(), "🔍 Escaneando: {}".to_string());
        pt_locale.add_entry("validating-roms".to_string(), "🔍 Validando {} ROMs...".to_string());
        pt_locale.add_entry("validation-results".to_string(), "📊 Resultado da Validação:".to_string());
        pt_locale.add_entry("valid-roms".to_string(), "├─ ✅ Válidas: {}".to_string());
        pt_locale.add_entry("invalid-roms".to_string(), "├─ ❌ Inválidas: {}".to_string());
        pt_locale.add_entry("unknown-roms".to_string(), "├─ ❓ Desconhecidas: {}".to_string());
        pt_locale.add_entry("validation-time".to_string(), "└─ Tempo de validação: {:.2}s".to_string());
        
        // Additional missing strings in Portuguese
        pt_locale.add_entry("use-command-format".to_string(), "Use: {} --roms-dir <PATH>".to_string());
        pt_locale.add_entry("conversion-completed".to_string(), "Conversão concluída".to_string());
        pt_locale.add_entry("download-completed".to_string(), "Download concluído".to_string());
        pt_locale.add_entry("errors-found".to_string(), "   {} erros encontrados".to_string());
        pt_locale.add_entry("errors-count".to_string(), "├─ ❌ Erros: {}".to_string());
        pt_locale.add_entry("deduplication-complete".to_string(), "✅ Deduplicação concluída com sucesso!".to_string());
        pt_locale.add_entry("completed-successfully".to_string(), "Concluído com sucesso".to_string());
        pt_locale.add_entry("processing-all-consoles-complete".to_string(), "🎉 Processamento de todos os consoles concluído!".to_string());
        pt_locale.add_entry("indexing-completed-success".to_string(), "🎉 Indexação Concluída com Sucesso!".to_string());
        pt_locale.add_entry("directory-not-exist-warning".to_string(), "⚠️ Aviso: O diretório {} não existe".to_string());
        pt_locale.add_entry("error-getting-roms-dirs".to_string(), "❌ Erro ao obter diretórios de ROMs: {}".to_string());
        pt_locale.add_entry("error-getting-platforms".to_string(), "❌ Erro ao obter plataformas: {}".to_string());
        pt_locale.add_entry("error-getting-output-dir".to_string(), "❌ Erro ao obter diretório de saída: {}".to_string());
        pt_locale.add_entry("no-available-systems".to_string(), "❌ Nenhum sistema disponível encontrado".to_string());
        pt_locale.add_entry("error-processing-system".to_string(), "  ❌ Erro ao processar {}: {}".to_string());
        
        // Deduplication strings in Portuguese
        pt_locale.add_entry("intelligent-deduplication".to_string(), "🗂️ Deduplicação Inteligente".to_string());
        pt_locale.add_entry("deduplication-results".to_string(), "📊 Resultado da Deduplicação:".to_string());
        pt_locale.add_entry("duplicates-removed".to_string(), "├─ Duplicatas removidas: {}".to_string());
        pt_locale.add_entry("files-kept".to_string(), "├─ Arquivos mantidos: {}".to_string());
        pt_locale.add_entry("space-saved".to_string(), "└─ Espaço economizado: {}".to_string());
        
        // Mode selection strings in Portuguese
        pt_locale.add_entry("watch-mode-active".to_string(), "👀 Modo Watch Ativado".to_string());
        pt_locale.add_entry("simulation-mode-active".to_string(), "🚫 Modo simulação ativado - nenhum arquivo será removido".to_string());
        pt_locale.add_entry("execution-mode".to_string(), "🎮 Modo de Execução".to_string());
        pt_locale.add_entry("interactive-mode-console-selection".to_string(), "Modo Interativo - Seleção de Consoles".to_string());
        pt_locale.add_entry("automatic-mode-scan-all".to_string(), "Modo Automático - Escanear todos os diretórios".to_string());
        pt_locale.add_entry("scan-all-directories-current".to_string(), "Escanear automaticamente todos os diretórios fornecidos (modo atual)".to_string());
        pt_locale.add_entry("select-mode".to_string(), "Selecione o modo".to_string());
        
        // Mode descriptions in Portuguese
        pt_locale.add_entry("interactive-mode-desc".to_string(), "Escolha quais consoles/cores sincronizar e configure pastas específicas".to_string());
        pt_locale.add_entry("automatic-mode-desc".to_string(), "Escanear automaticamente todos os diretórios fornecidos (modo atual)".to_string());
        pt_locale.add_entry("choose-indexer-execution".to_string(), "Escolha como deseja executar o indexador:".to_string());
        
        // Validation and report strings in Portuguese
        pt_locale.add_entry("integrity-validation-title".to_string(), "🔍 Validação de Integridade".to_string());
        pt_locale.add_entry("dat-not-found-warning".to_string(), "⚠️ DAT não encontrado: {}".to_string());
        pt_locale.add_entry("validation-results-title".to_string(), "📊 Resultado da Validação:".to_string());
        pt_locale.add_entry("valid-roms-percent".to_string(), "├─ ✅ Válidas: {} ({:.1}%)".to_string());
        pt_locale.add_entry("detailed-report-saved".to_string(), "📄 Relatório detalhado salvo em: {}".to_string());
        pt_locale.add_entry("strategy-info".to_string(), "🎯 Estratégia: {:?}".to_string());
        pt_locale.add_entry("space-freed".to_string(), "├─ Espaço liberado: {}".to_string());
        pt_locale.add_entry("unique-roms-remaining".to_string(), "└─ ROMs únicas restantes: {}".to_string());
        pt_locale.add_entry("cache-statistics".to_string(), "📊 Estatísticas do Cache:".to_string());
        pt_locale.add_entry("last-updated".to_string(), "└─ Última atualização: {}".to_string());
        pt_locale.add_entry("source-platform-prompt".to_string(), "origem (onde está executando)".to_string());
        pt_locale.add_entry("target-platform-prompt".to_string(), "destino (onde será usado)".to_string());
        pt_locale.add_entry("invalid-selection".to_string(), "❌ Seleção inválida. Tente novamente.".to_string());
        pt_locale.add_entry("indexing-report-content".to_string(), "# Relatório de Indexação\n\nRelatório em desenvolvimento...".to_string());
        
        // Additional statistics and summary strings in Portuguese
        pt_locale.add_entry("total-roms-found".to_string(), "🔍 Total de ROMs: {}".to_string());
        pt_locale.add_entry("total-cache-entries".to_string(), "├─ Total de entradas: {}".to_string());
        pt_locale.add_entry("total-roms-stat".to_string(), "├─ Total de ROMs: {}".to_string());
        pt_locale.add_entry("roms-processed".to_string(), "├─ ROMs processadas: {}".to_string());
        pt_locale.add_entry("master-playlist-info".to_string(), "└─ roms.lpl (playlist master com {} ROMs)".to_string());
        
        // Validation result details in Portuguese
        pt_locale.add_entry("validation-total".to_string(), "├─ Total: {}".to_string());
        pt_locale.add_entry("validation-valid".to_string(), "├─ ✅ Válidas: {} ({:.1}%)".to_string());
        pt_locale.add_entry("validation-need-rename".to_string(), "├─ 🔄 Precisam Renomear: {}".to_string());
        pt_locale.add_entry("validation-unknown".to_string(), "├─ ❓ Desconhecidas: {}".to_string());
        pt_locale.add_entry("validation-homebrew".to_string(), "├─ 🏠 Homebrew/Hack: {}".to_string());
        pt_locale.add_entry("validation-bad-dumps".to_string(), "├─ ❌ Bad Dumps: {}".to_string());
        pt_locale.add_entry("validation-corrupted".to_string(), "└─ 💥 Corrompidas: {}".to_string());
        
        // Additional prompt strings
        pt_locale.add_entry("use-default-output-dir".to_string(), "Usar diretório de saída padrão './playlists'?".to_string());
        pt_locale.add_entry("console-cores-selection".to_string(), "🎯 Seleção de Consoles/Cores".to_string());
        pt_locale.add_entry("systems-selection-prompt".to_string(), "Prompt para seleção de sistemas".to_string());
        pt_locale.add_entry("select-system-or-finish".to_string(), "Selecione um sistema para configurar (ou 'Concluir seleção' para finalizar)".to_string());
        pt_locale.add_entry("finish-selection".to_string(), "🏁 Concluir seleção".to_string());
        pt_locale.add_entry("system-already-selected".to_string(), "⚠️ Sistema {} já foi selecionado!".to_string());
        pt_locale.add_entry("systems-selected-so-far".to_string(), "Sistemas selecionados até agora: {}".to_string());
        pt_locale.add_entry("configuration-for-system".to_string(), "⚙️ Configuração para: {}".to_string());
        pt_locale.add_entry("roms-directory-for-system".to_string(), "Diretório de ROMs para {}".to_string());
        pt_locale.add_entry("output-directory-for-system".to_string(), "Diretório de saída para playlists de {}".to_string());
        pt_locale.add_entry("create-output-directory".to_string(), "Criar diretório de saída '{}'?".to_string());
        pt_locale.add_entry("directory-created".to_string(), "✅ Diretório criado: {}".to_string());
        pt_locale.add_entry("system-added".to_string(), "✅ Sistema {} adicionado!".to_string());
        pt_locale.add_entry("forced-system".to_string(), "🎯 Sistema Forçado: {}".to_string());
        pt_locale.add_entry("forcing-rom-to-system".to_string(), "🎯 Forçando {} para sistema: {}".to_string());
        pt_locale.add_entry("forced-system-scan".to_string(), "├─ Sistema Forçado: {}".to_string());
        pt_locale.add_entry("scanning-directory-indexed".to_string(), "🔍 Escaneando diretório {} de {}: {}".to_string());
        pt_locale.add_entry("force-all-roms-comment".to_string(), "FORÇAR TODAS AS ROMS PARA O SISTEMA SELECIONADO".to_string());
        pt_locale.add_entry("build-playlists-forced-only".to_string(), "Build playlists - apenas para o sistema forçado".to_string());
        pt_locale.add_entry("generate-playlist-forced-only".to_string(), "Gerar playlist apenas para o sistema forçado".to_string());
        pt_locale.add_entry("converting-to".to_string(), "🎯 Convertendo para: {}".to_string());
        pt_locale.add_entry("watch-active-press-ctrl-c".to_string(), "✅ Watch ativo! Pressione Ctrl+C para parar...".to_string());
        pt_locale.add_entry("systems-for-download".to_string(), "🎯 Sistemas para download: {}".to_string());
        pt_locale.add_entry("configure-args-temporarily".to_string(), "Configurar args temporariamente para este console".to_string());
        pt_locale.add_entry("available-systems-consoles".to_string(), "Sistemas/Consoles disponíveis:".to_string());
        
        self.locales.insert("pt".to_string(), pt_locale);
        
        // Basic detection based on env
        if let Ok(lang) = std::env::var("LANG") {
            if lang.starts_with("pt") {
                self.current_locale = "pt".to_string();
            }
        }
    }
    
    pub fn set_locale(&mut self, locale: &str) {
        if self.locales.contains_key(locale) {
            self.current_locale = locale.to_string();
        }
    }
    
    pub fn get_current_locale(&self) -> &str {
        &self.current_locale
    }
    
    pub fn t(&self, key: &str) -> String {
        // Try current locale first
        if let Some(locale) = self.locales.get(&self.current_locale) {
            if let Some(value) = locale.get(key) {
                return value.clone();
            }
        }
        
        // Fallback to English
        if self.current_locale != self.fallback_locale {
            if let Some(locale) = self.locales.get(&self.fallback_locale) {
                if let Some(value) = locale.get(key) {
                    return value.clone();
                }
            }
        }
        
        // Ultimate fallback: return the key itself
        key.to_string()
    }
    
    pub fn t_with_arg(&self, key: &str, arg: &str) -> String {
        let template = self.t(key);
        template.replace("{}", arg)
    }
    
    pub fn t_count(&self, key: &str, count: i32) -> String {
        self.t_with_arg(key, &count.to_string())
    }
}

// Global instance using lazy_static for thread safety
lazy_static! {
    static ref I18N_INSTANCE: RwLock<I18n> = RwLock::new(I18n::new());
}

pub fn init_i18n() -> Result<()> {
    // Initialize is already done by lazy_static
    Ok(())
}

pub fn set_locale(locale: &str) {
    if let Ok(mut i18n) = I18N_INSTANCE.write() {
        i18n.set_locale(locale);
    }
}

pub fn get_current_locale() -> String {
    if let Ok(i18n) = I18N_INSTANCE.read() {
        i18n.get_current_locale().to_string()
    } else {
        "en".to_string()
    }
}

pub fn t(key: &str) -> String {
    if let Ok(i18n) = I18N_INSTANCE.read() {
        i18n.t(key)
    } else {
        key.to_string()
    }
}

pub fn t_count(key: &str, count: i32) -> String {
    if let Ok(i18n) = I18N_INSTANCE.read() {
        i18n.t_count(key, count)
    } else {
        format!("{}: {}", key, count)
    }
}

pub fn t_with_arg(key: &str, arg: &str) -> String {
    if let Ok(i18n) = I18N_INSTANCE.read() {
        i18n.t_with_arg(key, arg)
    } else {
        format!("{}: {}", key, arg)
    }
}

pub fn t_filename(key: &str, filename: &str) -> String {
    if let Ok(i18n) = I18N_INSTANCE.read() {
        i18n.t_with_arg(key, filename)
    } else {
        format!("{}: {}", key, filename)
    }
}

pub fn t_progress(key: &str, progress: i32) -> String {
    if let Ok(i18n) = I18N_INSTANCE.read() {
        i18n.t_with_arg(key, &format!("{}%", progress))
    } else {
        format!("{}: {}%", key, progress)
    }
}

pub fn t_path(key: &str, path: &str) -> String {
    if let Ok(i18n) = I18N_INSTANCE.read() {
        i18n.t_with_arg(key, path)
    } else {
        format!("{}: {}", key, path)
    }
}

pub fn t_platform(key: &str, platform: &str) -> String {
    if let Ok(i18n) = I18N_INSTANCE.read() {
        i18n.t_with_arg(key, platform)
    } else {
        format!("{}: {}", key, platform)
    }
}

pub fn t_with_args(key: &str, args: &[&str]) -> String {
    if let Ok(i18n) = I18N_INSTANCE.read() {
        let mut result = i18n.t(key);
        for (i, arg) in args.iter().enumerate() {
            result = result.replace(&format!("{{{}}}", i), arg);
        }
        result
    } else {
        format!("{}: {:?}", key, args)
    }
}

pub fn t_conversion(source: &str, target: &str) -> String {
    t_with_args("conversion", &[source, target])
}

pub fn t_directory_progress(current: usize, total: usize, path: &str) -> String {
    t_with_args("scanning-directory-progress", &[&current.to_string(), &total.to_string(), path])
}
