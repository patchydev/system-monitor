mod plugin;
mod plugins {
    mod default {
        mod memory;
    }
}

use sysinfo::System;

fn main() {
    let system = System::new_all();

    println!("{}", system.free_memory())
}
