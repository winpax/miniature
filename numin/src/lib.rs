#![warn(
    missing_debug_implementations,
    missing_copy_implementations,
    // missing_docs
)]

pub mod error;
mod executable;
pub(crate) mod interop;
mod table;

pub use executable::*;
