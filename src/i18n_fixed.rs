use std::collections::HashMap;
use std::fs;
use std::path::Path;
use anyhow::Result;
use std::sync::{Arc, RwLock, OnceLock};
use fluent::{FluentBundle, FluentResource};
use unic_langid::{LanguageIdentifier, langid};

type ThreadSafeFluentBundle = FluentBundle<FluentResource>;

pub struct I18n {
    bundles: HashMap<LanguageIdentifier, ThreadSafeFluentBundle>,
    current_locale: LanguageIdentifier,
    fallback_locale: LanguageIdentifier,
}

impl I18n {
    pub fn new() -> Result<Self> {
        let mut i18n = Self {
            bundles: HashMap::new(),
            current_locale: langid!("pt"),
            fallback_locale: langid!("en"),
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
                            if let Ok(locale_id) = locale_str.parse::<LanguageIdentifier>() {
                                self.load_locale_file(&path, locale_id)?;
                            }
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn load_locale_file(&mut self, path: &Path, locale_id: LanguageIdentifier) -> Result<()> {
        let content = fs::read_to_string(path)?;
        let resource = FluentResource::try_new(content)
            .map_err(|_| anyhow::anyhow!("Failed to parse Fluent resource for locale: {}", locale_id))?;
        
        let mut bundle = FluentBundle::new(vec![locale_id.clone()]);
        bundle.add_resource(resource)
            .map_err(|_| anyhow::anyhow!("Failed to add resource to bundle for locale: {}", locale_id))?;
        
        self.bundles.insert(locale_id, bundle);
        Ok(())
    }
    
    fn load_embedded_locales(&mut self) -> Result<()> {
        // Fallback implementation with embedded strings
        // This will be used if the locales directory is not found
        self.load_embedded_locale(langid!("en"), include_str!("../locales/en.ftl"))?;
        self.load_embedded_locale(langid!("pt"), include_str!("../locales/pt.ftl"))?;
        Ok(())
    }
    
    fn load_embedded_locale(&mut self, locale_id: LanguageIdentifier, content: &str) -> Result<()> {
        let resource = FluentResource::try_new(content.to_string())
            .map_err(|_| anyhow::anyhow!("Failed to parse embedded Fluent resource for locale: {}", locale_id))?;
        
        let mut bundle = FluentBundle::new(vec![locale_id.clone()]);
        bundle.add_resource(resource)
            .map_err(|_| anyhow::anyhow!("Failed to add embedded resource to bundle for locale: {}", locale_id))?;
        
        self.bundles.insert(locale_id, bundle);
        Ok(())
    }
    
    pub fn set_locale(&mut self, locale: &str) -> Result<()> {
        let locale_id: LanguageIdentifier = locale.parse()?;
        if self.bundles.contains_key(&locale_id) {
            self.current_locale = locale_id;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Locale not found: {}", locale))
        }
    }
    
    pub fn get(&self, key: &str) -> String {
        self.get_with_args(key, None)
    }
    
    pub fn get_with_args(&self, key: &str, args: Option<&HashMap<String, String>>) -> String {
        // Try current locale first
        if let Some(bundle) = self.bundles.get(&self.current_locale) {
            if let Some(message) = bundle.get_message(key) {
                if let Some(pattern) = message.value() {
                    let mut errors = Vec::new();
                    let fluent_args = args.map(|args_map| {
                        args_map.iter().map(|(k, v)| (k.as_str(), fluent::FluentValue::from(v.as_str()))).collect()
                    });
                    
                    let formatted = bundle.format_pattern(pattern, fluent_args.as_ref(), &mut errors);
                    if errors.is_empty() {
                        return formatted.to_string();
                    }
                }
            }
        }
        
        // Fallback to fallback locale
        if let Some(bundle) = self.bundles.get(&self.fallback_locale) {
            if let Some(message) = bundle.get_message(key) {
                if let Some(pattern) = message.value() {
                    let mut errors = Vec::new();
                    let fluent_args = args.map(|args_map| {
                        args_map.iter().map(|(k, v)| (k.as_str(), fluent::FluentValue::from(v.as_str()))).collect()
                    });
                    
                    let formatted = bundle.format_pattern(pattern, fluent_args.as_ref(), &mut errors);
                    if errors.is_empty() {
                        return formatted.to_string();
                    }
                }
            }
        }
        
        // Final fallback to key itself
        key.to_string()
    }
    
    pub fn get_current_locale(&self) -> &LanguageIdentifier {
        &self.current_locale
    }
    
    pub fn available_locales(&self) -> Vec<&LanguageIdentifier> {
        self.bundles.keys().collect()
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
        i18n.get_with_args(key, Some(args))
    } else {
        key.to_string()
    }
}

pub fn t_count(key: &str, count: usize) -> String {
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
        i18n.available_locales().iter().map(|id| id.to_string()).collect()
    } else {
        vec!["pt".to_string(), "en".to_string()]
    }
}
