use crate::plugin::{Metric, Plugin};
use std::collections::HashMap;
use sysinfo::System;

#[derive(Debug)]
pub struct MemoryPlugin {
    system: System,
    show_swap: bool,
}

impl MemoryPlugin {
    pub fn new() -> Self {
        let mut system = System::new();
        system.refresh_memory();
        Self {
            system,
            show_swap: true,
        }
    }

    fn bytes_to_mb(bytes: u64) -> f64 {
        bytes as f64 / 1024.0 / 1024.0
    }
}

impl Plugin for MemoryPlugin {
    fn name(&self) -> &str {
        "Memory"
    }

    fn id(&self) -> &str {
        "default_memory"
    }

    fn init(&mut self, settings: HashMap<String, String>) -> Result<(), String> {
        if let Some(show_swap) = settings.get("show_swap") {
            self.show_swap = show_swap.to_lowercase() == "true";
        }

        Ok(())
    }

    fn collect(&mut self) -> Vec<Metric> {
        self.system.refresh_memory();

        let mut metrics = Vec::new();

        let total_memory = Self::bytes_to_mb(self.system.total_memory());
        metrics.push(Metric {
            name: "total".to_string(),
            value: total_memory,
            unit: "MB".to_string(),
        });

        let used_memory = Self::bytes_to_mb(self.system.used_memory());
        metrics.push(Metric {
            name: "used".to_string(),
            value: used_memory,
            unit: "MB".to_string(),
        });

        let available_memory = Self::bytes_to_mb(self.system.available_memory());
        metrics.push(Metric {
            name: "available".to_string(),
            value: available_memory,
            unit: "MB".to_string(),
        });

        let memory_percent = (used_memory / total_memory) * 100.0;
        metrics.push(Metric {
            name: "usage".to_string(),
            value: memory_percent,
            unit: "%".to_string(),
        });

        if self.show_swap {
            let total_swap = Self::bytes_to_mb(self.system.total_swap());
            if total_swap > 0.0 {
                metrics.push(Metric {
                    name: "swap_total".to_string(),
                    value: total_swap,
                    unit: "MB".to_string(),
                });

                let used_swap = Self::bytes_to_mb(self.system.used_swap());
                metrics.push(Metric {
                    name: "used_swap".to_string(),
                    value: used_swap,
                    unit: "MB".to_string(),
                });

                let swap_percent = (used_swap / total_swap) * 100.0;
                metrics.push(Metric {
                    name: "swap_usage".to_string(),
                    value: swap_percent,
                    unit: "%".to_string(),
                });
            }
        }

        metrics
    }
}
