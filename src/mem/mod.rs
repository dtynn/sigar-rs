//! Provides methods for gathering memory informations
//!

use super::result::*;
use sigar_sys::*;

/// Memory information
#[derive(Debug)]
pub struct Mem {
    /// total ram, unit: MiB
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

/// Swap usage
#[derive(Debug)]
pub struct Swap {
    pub total: u64,
    pub used: u64,
    pub free: u64,
    pub page_in: u64,
    pub page_out: u64,
}

/// Returns swap usage
pub fn swap() -> SigarResult<Swap> {
    let raw = ffi_wrap!(sigar_swap_get, sigar_swap_t)?;
    Ok(value_convert!(
        Swap, raw, total, used, free, page_in, page_out,
    ))
}
