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
