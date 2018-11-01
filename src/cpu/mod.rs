//! Provides methods for gathering cpu informations
//!

use super::{result::*, util::*};
use sigar_sys::*;
use std;

/// CPU usage
#[derive(Debug)]
pub struct Usage {
    pub user: u64,
    pub sys: u64,
    pub nice: u64,
    pub idle: u64,
    pub wait: u64,
    pub irq: u64,
    pub soft_irq: u64,
    pub stolen: u64,
    pub total: u64,
}

impl Usage {
    fn from_raw(raw: &sigar_cpu_t) -> Self {
        value_convert!(Usage, raw, user, sys, nice, idle, wait, irq, soft_irq, stolen, total)
    }

    fn to_raw(&self) -> sigar_cpu_t {
        value_convert!(
            sigar_cpu_t,
            self,
            user,
            sys,
            nice,
            idle,
            wait,
            irq,
            soft_irq,
            stolen,
            total
        )
    }
}

/// Returns cpu usage
pub fn get() -> SigarResult<Usage> {
    let raw = ffi_wrap!(sigar_cpu_get, sigar_cpu_t)?;
    Ok(Usage::from_raw(&raw))
}

/// CPU usage list
pub type UsageList = Vec<Usage>;

/// Returns cpu usage list
pub fn list() -> SigarResult<UsageList> {
    ffi_wrap_destroy!(
        sigar_cpu_list_get,
        sigar_cpu_list_destroy,
        sigar_cpu_list_t,
        (|list: &sigar_cpu_list_t| ffi_extract_list!(list, Usage::from_raw))
    )
}

/// CPU informations
#[derive(Debug)]
pub struct Info {
    pub vendor: String,
    pub model: String,
    pub mhz: i32,
    pub mhz_max: i32,
    pub mhz_min: i32,
    pub cache_size: u64,
    pub total_sockets: i32,
    pub total_cores: i32,
    pub cores_per_socket: i32,
}

/// CPU info list
pub type InfoList = Vec<Info>;

/// Returns cpu info list
pub fn info_list() -> SigarResult<InfoList> {
    ffi_wrap_destroy!(
        sigar_cpu_info_list_get,
        sigar_cpu_info_list_destroy,
        sigar_cpu_info_list_t,
        (|list: &sigar_cpu_info_list_t| ffi_extract_list!(
            list,
            (|one: &sigar_cpu_info_t| value_convert!(
            Info,
            one,
            mhz,
            mhz_max,
            mhz_min,
            cache_size,
            total_sockets,
            total_cores,
            cores_per_socket,
            (vendor: must_chars_to_string(&one.vendor[..])),
            (model: must_chars_to_string(&one.model[..])),
        ))
        ))
    )
}

/// CPU usage percentage
pub type UsagePercent = sigar_cpu_perc_t;

pub fn calc_percent(prev: &Usage, curr: &Usage) -> UsagePercent {
    let mut perc = Default::default();
    unsafe {
        sigar_cpu_perc_calculate(&mut prev.to_raw(), &mut curr.to_raw(), &mut perc);
    };

    perc
}
