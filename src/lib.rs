//! libsamplerate is a C2Rust-transpiled version of the original libsamplerate. Therefore, it's a
//! pure (albeit not necessarily memory-safe) Rust version of the library. No dealing with build
//! hassles, no dealing with C.
//!
//! The interface is nearly identical to `libsamplerate-rust`, and should be a drop-in replacement.

#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
//#![feature(extern_types)]

extern crate libc;

pub mod samplerate;

pub mod src_linear;

pub mod src_sinc;

pub mod src_zoh;

pub use samplerate::SRC_DATA;
pub use samplerate::*;
pub use src_sinc::*;
