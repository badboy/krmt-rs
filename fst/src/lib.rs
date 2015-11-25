#![feature(const_fn)]
#![feature(linkage)]
#![feature(plugin)]
#![plugin(concat_bytes)]

extern crate libc;
#[macro_use]
extern crate redis_dynamic;
#[macro_use]
extern crate lazy_static;
extern crate fst;

use std::ptr::null;
use std::collections::HashMap;

use redis_dynamic::structs::*;
use redis_dynamic::redis;
use redis_dynamic::redis::Client;

use std::sync::Mutex;

use FstValue::*;

enum FstValue {
    Builder(fst::SetBuilder<Vec<u8>>),
    Set(fst::Set),
}

lazy_static!{
    static ref DATABASE : Mutex<HashMap<String, FstValue>> = Mutex::new(HashMap::new());
}

REDIS_MODULE_DETAIL!(
    "de.fnordig.test.fst",
    "0.0001",
    Some(load),
    Some(cleanup)
);

REDIS_COMMAND_TABLE!(
    3,
    ["fstadd", Some(fstadd), 3, "rt", None, 0, 0, 0],
    ["fstfinish", Some(fstfinish), 2, "rt", None, 0, 0, 0],
    ["fstlen", Some(fstlen), 2, "rt", None, 0, 0, 0]
);

#[no_mangle]
pub extern "C" fn fstadd(client: Client) {
    let key = "hello".to_owned();
    let mut database = DATABASE.lock().unwrap();
    let mut val = database.entry(key).or_insert(
        Builder(fst::SetBuilder::memory()));

    match val {
        &mut Set(_) => {
            let hello = "-Can't modify finished set";
            redis::add_reply(client, hello);
            return;
        },
        &mut Builder(ref mut b) => {
            b.insert("hello").unwrap();
            let hello = "+OK";
            redis::add_reply(client, hello);
        },
    };
}

#[no_mangle]
pub extern "C" fn fstfinish(client: Client) {
    let key = "hello".to_owned();
    let mut database = DATABASE.lock().unwrap();
    let val = database.remove(&key);

    let val = match val {
        Some(val) => val,
        None => {
            let hello = "-Can't finish empty set";
            redis::add_reply(client, hello);
            return;
        },
    };

    let builder = match val {
        Set(_) => {
            let hello = "-Can't modify finished set";
            redis::add_reply(client, hello);
            return;
        },
        Builder(b) => b,
    };

    let bytes = builder.into_inner().unwrap();
    let set = fst::Set::from_bytes(bytes).unwrap();
    database.insert(key, Set(set));

    let hello = "+OK";
    redis::add_reply(client, hello);
}

#[no_mangle]
pub extern "C" fn fstlen(client: Client) {
    let key = "hello".to_owned();
    let database = DATABASE.lock().unwrap();
    let val = database.get(&key);

    let val = match val {
        Some(val) => val,
        None => {
            let hello = ":0";
            redis::add_reply(client, hello);
            return;
        },
    };

    match val {
        &Builder(_) => {
            let hello = "-Can't get len of unfinished set";
            redis::add_reply(client, hello);
            return;
        },
        &Set(ref s) => {
            let hello = format!(":{}", s.len());
            redis::add_reply(client, &hello);
            return;
        },
    };
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
