#![warn(clippy::all, clippy::pedantic)]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod exe_type;
pub mod interop;

extern crate alloc;
