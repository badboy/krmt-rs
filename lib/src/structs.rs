use libc::{c_void, c_int, c_long, c_longlong, c_ulong, time_t, c_char};
use std::ptr;
use super::redis::Client;

#[repr(C)]
#[allow(dead_code)]
pub struct redisCommand {
    pub name: *const u8,
    pub proc_: Option<extern "C" fn(Client)>,
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

#[repr(C)]
#[allow(dead_code)]
pub struct redisClient {
    id: u64,
    fd: c_int,
    db: *const c_void,
    dictid: c_int,
    name: *const c_void,
    querybuf: *const c_void,
    querybuf_peak: usize,
    argc: c_int,
    argv: *const *const c_void,
    cmd: *const c_void,
    lastcmd: *const c_void,
    reqtype: c_int,
    multibulklen: c_int,
    bulklen: c_long,
    reply: *const c_void,
    reply_bytes: c_ulong,
    sentlen: c_int,
    ctime: time_t,
    lastinteraction: time_t,
    obuf_soft_limit_reached_time: time_t,
    flags: c_int,
    authenticated: c_int,
    replstate: c_int,
    repl_put_online_on_ack: c_int,
    repldbfd: c_int,
    repldboff: u64,
    repldbsize: u64,
    replpreamble: *const c_void,
    reploff: c_longlong,
    repl_ack_off: c_longlong,
    repl_ack_time: c_longlong,
    replrunid: [c_char; 41],
    slave_listening_port: c_int,
    mstate: [u8; 24],
    btype: c_int,
    bpop: [u8; 40],
    woff: c_longlong,
    watched_keys: *const c_void,
    pubsub_channels: *const c_void,
    pubsub_patterns: *const c_void,
    peerid: *const c_void,
    bufpos: c_int,
    buf: [u8; (16*1024)]
}


// It's not
unsafe impl Sync for redisCommand {}
unsafe impl Sync for redisModule {}
unsafe impl Sync for redisClient {}

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
