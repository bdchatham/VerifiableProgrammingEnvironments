mod metric;

use sysinfo::{System, SystemExt, CpuExt, DiskExt};
use std::collections::HashMap;

pub struct MetricCollector {
    system: System,
}

impl MetricCollector {
    pub fn new() -> Self {
        Self {
            system: System::new_all(),
        }
    }

    /// Collects system metrics and returns them as a HashMap.
    pub fn collect_metrics(&mut self) -> HashMap<&'static str, u64> {
        self.system.refresh_all();

        let cpu_usage = (self.system.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>()
            / self.system.cpus().len() as f32) as u64;

        let memory_used = self.system.used_memory();  // in KB
        let memory_total = self.system.total_memory(); // in KB

        let disk = self.system.disks().first().unwrap();
        let disk_available = disk.available_space() / 1024; // in KB
        let disk_total = disk.total_space() / 1024; // in KB

        let mut metrics = HashMap::new();
        metrics.insert("cpu_usage_percent", cpu_usage);
        metrics.insert("memory_used_kb", memory_used);
        metrics.insert("memory_total_kb", memory_total);
        metrics.insert("disk_available_kb", disk_available);
        metrics.insert("disk_total_kb", disk_total);

        metrics
    }
}
