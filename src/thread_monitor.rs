use std::sync::Arc;
use std::time::Instant;
use std::collections::HashMap;
use parking_lot::RwLock;
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};

/// Status de uma thread de processamento
#[derive(Debug, Clone)]
pub enum ThreadStatus {
    Idle,
    ScanningFile(String),
    ExtractingArchive { file: String, progress: f32 },
    CalculatingCrc(String),
    ProcessingComplete(String),
    Error(String, String), // filename, error
}

/// Informações sobre uma thread
#[derive(Debug, Clone)]
pub struct ThreadInfo {
    pub id: usize,
    pub status: ThreadStatus,
    pub start_time: Instant,
    pub files_processed: usize,
}

/// Gerenciador de status das threads em tempo real
pub struct ThreadMonitor {
    threads: Arc<RwLock<HashMap<usize, ThreadInfo>>>,
    multi_progress: MultiProgress,
    main_progress: ProgressBar,
    thread_progress_bars: Arc<RwLock<HashMap<usize, ProgressBar>>>,
    verbose: bool,
}

impl ThreadMonitor {
    pub fn new(total_files: usize, _max_threads: usize, verbose: bool) -> Self {
        let multi_progress = MultiProgress::new();
        
        // Progress bar principal
        let main_progress = multi_progress.add(ProgressBar::new(total_files as u64));
        main_progress.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) {msg}")
                .unwrap()
                .progress_chars("=>-")
        );
        main_progress.set_message("Inicializando processamento...");

        Self {
            threads: Arc::new(RwLock::new(HashMap::new())),
            multi_progress,
            main_progress,
            thread_progress_bars: Arc::new(RwLock::new(HashMap::new())),
            verbose,
        }
    }

    /// Registra uma nova thread
    pub fn register_thread(&self, thread_id: usize) {
        let mut threads = self.threads.write();
        threads.insert(thread_id, ThreadInfo {
            id: thread_id,
            status: ThreadStatus::Idle,
            start_time: Instant::now(),
            files_processed: 0,
        });

        if self.verbose {
            let thread_pb = self.multi_progress.add(ProgressBar::new(100));
            thread_pb.set_style(
                ProgressStyle::default_bar()
                    .template(&format!("{{spinner:.blue}} Thread {:2} | {{wide_msg}}", thread_id))
                    .unwrap()
                    .progress_chars("⠁⠂⠄⡀⢀⠠⠐⠈")
            );
            thread_pb.set_message("Idle");
            
            let mut progress_bars = self.thread_progress_bars.write();
            progress_bars.insert(thread_id, thread_pb);
        }
    }

    /// Atualiza o status de uma thread
    pub fn update_thread_status(&self, thread_id: usize, status: ThreadStatus) {
        let mut threads = self.threads.write();
        if let Some(thread_info) = threads.get_mut(&thread_id) {
            thread_info.status = status.clone();
            
            if matches!(status, ThreadStatus::ProcessingComplete(_)) {
                thread_info.files_processed += 1;
                self.main_progress.inc(1);
            }
        }

        // Atualizar progress bar da thread se estiver no modo verbose
        if self.verbose {
            let progress_bars = self.thread_progress_bars.read();
            if let Some(pb) = progress_bars.get(&thread_id) {
                let message = match &status {
                    ThreadStatus::Idle => "🔸 Aguardando...".to_string(),
                    ThreadStatus::ScanningFile(file) => {
                        format!("🔍 Escaneando: {}", Self::truncate_filename(file, 30))
                    },
                    ThreadStatus::ExtractingArchive { file, progress } => {
                        format!("📦 Extraindo {} ({:.1}%)", Self::truncate_filename(file, 20), progress)
                    },
                    ThreadStatus::CalculatingCrc(file) => {
                        format!("🔢 CRC32: {}", Self::truncate_filename(file, 30))
                    },
                    ThreadStatus::ProcessingComplete(file) => {
                        format!("✅ Concluído: {}", Self::truncate_filename(file, 25))
                    },
                    ThreadStatus::Error(file, error) => {
                        format!("❌ Erro em {}: {}", Self::truncate_filename(file, 20), Self::truncate_text(error, 20))
                    },
                };
                pb.set_message(message);
                
                // Atualizar progresso para extração de arquivos
                if let ThreadStatus::ExtractingArchive { progress, .. } = status {
                    pb.set_position(progress as u64);
                }
            }
        }
    }

    /// Remove uma thread do monitoramento
    pub fn unregister_thread(&self, thread_id: usize) {
        let mut threads = self.threads.write();
        threads.remove(&thread_id);

        if self.verbose {
            let mut progress_bars = self.thread_progress_bars.write();
            if let Some(pb) = progress_bars.remove(&thread_id) {
                pb.finish_and_clear();
            }
        }
    }

    /// Finaliza o monitoramento
    pub fn finish(&self, message: &str) {
        self.main_progress.finish_with_message(message.to_string());
        
        if self.verbose {
            let progress_bars = self.thread_progress_bars.read();
            for pb in progress_bars.values() {
                pb.finish_and_clear();
            }
        }
    }

    /// Atualiza a mensagem principal
    pub fn set_main_message(&self, message: &str) {
        self.main_progress.set_message(message.to_string());
    }

    /// Obtém estatísticas das threads
    pub fn get_thread_stats(&self) -> (usize, usize, usize) {
        let threads = self.threads.read();
        let total_threads = threads.len();
        let active_threads = threads.values().filter(|t| !matches!(t.status, ThreadStatus::Idle)).count();
        let total_processed = threads.values().map(|t| t.files_processed).sum();
        
        (total_threads, active_threads, total_processed)
    }

    /// Trunca o nome do arquivo para exibição
    fn truncate_filename(path: &str, max_len: usize) -> String {
        let filename = std::path::Path::new(path)
            .file_name()
            .unwrap_or_default()
            .to_string_lossy();
        
        Self::truncate_text(&filename, max_len)
    }

    /// Trunca texto genérico
    fn truncate_text(text: &str, max_len: usize) -> String {
        if text.len() <= max_len {
            text.to_string()
        } else {
            format!("{}...", &text[..max_len.saturating_sub(3)])
        }
    }

    /// Obtém referência ao MultiProgress para uso externo
    pub fn multi_progress(&self) -> &MultiProgress {
        &self.multi_progress
    }
}

impl Drop for ThreadMonitor {
    fn drop(&mut self) {
        self.finish("Monitoramento finalizado");
    }
}
