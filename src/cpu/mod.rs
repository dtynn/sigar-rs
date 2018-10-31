//! Provides methods for gathering cpu informations
//!

use super::result::*;
use sigar_sys as ffi;
use std::{self, slice::from_raw_parts};

/// CPU usage
#[derive(Debug)]
pub struct CPU {
    /// user ticks
    pub user: u64,

    /// system ticks
    pub sys: u64,

    /// nice ticks
    pub nice: u64,

    /// idle ticks
    pub idle: u64,

    /// wait ticks
    pub wait: u64,

    /// irq ticks
    pub irq: u64,

    /// sort_irq ticks
    pub soft_irq: u64,

    /// stolen ticks
    pub stolen: u64,

    /// total ticks
    pub total: u64,
}

/// Returns cpu usage
pub fn get() -> SigarResult<CPU> {
    let cpu = ffi_wrap!(sigar_cpu_get, sigar_cpu_t)?;
    Ok(value_convert!(
        CPU, cpu, user, sys, nice, idle, wait, irq, soft_irq, stolen, total
    ))
}

/// CPU usage list
#[derive(Debug, Default)]
pub struct CPUList {
    /// cpu core number
    pub number: u64,

    /// cpu core size
    pub size: u64,

    /// cpu usage for each core
    pub data: Vec<CPU>,
}

/// Returns cpu usage list
pub fn list() -> SigarResult<CPUList> {
    ffi_wrap_destroy!(
        sigar_cpu_list_get,
        sigar_cpu_list_destroy,
        sigar_cpu_list_t,
        list_trans
    )
}

fn list_trans(info: ffi::sigar_cpu_list_t) -> CPUList {
    let mut cpulist = value_convert!(
        CPUList,
        info,
        number,
        size,
        (data: Vec::with_capacity(info.number as usize)),
    );

    let data = unsafe { from_raw_parts(info.data, info.number as usize) };
    for one in data {
        cpulist.data.push(value_convert!(
            CPU, one, user, sys, nice, idle, wait, irq, soft_irq, stolen, total
        ));
    }

    cpulist
}
