use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

use rand::SeedableRng;
use rand::rngs::SmallRng;
use tokio::task;

use crate::messages::{self, LogLevel};
use crate::rotation::RotatingWriter;

/// Shared state for tracking progress and cancellation.
pub struct GeneratorState {
    pub bytes_written: AtomicU64,
    pub target_bytes: AtomicU64,
    pub running: AtomicBool,
    pub cancel: AtomicBool,
    pub services_total: AtomicU64,
    pub services_done: AtomicU64,
}

impl GeneratorState {
    pub fn new() -> Self {
        Self {
            bytes_written: AtomicU64::new(0),
            target_bytes: AtomicU64::new(0),
            running: AtomicBool::new(false),
            cancel: AtomicBool::new(false),
            services_total: AtomicU64::new(0),
            services_done: AtomicU64::new(0),
        }
    }

    pub fn reset(&self) {
        self.bytes_written.store(0, Ordering::SeqCst);
        self.target_bytes.store(0, Ordering::SeqCst);
        self.running.store(false, Ordering::SeqCst);
        self.cancel.store(false, Ordering::SeqCst);
        self.services_total.store(0, Ordering::SeqCst);
        self.services_done.store(0, Ordering::SeqCst);
    }
}

#[derive(Clone)]
pub struct GeneratorConfig {
    pub num_services: u32,
    pub target_bytes: u64,
    pub file_max_bytes: u64,
    pub output_dir: PathBuf,
    pub service_names: Vec<String>,
}

/// Start log generation. Spawns one blocking task per microservice.
pub fn start_generation(config: GeneratorConfig, state: Arc<GeneratorState>) {
    state.reset();
    state.target_bytes.store(config.target_bytes, Ordering::SeqCst);
    state.running.store(true, Ordering::SeqCst);
    state.services_total.store(config.num_services as u64, Ordering::SeqCst);

    let bytes_per_service = config.target_bytes / config.num_services as u64;

    for i in 0..config.num_services {
        let svc_name = config.service_names[i as usize].clone();
        let svc_dir = config.output_dir.join(&svc_name);
        let file_max = config.file_max_bytes;
        let state = Arc::clone(&state);
        let target = if i == config.num_services - 1 {
            // Last service gets the remainder
            config.target_bytes - bytes_per_service * (config.num_services as u64 - 1)
        } else {
            bytes_per_service
        };

        task::spawn_blocking(move || {
            if let Err(e) = generate_service_logs(&svc_name, &svc_dir, file_max, target, &state) {
                eprintln!("Error generating logs for {}: {}", svc_name, e);
            }
            state.services_done.fetch_add(1, Ordering::SeqCst);

            // If all services are done, mark as not running
            if state.services_done.load(Ordering::SeqCst) >= state.services_total.load(Ordering::SeqCst) {
                state.running.store(false, Ordering::SeqCst);
            }
        });
    }
}

fn generate_service_logs(
    service_name: &str,
    dir: &std::path::Path,
    file_max_bytes: u64,
    target_bytes: u64,
    state: &GeneratorState,
) -> std::io::Result<()> {
    let mut writer = RotatingWriter::new(dir, file_max_bytes)?;
    let mut rng = SmallRng::from_entropy();
    let mut local_bytes: u64 = 0;

    loop {
        // Check cancellation
        if state.cancel.load(Ordering::Relaxed) {
            break;
        }

        // Check if this service has generated enough
        if local_bytes >= target_bytes {
            break;
        }

        // Also check global target (in case of rounding)
        if state.bytes_written.load(Ordering::Relaxed) >= state.target_bytes.load(Ordering::Relaxed) {
            break;
        }

        let level = LogLevel::random(&mut rng);
        let msg = messages::generate_message(&mut rng, level, service_name);
        let written = writer.write_line(&msg)?;

        local_bytes += written as u64;
        state.bytes_written.fetch_add(written as u64, Ordering::Relaxed);
    }

    writer.flush()?;
    Ok(())
}

