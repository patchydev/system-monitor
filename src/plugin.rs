use std::{collections::HashMap, fmt::Debug};

#[derive(Debug, Clone)]
pub struct Metric {
    pub name: String,
    pub value: f64,
    pub unit: String,
}

pub trait Plugin: Debug {
    fn name(&self) -> &str;
    fn id(&self) -> &str;
    fn init(&mut self, settings: HashMap<String, String>) -> Result<(), String> {
        Ok(())
    }
    fn collect(&mut self) -> Vec<Metric>;
}

#[derive(Debug, Default)]
pub struct Registry {
    plugins: Vec<Box<dyn Plugin>>,
}

impl Registry {
    pub fn new() -> Self {
        Self { plugins: Vec::new() }
    }

    pub fn register<P: Plugin + 'static>(&mut self, mut plugin: P, settings: HashMap<String, String>) -> Result<(), String> {
        if let Err(e) = plugin.init(settings) {
            return Err(format!("Failed to initialize plugin {}: {}", plugin.name(), e))
        }
        self.plugins.push(Box::new(plugin));
        Ok(())
    }

    pub fn collect_all(&mut self) -> Vec<(String, Vec<Metric>)> {
        let mut results = Vec::new();

        for plugin in &mut self.plugins {
            let name = plugin.name().to_string();
            let metrics = plugin.collect();

            if !metrics.is_empty() {
                results.push((name, metrics));
            }
        }

        results
    }
}