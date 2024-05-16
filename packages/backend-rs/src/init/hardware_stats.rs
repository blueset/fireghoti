use std::sync::{Mutex, MutexGuard, OnceLock, PoisonError};
use sysinfo::System;

pub type SystemMutexError = PoisonError<MutexGuard<'static, System>>;

// TODO: handle this in a more proper way when we move the entry point to backend-rs
pub fn system() -> Result<MutexGuard<'static, System>, SystemMutexError> {
    pub static SYSTEM: OnceLock<Mutex<System>> = OnceLock::new();
    SYSTEM.get_or_init(|| Mutex::new(System::new_all())).lock()
}

#[crate::export]
pub fn show_server_info() -> Result<(), SystemMutexError> {
    let system_info = system()?;

    tracing::info!(
        "Hostname: {}",
        System::host_name().unwrap_or("unknown".to_string())
    );
    tracing::info!(
        "OS: {}",
        System::long_os_version().unwrap_or("unknown".to_string())
    );
    tracing::info!(
        "Kernel: {}",
        System::kernel_version().unwrap_or("unknown".to_string())
    );
    tracing::info!(
        "CPU architecture: {}",
        System::cpu_arch().unwrap_or("unknown".to_string())
    );
    tracing::info!("CPU threads: {}", system_info.cpus().len());
    tracing::info!("Total memory: {} MiB", system_info.total_memory() / 1048576);
    tracing::info!("Free memory: {} MiB", system_info.free_memory() / 1048576);
    tracing::info!("Total swap: {} MiB", system_info.total_swap() / 1048576);
    tracing::info!("Free swap: {} MiB", system_info.free_swap() / 1048576);

    Ok(())
}
