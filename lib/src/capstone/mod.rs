/*
 * Copyright 2023 The Native4J Authors
 *
 * Use of this source code is governed by the MIT license found in the LICENSE file.
 */
use crate::capstone::context::CapstoneContext;
use crate::capstone::mode::CapstoneMode;
use crate::capstone::output::CapstoneOutput;
use capstone::arch::BuildsCapstone;
use capstone::{arch, Capstone, InsnGroupId, InsnGroupIdInt, InsnId, InsnIdInt, RegId, RegIdInt};
use jni::objects::{JByteArray, JObject, ReleaseMode};
use jni::sys::{jint, jlong, jshort};
use jni::JNIEnv;
use std::ops::BitAnd;

pub mod context;
pub mod mode;
pub mod output;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn init<'local>(
    env: &mut JNIEnv<'local>,
    this: JObject<'local>,
    mode: JObject<'local>,
) -> Result<()> {
    let mode = CapstoneMode::from(env, &mode);

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
        _ => return Err("invalid argument 'mode'".into()),
    }
    .map_err(|e| e.to_string())?;

    let instance = CapstoneContext::new(capstone, mode.unwrap());

    CapstoneContext::surrender_instance(instance, env, &this)?;
    Ok(())
}

pub fn disassemble<'local>(
    env: &mut JNIEnv<'local>,
    this: JObject<'local>,
    result_object: JObject<'local>,
    data: JByteArray<'local>,
    count: jint,
    address: jlong,
) -> Result<()> {
    let code: Vec<u8> = {
        let result = unsafe { env.get_array_elements(&data, ReleaseMode::NoCopyBack)? };
        result.iter().map(|b| (*b as u8).bitand(0xff)).collect()
    };

    let ctx = CapstoneContext::get(env, &this)?;
    let capstone = ctx.capstone.lock().unwrap();

    let instructions = {
        if count == 0 {
            capstone.disasm_all(&code, address as u64)
        } else {
            capstone.disasm_count(&code, address as u64, count as usize)
        }
        .map_err(|e| e.to_string())?
    };

    let mut output = CapstoneOutput::new(env, &ctx.mode, &capstone, &result_object);
    output.copy_instructions(instructions)
}

pub fn get_insn_name<'local>(
    env: &mut JNIEnv<'local>,
    this: JObject<'local>,
    insn_id: jint,
) -> Result<Option<String>> {
    let ctx = CapstoneContext::get(env, &this)?;
    let capstone = ctx.capstone.lock().unwrap();
    Ok(capstone.insn_name(InsnId(insn_id as InsnIdInt)))
}

pub fn get_reg_name<'local>(
    env: &mut JNIEnv<'local>,
    this: JObject<'local>,
    reg_id: jint,
) -> Result<Option<String>> {
    let ctx = CapstoneContext::get(env, &this)?;
    let capstone = ctx.capstone.lock().unwrap();
    Ok(capstone.reg_name(RegId(reg_id as RegIdInt)))
}

pub fn get_group_name<'local>(
    env: &mut JNIEnv<'local>,
    this: JObject<'local>,
    group_id: jshort,
) -> Result<Option<String>> {
    let ctx = CapstoneContext::get(env, &this)?;
    let capstone = ctx.capstone.lock().unwrap();
    Ok(capstone.group_name(InsnGroupId(group_id as InsnGroupIdInt)))
}

pub fn throw(env: &mut JNIEnv, message: &str) {
    env.throw_new("org/native4j/capstone/exception/CapstoneException", message)
        .unwrap();
}
