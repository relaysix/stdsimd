//! SIMD and vendor intrinsics support library.
//!
//! This crate defines the vendor intrinsics and types primarily used for SIMD
//! in Rust. The crate here will soon be available in the standard library, but
//! for now you can also browse the documentation here, primarily in the `arch`
//! submodule.
//!
//! [stdsimd]: https://rust-lang-nursery.github.io/stdsimd/x86_64/stdsimd/

#![feature(const_fn, integer_atomics, staged_api, stdsimd)]
#![cfg_attr(target_os = "linux", feature(linkage))]
#![no_std]
#![unstable(feature = "stdsimd", issue = "0")]

extern crate std as _std;
extern crate coresimd;
extern crate libc;
#[macro_use]
extern crate cfg_if;

#[cfg(test)]
#[macro_use]
extern crate std;

#[path = "../../../stdsimd/mod.rs"]
mod stdsimd;

pub use stdsimd::*;

use _std::prelude;
use _std::fs;
use _std::io;
