use log::info;
use std::time::Instant;

pub struct SystemMonitor {
	start_time: Instant,
}

impl SystemMonitor {
	pub fn new() -> Self {
		SystemMonitor {
			start_time: Instant::now(),
		}
	}

	pub fn log_uptime(&self) {
		let uptime = self.start_time.elapsed();
		info!("System uptime: {:.2?}", uptime);
	}

	pub fn log_memory_usage(&self) {
		// Placeholder for memory usage logging
		// You can use a crate like `sysinfo` to get actual memory usage
		info!("Memory usage: Placeholder value");
	}

	pub fn log_cpu_usage(&self) {
		// Placeholder for CPU usage logging
		// You can use a crate like `sysinfo` to get actual CPU usage
		info!("CPU usage: Placeholder value");
	}
}