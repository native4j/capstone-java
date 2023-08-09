/*
 * Copyright 2023 The Native4J Authors
 *
 * Use of this source code is governed by the MIT license found in the LICENSE file.
 */
use std::ops::BitAnd;

use ::capstone::arch::BuildsCapstone;
use ::capstone::{arch, Capstone, InsnGroupId, InsnGroupIdInt, InsnId, InsnIdInt, RegId, RegIdInt};
use jni::objects::{JByteArray, JObject, ReleaseMode};
use jni::sys::{jint, jlong, jshort, jstring};
use jni::JNIEnv;

use crate::capstone::context::CapstoneContext;
use crate::capstone::mode::CapstoneMode;
use crate::capstone::output::CapstoneOutput;

mod capstone;
mod obj;
mod util;
mod writer;
#[no_mangle]
pub extern "system" fn Java_org_native4j_capstone_Capstone_init<'local>(
    mut env: JNIEnv<'local>,
    this: JObject<'local>,
    mode: JObject<'local>,
) -> jstring {
    let mode = CapstoneMode::from(&mut env, &mode);

    let capstone = match mode {
        Some(CapstoneMode::ARM32) => Capstone::new()
            .arm()
            .mode(arch::arm::ArchMode::Arm)
            .detail(true)
            .build(),
        Some(CapstoneMode::ARM64) => Capstone::new()
            .arm64()
            .mode(arch::arm64::ArchMode::Arm)
            .detail(true)
            .build(),
        _ => return make_error!(env, "invalid argument 'mode'"),
    };
    check_result!(env, capstone);

    let instance = CapstoneContext::new(capstone.unwrap(), mode.unwrap());

    let result = CapstoneContext::surrender_instance(instance, &mut env, &this);
    check_result!(env, result);

    0 as jstring /* null */
}

#[no_mangle]
pub extern "system" fn Java_org_native4j_capstone_Capstone_shutdown<'local>(
    mut env: JNIEnv<'local>,
    this: JObject<'local>,
) -> jstring {
    let result = CapstoneContext::drop_instance(&mut env, &this);
    check_result!(env, result);

    0 as jstring /* null */
}

#[no_mangle]
pub extern "system" fn Java_org_native4j_capstone_Capstone_disassemble<'local>(
    mut env: JNIEnv<'local>,
    this: JObject<'local>,
    result_object: JObject<'local>,
    data: JByteArray<'local>,
    count: jint,
    address: jlong,
) -> jstring {
    let code: Vec<u8> = {
        let result = unsafe { env.get_array_elements(&data, ReleaseMode::NoCopyBack) };
        check_result!(env, result);
        result
            .unwrap()
            .iter()
            .map(|b| (*b as u8).bitand(0xff))
            .collect()
    };

    let ctx = CapstoneContext::get(&mut env, &this);
    let capstone = ctx.capstone.lock().unwrap();

    let instructions = {
        let result = if count == 0 {
            capstone.disasm_all(&code, address as u64)
        } else {
            capstone.disasm_count(&code, address as u64, count as usize)
        };
        if let Err(e) = result {
            return make_error!(env, e.to_string());
        }
        result.unwrap()
    };

    let mut output = CapstoneOutput::new(&mut env, &ctx.mode, &capstone, &result_object);

    let result = output.copy_instructions(instructions);
    check_result!(env, result);

    0 as jstring /* null */
}

#[no_mangle]
pub extern "system" fn Java_org_native4j_capstone_Capstone_getInsnName<'local>(
    mut env: JNIEnv<'local>,
    this: JObject<'local>,
    insn_id: jint,
) -> jstring {
    let ctx = CapstoneContext::get(&mut env, &this);
    let capstone = ctx.capstone.lock().unwrap();
    match capstone.insn_name(InsnId(insn_id as InsnIdInt)) {
        Some(str) => make_jstring!(env, str),
        None => 0 as jstring, /* null */
    }
}

#[no_mangle]
pub extern "system" fn Java_org_native4j_capstone_Capstone_getRegName<'local>(
    mut env: JNIEnv<'local>,
    this: JObject<'local>,
    reg_id: jint,
) -> jstring {
    let ctx = CapstoneContext::get(&mut env, &this);
    let capstone = ctx.capstone.lock().unwrap();
    match capstone.reg_name(RegId(reg_id as RegIdInt)) {
        Some(str) => make_jstring!(env, str),
        None => 0 as jstring, /* null */
    }
}

#[no_mangle]
pub extern "system" fn Java_org_native4j_capstone_Capstone_getGroupName<'local>(
    mut env: JNIEnv<'local>,
    this: JObject<'local>,
    group_id: jshort,
) -> jstring {
    let ctx = CapstoneContext::get(&mut env, &this);
    let capstone = ctx.capstone.lock().unwrap();
    match capstone.group_name(InsnGroupId(group_id as InsnGroupIdInt)) {
        Some(str) => make_jstring!(env, str),
        None => 0 as jstring, /* null */
    }
}
