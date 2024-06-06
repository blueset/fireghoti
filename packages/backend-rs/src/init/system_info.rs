use std::sync::{Mutex, MutexGuard, OnceLock, PoisonError};
use sysinfo::System;

pub type SysinfoPoisonError = PoisonError<MutexGuard<'static, System>>;

static SYSTEM_INFO: OnceLock<Mutex<System>> = OnceLock::new();

/// Gives an access to the shared static [System] object.
///
/// # Example
///
/// ```
/// # use backend_rs::init::system_info::{system_info, SysinfoPoisonError};
/// let system_info = system_info().lock()?;
/// println!("The number of CPU threads is {}.", system_info.cpus().len());
/// # Ok::<(), SysinfoPoisonError>(())
/// ```
pub fn system_info() -> &'static std::sync::Mutex<System> {
    SYSTEM_INFO.get_or_init(|| Mutex::new(System::new_all()))
}

/// Prints the server hardware information as the server info log.
#[crate::export]
pub fn show_server_info() -> Result<(), SysinfoPoisonError> {
    let system_info = system_info().lock()?;

    tracing::info!(
        "Hostname: {}",
        System::host_name().unwrap_or_else(|| "unknown".to_string())
    );
    tracing::info!(
        "OS: {}",
        System::long_os_version().unwrap_or_else(|| "unknown".to_string())
    );
    tracing::info!(
        "Kernel: {}",
        System::kernel_version().unwrap_or_else(|| "unknown".to_string())
    );
    tracing::info!(
        "CPU architecture: {}",
        System::cpu_arch().unwrap_or_else(|| "unknown".to_string())
    );
    tracing::info!("CPU threads: {}", system_info.cpus().len());
    tracing::info!("Total memory: {} MiB", system_info.total_memory() / 1048576);
    tracing::info!("Free memory: {} MiB", system_info.free_memory() / 1048576);
    tracing::info!("Total swap: {} MiB", system_info.total_swap() / 1048576);
    tracing::info!("Free swap: {} MiB", system_info.free_swap() / 1048576);

    Ok(())
}
