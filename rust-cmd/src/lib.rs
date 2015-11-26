#![feature(const_fn)]
#![feature(linkage)]
#![feature(plugin)]
#![plugin(concat_bytes)]

extern crate libc;
#[macro_use]
extern crate redis_dynamic;

use std::ptr::null;

use redis_dynamic::structs::*;
use redis_dynamic::redis;
use redis_dynamic::redis::Client;

REDIS_MODULE_DETAIL!(
    "de.fnordig.test.rust",
    "0.0001",
    Some(load),
    Some(cleanup)
);

REDIS_COMMAND_TABLE!(
    2,
    ["rust", Some(rust_command), 2, "rt", None, 0, 0, 0],
    ["dumdidum", Some(rust_command), 1, "rt", None, 0, 0, 0]
);

#[no_mangle]
pub extern "C" fn rust_command(client: Client) {
    let args = redis::args(client);
    println!("ARGS: {:?}", args);

    let hello = "+Hello, this is Rust!";
    redis::add_reply(client, hello)
}

#[no_mangle]
pub extern "C" fn load() -> *const libc::c_void {
    println!("Rust Module loaded");

    null()
}

#[no_mangle]
pub extern "C" fn cleanup() {
    println!("Rust Module cleaned up");
}
