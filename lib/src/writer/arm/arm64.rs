/*
 * Copyright 2023 The Native4J Authors
 *
 * Use of this source code is governed by the MIT license found in the LICENSE file.
 */
use std::error::Error;
use std::sync::MutexGuard;

use capstone::arch::arm64::{Arm64InsnDetail, Arm64OpMem, Arm64OperandType, Arm64Shift};
use capstone::arch::DetailsArchInsn;
use capstone::{Capstone, Insn};
use jni::objects::{JObject, JValue};
use jni::sys::jint;
use jni::JNIEnv;

use crate::obj::boxing::{boxed_byte, boxed_double, boxed_int, boxed_long};
use crate::obj::obj::Obj;
use crate::util::{AutoObject, JObjectResult, JResult};
use crate::writer::arm::{ArmInstructionWriter, ShiftType};
use crate::writer::InstructionWriter;

/// Writer for ARM64 instructions.
pub struct Arm64Writer;

impl ArmInstructionWriter<Arm64Shift, Arm64OpMem, Arm64OperandType, Arm64InsnDetail<'_>>
    for Arm64Writer
{
    fn get_shift(&self, shift: Arm64Shift) -> (ShiftType, u32) {
        match shift {
            Arm64Shift::Invalid => (0, 0),
            Arm64Shift::Lsl(v) => (1, v),
            Arm64Shift::Msl(v) => (2, v),
            Arm64Shift::Lsr(v) => (3, v),
            Arm64Shift::Asr(v) => (4, v),
            Arm64Shift::Ror(v) => (5, v),
        }
    }

    fn create_memory_operand<'jni>(
        &self,
        env: &mut JNIEnv<'jni>,
        operand: &Arm64OpMem,
    ) -> JObjectResult<'jni> {
        let obj = env.new_object(
            self.get_memory_operand_class(),
            "(III)V",
            &[
                JValue::Int(operand.base().0 as jint),
                JValue::Int(operand.index().0 as jint),
                JValue::Int(operand.disp()),
            ],
        )?;
        Ok(env.auto_local(obj))
    }

    fn write_operand_value(&self, dst: &mut Obj, op: &Arm64OperandType) -> JResult<()> {
        let env = dst.env();

        let value = match op {
            Arm64OperandType::Invalid => None,
            Arm64OperandType::Reg(v) => Some((1, boxed_int(env, v.0 as i32)?)),
            Arm64OperandType::Imm(v) => Some((2, boxed_long(env, *v)?)),
            Arm64OperandType::Mem(v) => Some((3, self.create_memory_operand(env, v)?)),
            Arm64OperandType::Fp(v) => Some((4, boxed_double(env, *v)?)),
            Arm64OperandType::Cimm(v) => Some((5, boxed_long(env, *v)?)),
            Arm64OperandType::RegMrs(v) => Some((6, boxed_int(env, *v as i32)?)),
            Arm64OperandType::RegMsr(v) => Some((7, boxed_int(env, *v as i32)?)),
            Arm64OperandType::Pstate(v) => Some((8, boxed_byte(env, *v as i8)?)),
            Arm64OperandType::Sys(v) => Some((9, boxed_long(env, *v as i64)?)),
            Arm64OperandType::Prefetch(v) => Some((10, boxed_byte(env, *v as i8)?)),
            Arm64OperandType::Barrier(v) => Some((11, boxed_byte(env, *v as i8)?)),
        };

        if let Some((op_type, value)) = value {
            dst.set_byte("operandType", op_type)?;
            dst.set_object("rawOperandValue", "Ljava/lang/Object;", &value)?;
        } else {
            dst.set_byte("operandType", 0)?;
        }

        Ok(())
    }

    fn create_operands<'jni>(
        &self,
        env: &mut JNIEnv<'jni>,
        detail: &Arm64InsnDetail<'_>,
    ) -> JResult<Vec<AutoObject<'jni>>> {
        let mut objects = Vec::new();

        for ref op in detail.operands() {
            let object = self.create_operand_object(env)?;

            let mut op_dst = Obj::from(env, &object);

            let vector_index = op.vector_index.unwrap_or(u32::MAX);
            op_dst.set_int("vectorIndex", vector_index as i32)?;

            op_dst.set_byte("vas", op.vas as i8)?;

            let (shift_type, value) = self.get_shift(op.shift);
            op_dst.set_byte("shiftType", shift_type)?;
            op_dst.set_long("shiftValue", value as i64)?;

            op_dst.set_byte("ext", op.ext as i8)?;

            self.write_operand_value(&mut op_dst, &op.op_type)?;

            objects.push(object);
        }

        Ok(objects)
    }

    fn get_memory_operand_class(&self) -> &str {
        "Lorg/native4j/capstone/insn/arm/CsMemOperandArm64;"
    }

    fn get_operand_class(&self) -> &str {
        "Lorg/native4j/capstone/insn/arm/CsOperandArm64;"
    }
}

impl InstructionWriter for Arm64Writer {
    fn write<'jni, 'a>(
        &self,
        env: &'a mut JNIEnv<'jni>,
        insn_object: &'a JObject<'jni>,
        insn: &Insn,
        capstone: &MutexGuard<'a, Capstone>,
    ) -> Result<(), Box<dyn Error>> {
        let detail = capstone
            .insn_detail(insn)
            .expect("detail mode should be on");

        let mut dst = Obj::from(env, insn_object);
        self.write_common(&mut dst, &detail, insn)?;

        let arch_detail = detail.arch_detail();
        let arch = arch_detail.arm64().ok_or("no detail")?;

        dst.set_byte("conditionCodes", arch.cc() as i8)?;
        dst.set_bool("updatesFlags", arch.update_flags())?;
        dst.set_bool("writebackRequired", arch.writeback())?;

        self.write_operands(&mut dst, arch)?;

        Ok(())
    }

    fn get_instruction_class(&self) -> &'static str {
        "Lorg/native4j/capstone/insn/arm/CsInsnArm64;"
    }
}
