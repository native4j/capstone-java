/*
 * Copyright 2023 The Native4J Authors
 *
 * Use of this source code is governed by the MIT license found in the LICENSE file.
 */
use std::error::Error;
use std::sync::MutexGuard;

use capstone::{Capstone, Insn};
use jni::objects::JObject;
use jni::JNIEnv;

use crate::capstone::mode::CapstoneMode;
use crate::writer::arm::{arm32, arm64};

/// Creates a new instruction writer for the given mode.
pub fn create_writer<'a>(mode: &CapstoneMode) -> &'a dyn InstructionWriter {
    let writer: &dyn InstructionWriter = match mode {
        CapstoneMode::ARM32 => &arm32::Arm32Writer {},
        CapstoneMode::ARM64 => &arm64::Arm64Writer {},
    };
    writer
}

/// Responsible for writing a Capstone instruction to a Java object.
pub trait InstructionWriter {
    /// Write an instruction to the given Java object.
    fn write<'jni, 'a>(
        &self,
        env: &'a mut JNIEnv<'jni>,
        insn_object: &'a JObject<'jni>,
        insn: &Insn,
        capstone: &MutexGuard<'a, Capstone>,
    ) -> Result<(), Box<dyn Error>>;

    /// Returns the name of the Java class that this writer handles.
    fn get_instruction_class(&self) -> &'static str;
}

pub mod arm;
