#![feature(const_fn)]
#![feature(linkage)]

extern crate libc;
use libc::{c_void, c_int, c_longlong};

const REDIS_MODULE_COMMAND : c_int = 1;
const REDIS_VERSION : *const u8 = b"2.9.999" as *const u8;

#[repr(C)]
pub struct redisCommand {
    name: *const u8,
    proc_: Option<extern "C" fn(*const c_void)>,
    arity: c_int,
    sflags: *const u8,
    flags: c_int,
    getkeys_proc: Option<extern "C" fn(*const c_void)>,
    firstkey: c_int,
    lastkey: c_int,
    keystep: c_int,
    microseconds: c_longlong,
    calls: c_longlong,
}

#[repr(C)]
pub struct redisModule {
    type_: c_int,
    redis_version: *const u8,
    module_version: *const u8,
    name: *const u8,
    load: Option<extern "C" fn()>,
    cleanup: Option<extern "C" fn() -> *const c_void>,
}


// It's not
unsafe impl Sync for redisCommand {}
unsafe impl Sync for redisModule {}

impl redisCommand {
    pub const fn null() -> redisCommand {
        redisCommand {
            name: std::ptr::null(),
            proc_: None,
            arity: 0,
            sflags: std::ptr::null(),
            flags: 0,
            getkeys_proc: None,
            firstkey: 0,
            lastkey: 0,
            keystep: 0,
            microseconds: 0,
            calls: 0
        }
    }
}

#[no_mangle]
#[allow(non_upper_case_globals)]
pub static redisCommandTable : [redisCommand; 2] = [
    redisCommand {
        name: b"rust\0" as *const u8,
        proc_: Some(rust_command),
        arity: 1,
        sflags: b"rt\0" as *const u8,
        flags: 0,
        getkeys_proc: None,
        firstkey: 0,
        lastkey: 0,
        keystep: 0,
        microseconds: 0,
        calls: 0
    },
    redisCommand::null(),
];

#[no_mangle]
#[allow(non_upper_case_globals)]
pub static redisModuleDetail : redisModule = redisModule {
    type_: REDIS_MODULE_COMMAND,
    redis_version: REDIS_VERSION,
    module_version: b"0.0001\0" as *const u8,
    name: b"de.fnordig.test.rust\0" as *const u8,
    load: Some(load),
    cleanup: None,
};

extern {
    fn addReplyString(client: *const c_void, s: *const u8, len: usize);
}

fn add_reply(client: *const c_void, reply: &str) {
    unsafe {
        let reply = format!("{}\r\n\0", reply);
        let bytes = reply.as_bytes();
        addReplyString(client, bytes.as_ptr(), bytes.len()-1);
    }
}

#[no_mangle]
pub extern "C" fn rust_command(client: *const c_void) {
    let hello = "+Hello, this is Rust!";
    add_reply(client, hello)
}

#[no_mangle]
pub extern "C" fn load() {
    println!("Rust Module loaded");
}
