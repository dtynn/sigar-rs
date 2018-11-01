//! Provides methods for controlling process,
//! and gathering resource usages.
//!

use super::{result::*, util::*};
use sigar_sys::*;

/// Returns pid for current process
pub fn current_pid() -> SigarResult<u32> {
    ffi_wrap_sigar_t!((|ptr_t| unsafe { sigar_pid_get(ptr_t) as u32 }))
}

// C: sigar_proc_kill
/// Kills a specific process with given process id & signal
pub fn kill(pid: u32, signal: i32) -> SigarResult<()> {
    let res = unsafe { sigar_proc_kill(pid as sigar_pid_t, signal as ::std::os::raw::c_int) };
    if res != SIGAR_CODE_OK {
        let reason = ffi_wrap_sigar_t!((|ptr_t| error_string(ptr_t, res.into())))?;
        return Err(Error::from_string(reason));
    }

    Ok(())
}

// C: sigar_proc_list_get
pub type PIDList = Vec<u32>;

/// Returns pid list
pub fn list() -> SigarResult<PIDList> {
    ffi_wrap_destroy!(
        sigar_proc_list_get,
        sigar_proc_list_destroy,
        sigar_proc_list_t,
        (|list: &sigar_proc_list_t| ffi_extract_list!(list, (|one: &sigar_pid_t| *one as u32)))
    )
}

// C: sigar_proc_stat_get
/// Process summary
#[derive(Debug)]
pub struct Summary {
    pub total: u64,
    pub sleeping: u64,
    pub running: u64,
    pub zombie: u64,
    pub stopped: u64,
    pub idle: u64,
    pub threads: u64,
}

/// Returns summary of all processes
pub fn summary() -> SigarResult<Summary> {
    let raw = ffi_wrap!(sigar_proc_stat_get, sigar_proc_stat_t)?;

    Ok(value_convert!(
        Summary, raw, total, sleeping, running, zombie, stopped, idle, threads
    ))
}

// C: sigar_proc_mem_get
/// Process memory info
#[derive(Debug, Default, Copy, Clone)]
pub struct Mem {
    pub size: u64,
    pub resident: u64,
    pub share: u64,
    pub minor_faults: u64,
    pub major_faults: u64,
    pub page_faults: u64,
}

/// Returns memory usage for given pid
pub fn mem(pid: u32) -> SigarResult<Mem> {
    let raw = ffi_wrap!(sigar_proc_mem_get, (pid as sigar_pid_t), sigar_proc_mem_t)?;
    Ok(value_convert!(
        Mem,
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
/// Disk IO info
#[derive(Debug)]
pub struct DiskIO {
    pub bytes_read: u64,
    pub bytes_written: u64,
    pub bytes_total: u64,
}

/// Returns disk io for given pid
pub fn disk_io(pid: u32) -> SigarResult<DiskIO> {
    let raw = ffi_wrap!(
        sigar_proc_disk_io_get,
        (pid as sigar_pid_t),
        sigar_proc_disk_io_t
    )?;

    Ok(value_convert!(
        DiskIO,
        raw,
        bytes_read,
        bytes_written,
        bytes_total,
    ))
}

// C: sigar_proc_cumulative_disk_io_get

/// Returns cumulative disk io for given pid
pub fn cum_disk_io(pid: u32) -> SigarResult<DiskIO> {
    let raw = ffi_wrap!(
        sigar_proc_cumulative_disk_io_get,
        (pid as sigar_pid_t),
        sigar_proc_cumulative_disk_io_t
    )?;

    Ok(value_convert!(
        DiskIO,
        raw,
        bytes_read,
        bytes_written,
        bytes_total,
    ))
}

// C: sigar_proc_cred_get

/// Process cred
#[derive(Debug)]
pub struct Cred {
    pub uid: u32,
    pub gid: u32,
    pub euid: u32,
    pub egid: u32,
}

/// Returns creds for given pid
pub fn cred(pid: u32) -> SigarResult<Cred> {
    let raw = ffi_wrap!(sigar_proc_cred_get, (pid as sigar_pid_t), sigar_proc_cred_t)?;

    Ok(value_convert!(Cred, raw, uid, gid, euid, egid))
}

// C: sigar_proc_cred_name_get
/// Process cred name
#[derive(Debug)]
pub struct CredName {
    pub user: Vec<u8>,
    pub group: Vec<u8>,
}

pub fn cred_name(pid: u32) -> SigarResult<CredName> {
    let raw = ffi_wrap!(
        sigar_proc_cred_name_get,
        (pid as sigar_pid_t),
        sigar_proc_cred_name_t
    )?;

    Ok(CredName {
        user: chars_to_bytes(&raw.user[..]),
        group: chars_to_bytes(&raw.group[..]),
    })
}

// C: sigar_proc_time_get
/// Process time
#[derive(Debug)]
pub struct Time {
    pub start_time: u64,
    pub user: u64,
    pub sys: u64,
    pub total: u64,
}

/// Returns process time for given pid
pub fn time(pid: u32) -> SigarResult<Time> {
    let raw = ffi_wrap!(sigar_proc_time_get, (pid as sigar_pid_t), sigar_proc_time_t)?;

    Ok(value_convert!(Time, raw, start_time, user, sys, total))
}

// C: sigar_proc_cpu_get
/// Process cpu usage
#[derive(Debug)]
pub struct CPU {
    pub start_time: u64,
    pub user: u64,
    pub sys: u64,
    pub total: u64,
    pub last_time: u64,
    pub percent: f64,
}

/// Returns cpu usage for given pid
pub fn cpu(pid: u32) -> SigarResult<CPU> {
    let raw = ffi_wrap!(sigar_proc_cpu_get, (pid as sigar_pid_t), sigar_proc_cpu_t)?;

    Ok(value_convert!(
        CPU, raw, start_time, user, sys, total, last_time, percent,
    ))
}

// C: sigar_proc_state_get
#[derive(Debug)]
pub struct State {
    pub name: Vec<u8>,
    pub state: u8,
    pub ppid: i32,
    pub tty: i32,
    pub priority: i32,
    pub nice: i32,
    pub processor: i32,
    pub threads: u64,
}

/// Returns process state for given pid
pub fn state(pid: u32) -> SigarResult<State> {
    let raw = ffi_wrap!(
        sigar_proc_state_get,
        (pid as sigar_pid_t),
        sigar_proc_state_t
    )?;

    Ok(value_convert!(
        State, raw, ppid, tty, priority, nice, processor, threads,
        (name: chars_to_bytes(&raw.name[..])),
        (state: raw.state as u8),
    ))
}

// C: sigar_proc_fd_get
/// Process file descriptor summary
#[derive(Debug)]
pub struct FD {
    pub total: u64,
}

/// Returns fd summary for given pid
pub fn fd(pid: u32) -> SigarResult<FD> {
    let raw = ffi_wrap!(sigar_proc_fd_get, (pid as sigar_pid_t), sigar_proc_fd_t)?;

    Ok(FD { total: raw.total })
}

// TODO: some methods
// C: sigar_proc_args_get
// C: sigar_proc_env_get
// C: sigar_proc_exe_get
// C: sigar_proc_modules_get
// C: sigar_proc_port_get
