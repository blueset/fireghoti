//! Utilities to check hardware information such as cpu, memory, storage usage

use crate::init::system_info::{system_info, SysinfoPoisonError};
use sysinfo::{Disks, MemoryRefreshKind};

// TODO: i64 -> u64 (we can't export u64 to Node.js)

#[macros::export(object)]
pub struct Cpu {
    pub model: String,
    // TODO: u16 -> usize (we can't export usize to Node.js)
    pub cores: u16,
}

#[macros::export(object)]
pub struct Memory {
    /// Total memory amount in bytes
    pub total: i64,
    /// Used memory amount in bytes
    pub used: i64,
    /// Available (for (re)use) memory amount in bytes
    pub available: i64,
}

#[macros::export(object)]
pub struct Storage {
    /// Total storage space in bytes
    pub total: i64,
    /// Used storage space in bytes
    pub used: i64,
}

#[macros::export]
pub fn cpu_info() -> Result<Cpu, SysinfoPoisonError> {
    let system_info = system_info().lock()?;

    Ok(Cpu {
        model: match system_info.cpus() {
            [] => {
                tracing::debug!("failed to get CPU info");
                "unknown".to_owned()
            }
            cpus => cpus[0].brand().to_owned(),
        },
        cores: system_info.cpus().len() as u16,
    })
}

#[macros::export]
pub fn cpu_usage() -> Result<f32, SysinfoPoisonError> {
    let mut system_info = system_info().lock()?;
    system_info.refresh_cpu_usage();

    let total_cpu_usage: f32 = system_info.cpus().iter().map(|cpu| cpu.cpu_usage()).sum();
    let cpu_threads = system_info.cpus().len();

    Ok(total_cpu_usage / (cpu_threads as f32))
}

#[macros::export]
pub fn memory_usage() -> Result<Memory, SysinfoPoisonError> {
    let mut system_info = system_info().lock()?;

    system_info.refresh_memory_specifics(MemoryRefreshKind::new().with_ram());

    Ok(Memory {
        total: system_info.total_memory() as i64,
        used: system_info.used_memory() as i64,
        available: system_info.available_memory() as i64,
    })
}

#[macros::export]
pub fn storage_usage() -> Option<Storage> {
    // Get the first disk that is actualy used (has available space & has at least 1 GB total space).
    let disks = Disks::new_with_refreshed_list();
    let disk = disks
        .iter()
        .find(|disk| disk.available_space() > 0 && disk.total_space() > 1024 * 1024 * 1024)?;

    let total = disk.total_space() as i64;
    let available = disk.available_space() as i64;

    Some(Storage {
        total,
        used: total - available,
    })
}
