mod config;
mod plugin;
mod plugins;

use std::{collections::HashMap, env, thread, time::Duration};

use config::Config;
use plugin::Registry;
use plugins::default::memory::MemoryPlugin;

fn main() {
    let config_path = env::args()
        .nth(1)
        .unwrap_or_else(|| "config.toml".to_string());

    let config = match Config::from_file(&config_path) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error loading configuration file: {}", e);
            Config::default()
        }
    };

    let mut registry = Registry::new();

    register_plugins(&mut registry, &config);

    let update_interval = config.monitor.update_inverval;

    loop {
        let all = registry.collect_all();

        for (name, metrics) in all {
            println!("\n[{}]", name);

            for metric in metrics {
                println!("{}: {:.2} {}", metric.name, metric.value, metric.unit);
            }
        }

        thread::sleep(Duration::from_secs(update_interval));
    }
}

fn register_plugins(registry: &mut Registry, config: &Config) {
    if let Some(plugin_config) = config.plugins.get("memory_default") {
        if plugin_config.enabled {
            if let Err(e) = registry.register(MemoryPlugin::new(), plugin_config.settings.clone()) {
                eprintln!("Failed to register default memory plugin: {}", e);
            }
        }
    } else {
        registry.register(MemoryPlugin::new(), HashMap::new());
    }
}
