//! Provides a high level wrapper of sigar

#![warn(missing_docs)]

extern crate sigar_sys;

mod result;

pub use result::{Error, SigarResult};

#[macro_use]
mod macros;

pub mod cpu;
