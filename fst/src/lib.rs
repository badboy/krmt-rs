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
use std::str::from_utf8;
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
    static ref DATABASE : Mutex<HashMap<Vec<u8>, FstValue>> = Mutex::new(HashMap::new());
}

REDIS_MODULE_DETAIL!(
    "de.fnordig.test.fst",
    "0.0001",
    Some(load),
    Some(cleanup)
);

REDIS_COMMAND_TABLE!(
    6,
    ["fstadd", Some(fstadd), 3, "rt", None, 0, 0, 0],
    ["fstfinish", Some(fstfinish), 2, "rt", None, 0, 0, 0],
    ["fstlen", Some(fstlen), 2, "rt", None, 0, 0, 0],
    ["fstdel", Some(fstdel), -2, "rt", None, 0, 0, 0],
    ["fstkeys", Some(fstkeys), 1, "rt", None, 0, 0, 0],
    ["fstismember", Some(fstismember), 3, "rt", None, 0, 0, 0]
);

#[no_mangle]
pub extern "C" fn fstadd(client: Client) {
    let mut args = redis::args(client).into_iter();
    args.next().unwrap(); // Drop command name

    let key = args.next().unwrap();
    let mut database = DATABASE.lock().unwrap();
    let mut builder = database.entry(key).or_insert(
        Builder(fst::SetBuilder::memory()));

    let value = args.next().unwrap();
    let value = match from_utf8(&value) {
        Err(_) => {
            redis::error_reply(client, "Value is not valid UTF-8");
            return;
        },
        Ok(v) => v
    };

    match builder {
        &mut Set(_) => {
            redis::error_reply(client, "Can't modify finished set");
            return;
        },
        &mut Builder(ref mut b) => {
            match b.insert(value) {
                Err(e) => redis::error_reply(client, &format!("{:?}", e)),
                Ok(_)  => redis::ok_reply(client)
            }
        }
    };
}

#[no_mangle]
pub extern "C" fn fstfinish(client: Client) {
    let mut args = redis::args(client).into_iter();
    args.next().unwrap(); // Drop command name

    let key = args.next().unwrap();
    let mut database = DATABASE.lock().unwrap();
    let val = database.remove(&key);

    let val = match val {
        Some(val) => val,
        None => {
            redis::error_reply(client, "Can't finish empty set");
            return;
        },
    };

    let builder = match val {
        Set(_) => {
            redis::error_reply(client, "Can't modify finished set");
            return;
        },
        Builder(b) => b,
    };

    let bytes = builder.into_inner().unwrap();
    let set = fst::Set::from_bytes(bytes).unwrap();
    database.insert(key, Set(set));

    redis::ok_reply(client);
}

#[no_mangle]
pub extern "C" fn fstdel(client: Client) {
    let mut args = redis::args(client).into_iter();
    args.next().unwrap(); // Drop command name

    let mut deleted = 0;
    for key in args {
        let mut database = DATABASE.lock().unwrap();
        let val = database.remove(&key);

        if let Some(_) = val {
            deleted += 1;
        }
    }

    redis::integer_reply(client, deleted);
}

#[no_mangle]
pub extern "C" fn fstkeys(client: Client) {
    let database = DATABASE.lock().unwrap();

    let keys = database.keys();
    let len = keys.len();

    redis::add_reply(client, &format!("*{}", len));

    for key in keys {
        let len = key.len();
        redis::add_reply(client, &format!("${}", len));
        redis::add_reply_bytes(client, key);
    }
}

#[no_mangle]
pub extern "C" fn fstlen(client: Client) {
    let mut args = redis::args(client).into_iter();
    args.next().unwrap(); // Drop command name

    let key = args.next().unwrap();
    let database = DATABASE.lock().unwrap();
    let val = database.get(&key);

    let val = match val {
        Some(val) => val,
        None => {
            redis::integer_reply(client, 0);
            return;
        },
    };

    match val {
        &Builder(_) => {
            redis::error_reply(client, "Can't get len of unfinished set");
            return;
        },
        &Set(ref s) => {
            redis::integer_reply(client, s.len() as i64);
            return;
        },
    };
}

#[no_mangle]
pub extern "C" fn fstismember(client: Client) {
    let mut args = redis::args(client).into_iter();
    args.next().unwrap(); // Drop command name

    let key = args.next().unwrap();
    let database = DATABASE.lock().unwrap();
    let val = database.get(&key);

    let val = match val {
        Some(val) => val,
        None => {
            redis::integer_reply(client, 0);
            return;
        },
    };

    let member = args.next().unwrap();
    let member = match from_utf8(&member) {
        Err(_) => {
            redis::error_reply(client, "Member is not valid UTF-8");
            return;
        },
        Ok(v) => v
    };

    match val {
        &Builder(_) => {
            redis::error_reply(client, "Can't check unfinished set");
            return;
        },
        &Set(ref s) => {
            if s.contains(member) {
                redis::integer_reply(client, 1);
            } else {
                redis::integer_reply(client, 0);
            }
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
