use std::time::Duration;

use serde::Serialize;
use sysinfo::{CpuExt, System, SystemExt};
use tokio::sync::broadcast::{self, Sender};

pub mod pages;
pub mod realtime;

#[derive(Clone, Serialize)]
pub struct Snapshot {
    cpu: Vec<(String, f32)>,
}

#[derive(Clone)]
pub struct AppState {
    pub tx: broadcast::Sender<Snapshot>,
}

impl Snapshot {
    pub fn usage_transmitter(tx: Sender<Snapshot>, interval: Duration) -> ! {
        let mut sys = System::new();
        loop {
            sys.refresh_cpu();
            let usages = sys
                .cpus()
                .iter()
                .map(|cpu| (String::from(cpu.name()), cpu.cpu_usage()))
                .collect();

            let snapshot = Snapshot { cpu: usages };
            let _ = tx.send(snapshot);

            std::thread::sleep(interval);
        }
    }
}
