macro_rules! REDIS_MODULE_DETAIL (
    ($name:expr, $module_version:expr, $load:expr, $cleanup:expr) => (
        #[no_mangle]
        #[allow(non_upper_case_globals)]
        pub static redisModuleDetail : redisModule = redisModule {
            type_: REDIS_MODULE_COMMAND,
            redis_version: REDIS_VERSION,
            module_version: $module_version as *const u8,
            name: $name as *const u8,
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
                    name: $name as *const u8,
                    proc_: $func,
                    arity: $arity,
                    sflags: $sflags as *const u8,
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
