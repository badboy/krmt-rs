use libc::{c_void, c_int, c_longlong};
use std::ptr;

#[repr(C)]
#[allow(dead_code)]
pub struct redisCommand {
    pub name: *const u8,
    pub proc_: Option<extern "C" fn(*const c_void)>,
    pub arity: c_int,
    pub sflags: *const u8,
    pub flags: c_int,
    pub getkeys_proc: Option<extern "C" fn(*const c_void)>,
    pub firstkey: c_int,
    pub lastkey: c_int,
    pub keystep: c_int,
    pub microseconds: c_longlong,
    pub calls: c_longlong,
}

#[repr(C)]
#[allow(dead_code)]
pub struct redisModule {
    pub type_: c_int,
    pub redis_version: *const u8,
    pub module_version: *const u8,
    pub name: *const u8,
    pub load: Option<extern "C" fn() -> *const c_void>,
    pub cleanup: Option<extern "C" fn()>,
}

// It's not
unsafe impl Sync for redisCommand {}
unsafe impl Sync for redisModule {}

impl redisCommand {
    #[allow(dead_code)]
    pub const fn null() -> redisCommand {
        redisCommand {
            name: ptr::null(),
            proc_: None,
            arity: 0,
            sflags: ptr::null(),
            flags: 0,
            getkeys_proc: None,
            firstkey: 0,
            lastkey: 0,
            keystep: 0,
            microseconds: 0,
            calls: 0,
        }
    }
}
