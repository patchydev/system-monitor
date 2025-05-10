use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub monitor: SysMonitorConfig,

    #[serde(default)]
    pub plugins: HashMap<String, PluginConfig>,
}

#[derive(Debug, Deserialize, Default)]
pub struct SysMonitorConfig {
    #[serde(default = "default_interval")]
    pub update_inverval: u64,
}

fn default_interval() -> u64 {
    2
}

#[derive(Debug, Deserialize)]
pub struct PluginConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,

    #[serde(default)]
    pub settings: HashMap<String, String>,
}

fn default_enabled() -> bool {
    true
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;

        Ok(config)
    }

    pub fn default() -> Self {
        let mut plugins = HashMap::new();

        let mut memory_settings = HashMap::new();
        memory_settings.insert("show_swap".to_string(), "true".to_string());

        plugins.insert("default_memory".to_string(), PluginConfig {
            enabled: true,
            settings: memory_settings,
        });

        Config {
            monitor: SysMonitorConfig { update_inverval: 2 },
            plugins
        }
    }
}