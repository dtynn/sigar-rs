//! Provides a high level wrapper of sigar

extern crate sigar_sys;

mod result;
mod util;

pub use result::{Error, SigarResult};

#[macro_use]
mod macros;

pub mod cpu;
pub mod load;
pub mod mem;
pub mod process;
