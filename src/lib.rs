#![feature(convert)]

extern crate unqlite_sys as ffi;
extern crate libc;

pub use error::*;

#[macro_use]
#[allow(dead_code, non_camel_case_types)]mod error;

pub use engine::UnQlite;
pub mod engine;
