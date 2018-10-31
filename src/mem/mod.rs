//! Provides methods for gathering memory informations
//!

use super::result::*;
use sigar_sys::*;

/// Memory information
#[derive(Debug)]
pub struct Mem {
    pub ram: u64,
    pub total: u64,
    pub used: u64,
    pub free: u64,
    pub actual_used: u64,
    pub actual_free: u64,
    pub used_percent: f64,
    pub free_percent: f64,
}

/// Returns memory information
pub fn get() -> SigarResult<Mem> {
    let raw = ffi_wrap!(sigar_mem_get, sigar_mem_t)?;
    Ok(value_convert!(
        Mem,
        raw,
        ram,
        total,
        used,
        free,
        actual_used,
        actual_free,
        used_percent,
        free_percent,
    ))
}
