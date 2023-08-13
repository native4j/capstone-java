/*
 * Copyright 2023 The Native4J Authors
 *
 * Use of this source code is governed by the MIT license found in the LICENSE file.
 */
use jni::objects::{JByteArray, JObject};
use jni::sys::{jint, jlong, jshort, jstring};
use jni::JNIEnv;

use crate::capstone::context::CapstoneContext;

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
    let result = capstone::init(&mut env, this, mode);
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
    let result = capstone::disassemble(&mut env, this, result_object, data, count, address);
    check_result!(env, result);
    0 as jstring /* null */
}

#[no_mangle]
pub extern "system" fn Java_org_native4j_capstone_Capstone_getInsnName<'local>(
    mut env: JNIEnv<'local>,
    this: JObject<'local>,
    insn_id: jint,
) -> jstring {
    let result = capstone::get_insn_name(&mut env, this, insn_id);
    if let Err(e) = result {
        capstone::throw(&mut env, &e.to_string());
        return 0 as jstring /* null */;
    }
    result
        .unwrap()
        .map(|str| make_jstring!(env, str))
        .unwrap_or(0 as jstring /* null */)
}

#[no_mangle]
pub extern "system" fn Java_org_native4j_capstone_Capstone_getRegName<'local>(
    mut env: JNIEnv<'local>,
    this: JObject<'local>,
    reg_id: jint,
) -> jstring {
    let result = capstone::get_reg_name(&mut env, this, reg_id);
    if let Err(e) = result {
        capstone::throw(&mut env, &e.to_string());
        return 0 as jstring /* null */;
    }
    result
        .unwrap()
        .map(|str| make_jstring!(env, str))
        .unwrap_or(0 as jstring /* null */)
}

#[no_mangle]
pub extern "system" fn Java_org_native4j_capstone_Capstone_getGroupName<'local>(
    mut env: JNIEnv<'local>,
    this: JObject<'local>,
    group_id: jshort,
) -> jstring {
    let result = capstone::get_group_name(&mut env, this, group_id);
    if let Err(e) = result {
        capstone::throw(&mut env, &e.to_string());
        return 0 as jstring /* null */;
    }
    result
        .unwrap()
        .map(|str| make_jstring!(env, str))
        .unwrap_or(0 as jstring /* null */)
}
