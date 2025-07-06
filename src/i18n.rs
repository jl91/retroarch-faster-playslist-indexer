use std::collections::HashMap;
use std::fs;
use std::path::Path;
use anyhow::Result;
use std::sync::{Arc, RwLock, OnceLock};

#[derive(Debug, Clone)]
pub struct I18n {
    translations: HashMap<String, HashMap<String, String>>,
    current_locale: String,
    fallback_locale: String,
}

impl I18n {
    pub fn new() -> Result<Self> {
        let mut i18n = Self {
            translations: HashMap::new(),
            current_locale: "pt".to_string(),
            fallback_locale: "en".to_string(),
        };
        
        i18n.load_locales()?;
        Ok(i18n)
    }
    
    fn load_locales(&mut self) -> Result<()> {
        let locales_dir = Path::new("locales");
        
        if !locales_dir.exists() {
            // Fallback to embedded locales if locales directory doesn't exist
            return self.load_embedded_locales();
        }
        
        // Load all .ftl files from locales directory
        for entry in fs::read_dir(locales_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if let Some(extension) = path.extension() {
                if extension == "ftl" {
                    if let Some(file_stem) = path.file_stem() {
                        if let Some(locale_str) = file_stem.to_str() {
                            self.load_locale_file(&path, locale_str)?;
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn load_locale_file(&mut self, path: &Path, locale: &str) -> Result<()> {
        let content = fs::read_to_string(path)?;
        let translations = self.parse_ftl_content(&content)?;
        self.translations.insert(locale.to_string(), translations);
        Ok(())
    }
    
    fn load_embedded_locales(&mut self) -> Result<()> {
        // Fallback implementation with embedded strings for all supported languages
        let en_content = include_str!("../locales/en.ftl");
        let pt_content = include_str!("../locales/pt.ftl");
        let es_content = include_str!("../locales/es.ftl");
        let fr_content = include_str!("../locales/fr.ftl");
        let de_content = include_str!("../locales/de.ftl");
        let ja_content = include_str!("../locales/ja.ftl");
        let ru_content = include_str!("../locales/ru.ftl");
        let zh_content = include_str!("../locales/zh.ftl");
        
        self.translations.insert("en".to_string(), self.parse_ftl_content(en_content)?);
        self.translations.insert("pt".to_string(), self.parse_ftl_content(pt_content)?);
        self.translations.insert("es".to_string(), self.parse_ftl_content(es_content)?);
        self.translations.insert("fr".to_string(), self.parse_ftl_content(fr_content)?);
        self.translations.insert("de".to_string(), self.parse_ftl_content(de_content)?);
        self.translations.insert("ja".to_string(), self.parse_ftl_content(ja_content)?);
        self.translations.insert("ru".to_string(), self.parse_ftl_content(ru_content)?);
        self.translations.insert("zh".to_string(), self.parse_ftl_content(zh_content)?);
        
        Ok(())
    }
    
    fn parse_ftl_content(&self, content: &str) -> Result<HashMap<String, String>> {
        let mut translations = HashMap::new();
        
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            if let Some(eq_pos) = line.find('=') {
                let key = line[..eq_pos].trim().to_string();
                let value = line[eq_pos + 1..].trim().to_string();
                translations.insert(key, value);
            }
        }
        
        Ok(translations)
    }
    
    pub fn set_locale(&mut self, locale: &str) -> Result<()> {
        if self.translations.contains_key(locale) {
            self.current_locale = locale.to_string();
            Ok(())
        } else {
            Err(anyhow::anyhow!("Locale not found: {}", locale))
        }
    }
    
    pub fn get(&self, key: &str) -> String {
        self.get_with_args(key, &HashMap::new())
    }
    
    pub fn get_with_args(&self, key: &str, args: &HashMap<String, String>) -> String {
        // Try current locale first
        if let Some(locale_translations) = self.translations.get(&self.current_locale) {
            if let Some(template) = locale_translations.get(key) {
                return self.format_message(template, args);
            }
        }
        
        // Fallback to fallback locale
        if let Some(locale_translations) = self.translations.get(&self.fallback_locale) {
            if let Some(template) = locale_translations.get(key) {
                return self.format_message(template, args);
            }
        }
        
        // Final fallback to key itself
        key.to_string()
    }
    
    fn format_message(&self, template: &str, args: &HashMap<String, String>) -> String {
        let mut result = template.to_string();
        
        for (key, value) in args {
            let placeholder = format!("{{ ${} }}", key);
            result = result.replace(&placeholder, value);
        }
        
        result
    }
    
    pub fn get_current_locale(&self) -> &str {
        &self.current_locale
    }
    
    pub fn available_locales(&self) -> Vec<&str> {
        self.translations.keys().map(|s| s.as_str()).collect()
    }
}

static I18N_INSTANCE: OnceLock<Arc<RwLock<I18n>>> = OnceLock::new();

pub fn init() -> Result<()> {
    let i18n = I18n::new()?;
    let _ = I18N_INSTANCE.set(Arc::new(RwLock::new(i18n)));
    Ok(())
}

pub fn set_locale(locale: &str) -> Result<()> {
    if let Some(instance) = I18N_INSTANCE.get() {
        let mut i18n = instance.write().unwrap();
        i18n.set_locale(locale)
    } else {
        Err(anyhow::anyhow!("I18n not initialized"))
    }
}

pub fn t(key: &str) -> String {
    if let Some(instance) = I18N_INSTANCE.get() {
        let i18n = instance.read().unwrap();
        i18n.get(key)
    } else {
        key.to_string()
    }
}

pub fn t_with_arg(key: &str, arg_value: &str) -> String {
    let mut args = HashMap::new();
    args.insert("arg".to_string(), arg_value.to_string());
    t_with_args(key, &args)
}

pub fn t_with_args(key: &str, args: &HashMap<String, String>) -> String {
    if let Some(instance) = I18N_INSTANCE.get() {
        let i18n = instance.read().unwrap();
        i18n.get_with_args(key, args)
    } else {
        key.to_string()
    }
}

pub fn t_count(key: &str, count: i32) -> String {
    let mut args = HashMap::new();
    args.insert("count".to_string(), count.to_string());
    t_with_args(key, &args)
}

pub fn t_path(key: &str, path: &str) -> String {
    let mut args = HashMap::new();
    args.insert("path".to_string(), path.to_string());
    t_with_args(key, &args)
}

pub fn t_filename(key: &str, filename: &str) -> String {
    let mut args = HashMap::new();
    args.insert("filename".to_string(), filename.to_string());
    t_with_args(key, &args)
}

pub fn t_progress(key: &str, progress: i32) -> String {
    let mut args = HashMap::new();
    args.insert("progress".to_string(), progress.to_string());
    t_with_args(key, &args)
}

pub fn t_platform(key: &str, platform: &str) -> String {
    let mut args = HashMap::new();
    args.insert("platform".to_string(), platform.to_string());
    t_with_args(key, &args)
}

pub fn t_conversion(source: &str, target: &str) -> String {
    let mut args = HashMap::new();
    args.insert("source".to_string(), source.to_string());
    args.insert("target".to_string(), target.to_string());
    t_with_args("conversion", &args)
}

pub fn t_directory_progress(current: usize, total: usize, path: &str) -> String {
    let mut args = HashMap::new();
    args.insert("current".to_string(), current.to_string());
    args.insert("total".to_string(), total.to_string());
    args.insert("path".to_string(), path.to_string());
    t_with_args("scanning-directory-progress", &args)
}

pub fn get_current_locale() -> String {
    if let Some(instance) = I18N_INSTANCE.get() {
        let i18n = instance.read().unwrap();
        i18n.get_current_locale().to_string()
    } else {
        "pt".to_string()
    }
}

pub fn available_locales() -> Vec<String> {
    if let Some(instance) = I18N_INSTANCE.get() {
        let i18n = instance.read().unwrap();
        i18n.available_locales().iter().map(|s| s.to_string()).collect()
    } else {
        vec!["pt".to_string(), "en".to_string()]
    }
}

pub fn t_with_array_args(key: &str, args: &[&str]) -> String {
    let mut args_map = HashMap::new();
    for (i, value) in args.iter().enumerate() {
        args_map.insert(format!("arg{}", i), value.to_string());
    }
    t_with_args(key, &args_map)
}

pub fn t_validation_valid(count: i32, percentage: f64) -> String {
    let mut args = HashMap::new();
    args.insert("count".to_string(), count.to_string());
    args.insert("percentage".to_string(), format!("{:.1}", percentage));
    t_with_args("validation-valid", &args)
}

pub fn t_command(key: &str, command: &str) -> String {
    let mut args = HashMap::new();
    args.insert("command".to_string(), command.to_string());
    t_with_args(key, &args)
}

pub fn t_error(key: &str, error: &str) -> String {
    let mut args = HashMap::new();
    args.insert("error".to_string(), error.to_string());
    t_with_args(key, &args)
}

pub fn t_type(key: &str, type_value: &str) -> String {
    let mut args = HashMap::new();
    args.insert("type".to_string(), type_value.to_string());
    t_with_args(key, &args)
}

pub fn t_system(key: &str, system: &str) -> String {
    let mut args = HashMap::new();
    args.insert("system".to_string(), system.to_string());
    t_with_args(key, &args)
}

pub fn t_systems(key: &str, systems: &str) -> String {
    let mut args = HashMap::new();
    args.insert("systems".to_string(), systems.to_string());
    t_with_args(key, &args)
}

pub fn t_rom_system(key: &str, rom: &str, system: &str) -> String {
    let mut args = HashMap::new();
    args.insert("rom".to_string(), rom.to_string());
    args.insert("system".to_string(), system.to_string());
    t_with_args(key, &args)
}

pub fn t_count_time(key: &str, count: usize, time: f32) -> String {
    let mut args = HashMap::new();
    args.insert("count".to_string(), count.to_string());
    args.insert("time".to_string(), format!("{:.2}", time));
    t_with_args(key, &args)
}

pub fn t_directory_scan(key: &str, current: usize, total: usize, path: &str) -> String {
    let mut args = HashMap::new();
    args.insert("current".to_string(), current.to_string());
    args.insert("total".to_string(), total.to_string());
    args.insert("path".to_string(), path.to_string());
    t_with_args(key, &args)
}
