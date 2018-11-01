//! Show loadavg

use super::{result::*, util::*};
use sigar_sys::*;

/// Load avg info
#[derive(Debug)]
pub struct Load {
    pub avg_1m: f64,
    pub avg_5m: f64,
    pub avg_15m: f64,
}

/// Returns loadavg
pub fn get() -> SigarResult<Load> {
    let raw = ffi_wrap!(sigar_loadavg_get, sigar_loadavg_t)?;
    Ok(Load {
        avg_1m: raw.loadavg[0],
        avg_5m: raw.loadavg[1],
        avg_15m: raw.loadavg[2],
    })
}
