//! Provides methods for controlling process,
//! and gathering resource usages.
//!

use super::{result::*, util::*};
use sigar_sys::*;

// C: sigar_proc_kill
/// Kills a specific process with given process id & signal
pub fn kill(pid: i32, signal: i32) -> SigarResult<()> {
    let res = unsafe { sigar_proc_kill(pid as sigar_pid_t, signal as ::std::os::raw::c_int) };
    if res != SIGAR_CODE_OK {
        return Err(Error::new(std::ptr::null_mut(), res));
    }

    Ok(())
}

// C: sigar_proc_list_get
pub type PIDList = Vec<i32>;

/// Returns pid list
pub fn list() -> SigarResult<PIDList> {
    ffi_wrap_destroy!(
        sigar_proc_list_get,
        sigar_proc_list_destroy,
        sigar_proc_list_t,
        (|list: &sigar_proc_list_t| ffi_extract_list!(list, (|one: &sigar_pid_t| *one as i32)))
    )
}

// C: sigar_proc_stat_get
/// Process summary
#[derive(Debug)]
pub struct ProcessStat {
    pub total: u64,
    pub sleeping: u64,
    pub running: u64,
    pub zombie: u64,
    pub stopped: u64,
    pub idle: u64,
    pub threads: u64,
}

/// Returns summary of all processes
pub fn stat() -> SigarResult<ProcessStat> {
    let raw = ffi_wrap!(sigar_proc_stat_get, sigar_proc_stat_t)?;

    Ok(value_convert!(
        ProcessStat,
        raw,
        total,
        sleeping,
        running,
        zombie,
        stopped,
        idle,
        threads
    ))
}

// C: sigar_proc_mem_get
/// Process memory info
#[derive(Debug)]
pub struct ProcMem {
    pub size: u64,
    pub resident: u64,
    pub share: u64,
    pub minor_faults: u64,
    pub major_faults: u64,
    pub page_faults: u64,
}

/// Returns memory usage for a given pid
pub fn mem(pid: i32) -> SigarResult<ProcMem> {
    let raw = ffi_wrap!(sigar_proc_mem_get, (pid as sigar_pid_t), sigar_proc_mem_t)?;
    Ok(value_convert!(
        ProcMem,
        raw,
        size,
        resident,
        share,
        minor_faults,
        major_faults,
        page_faults,
    ))
}

// C: sigar_proc_disk_io_get
// C: sigar_proc_cumulative_disk_io_get
// C: sigar_proc_cred_get
// C: sigar_proc_cred_name_get
// C: sigar_proc_time_get
// C: sigar_proc_cpu_get
// C: sigar_proc_state_get
// C: sigar_proc_args_get
// C: sigar_proc_args_destroy
// C: sigar_proc_env_get
// C: sigar_proc_fd_get
// C: sigar_proc_exe_get
// C: sigar_proc_modules_get
// C: sigar_proc_port_get
