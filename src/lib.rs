#![feature(const_fn)]
#![feature(linkage)]

extern crate libc;
use libc::{c_void, c_int};

#[macro_use]
mod raw;
mod structs;
mod redis;

use structs::{redisCommand,redisModule};

const REDIS_MODULE_COMMAND : c_int = 1;
const REDIS_VERSION : *const u8 = b"2.9.999" as *const u8;

REDIS_MODULE_DETAIL!(
    b"de.fnordig.test.rust\0",
    b"0.0001\0",
    Some(load),
    None
);

REDIS_COMMAND_TABLE!(
    2,
    [b"rust\0", Some(rust_command), 1, b"rt", None, 0,0,0],
    [b"dumdidum\0", Some(rust_command), 1, b"rt", None, 0,0,0]
);

#[no_mangle]
pub extern "C" fn rust_command(client: *const c_void) {
    let hello = "+Hello, this is Rust!";
    redis::add_reply(client, hello)
}

#[no_mangle]
pub extern "C" fn load() {
    println!("Rust Module loaded");
}
