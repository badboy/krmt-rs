macro_rules! REDIS_MODULE_DETAIL (
    ($name:expr, $module_version:expr, $load:expr, $cleanup:expr) => (
        const REDIS_MODULE_COMMAND : libc::c_int = 1;
        const REDIS_VERSION : *const u8 = b"2.9.999" as *const u8;

        #[no_mangle]
        #[allow(non_upper_case_globals)]
        pub static redisModuleDetail : redisModule = redisModule {
            type_: REDIS_MODULE_COMMAND,
            redis_version: REDIS_VERSION,
            module_version: concat_bytes!($module_version, b'\0') as *const u8,
            name: concat_bytes!($name, b'\0') as *const u8,
            load: $load,
            cleanup: $cleanup,
        };
    )
);

macro_rules! REDIS_COMMAND_TABLE (
    ($n:expr, $([$name:expr, $func:expr, $arity:expr, $sflags:expr, $getkey:expr, $firstkey:expr, $lastkey:expr, $keystep:expr]),+) => (
        #[no_mangle]
        #[allow(non_upper_case_globals)]
        pub static redisCommandTable : [redisCommand; $n+1] = [

            $(
                redisCommand {
                    name: concat_bytes!($name, b'\0') as *const u8,
                    proc_: $func,
                    arity: $arity,
                    sflags: concat_bytes!($sflags, b'\0') as *const u8,
                    flags: 0,
                    getkeys_proc: $getkey,
                    firstkey: $firstkey,
                    lastkey: $lastkey,
                    keystep: $keystep,
                    microseconds: 0,
                    calls: 0
                },
            )*
            redisCommand::null()
        ];
    )
);
