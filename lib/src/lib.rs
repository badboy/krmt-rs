#![feature(const_fn)]
#![feature(linkage)]
#![feature(plugin)]
#![plugin(concat_bytes)]

extern crate libc;

#[macro_use]
pub mod raw;
pub mod structs;
pub mod redis;
