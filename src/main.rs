mod plugin;
mod plugins;

use plugin::Registry;
use plugins::default::memory::MemoryPlugin;

fn main() {
    let mut registry = Registry::new();

    registry.register(MemoryPlugin::new());

    let all = registry.collect_all();

    for (name, metrics) in all {
        println!("[{}]", name);

        for metric in metrics {
            println!("{}: {:.2} {}", metric.name, metric.value, metric.unit);
        }
    }
}
