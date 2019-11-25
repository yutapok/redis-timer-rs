mod commands;
mod schedule;

use std::str;
use libc::c_int;

use redis_mod::redis::Command;
use redis_mod::raw;

const MODULE_NAME: &str = "rmod-timer-rs";
const MODULE_VERSION: c_int = 1;


use crate::commands::{TimerSetCommand, TimerGetCommand};

#[allow(non_snake_case)]
#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn TimerSet_RedisCommand(
    ctx: *mut raw::RedisModuleCtx,
    argv: *mut *mut raw::RedisModuleString,
    argc: c_int,
) -> raw::Status {
    Command::harness(&TimerSetCommand{}, ctx, argv, argc)
}

#[allow(non_snake_case)]
#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn TimerGet_RedisCommand(
    ctx: *mut raw::RedisModuleCtx,
    argv: *mut *mut raw::RedisModuleString,
    argc: c_int,
) -> raw::Status {
    Command::harness(&TimerGetCommand{}, ctx, argv, argc)
}

#[allow(non_snake_case)]
#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn RedisModule_OnLoad(
    ctx: *mut raw::RedisModuleCtx,
    argv: *mut *mut raw::RedisModuleString,
    argc: c_int,
) -> raw::Status {
    if raw::init(
        ctx,
        format!("{}\0", MODULE_NAME).as_ptr(),
        MODULE_VERSION,
        raw::REDISMODULE_APIVER_1,
    ) == raw::Status::Err
    {
        return raw::Status::Err;
    }

    let get_command = TimerSetCommand{};
    let set_command = TimerGetCommand{};
    if raw::create_command(
        ctx,
        format!("{}\0", get_command.name()).as_ptr(),
        Some(TimerSet_RedisCommand),
        format!("{}\0", get_command.str_flags()).as_ptr(),
        0,
        0,
        0,
    ) == raw::Status::Err
    {
        return raw::Status::Err;
    }

    if raw::create_command(
        ctx,
        format!("{}\0", set_command.name()).as_ptr(),
        Some(TimerGet_RedisCommand),
        format!("{}\0", set_command.str_flags()).as_ptr(),
        0,
        0,
        0,
    ) == raw::Status::Err
    {
        return raw::Status::Err;
    }

    raw::Status::Ok
}

