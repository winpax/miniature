//! numin is a library for creating and managing shim executables

#![warn(
    clippy::pedantic,
    clippy::all,
    missing_debug_implementations,
    missing_copy_implementations,
    missing_docs
)]

pub mod error;
mod executable;
pub(crate) mod interop;
pub mod subsystem;
mod table;

pub use executable::*;
