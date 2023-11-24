use std::time::Duration;

use serde::Serialize;
use sysinfo::{CpuExt, System, SystemExt};
use tokio::sync::broadcast::Sender;

pub mod pages;
pub mod realtime;

#[derive(Clone, Serialize)]
struct CPU {
    name: String,
    usage: f64,
}

#[derive(Clone, Serialize)]
struct Memory {
    total: u64,
    used: u64,
    free: u64,
}

#[derive(Clone, Serialize)]
pub struct Snapshot {
    cpus: Vec<CPU>,
    memory: Memory,
}

#[derive(Clone)]
pub struct AppState {
    pub tx: Sender<Snapshot>,
}

impl Snapshot {
    pub fn usage_transmitter(tx: Sender<Snapshot>, interval: Duration) -> ! {
        let mut sys = System::new();
        loop {
            sys.refresh_cpu();
            let cpus = sys.cpus().iter()
                .map(|cpu| CPU {
                    name: cpu.name().to_string(),
                    usage: cpu.cpu_usage() as f64,
                })
                .collect();

            sys.refresh_memory();
            let memory = Memory {
                total: sys.total_memory(),
                used: sys.used_memory(),
                free: sys.free_memory(),
            };

            let _ = tx.send(Snapshot {
                cpus,
                memory
            });

            std::thread::sleep(interval);
        }
    }
}
