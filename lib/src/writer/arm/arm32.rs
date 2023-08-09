/*
 * Copyright 2023 The Native4J Authors
 *
 * Use of this source code is governed by the MIT license found in the LICENSE file.
 */
use std::error::Error;
use std::sync::MutexGuard;

use capstone::arch::arm::{ArmInsnDetail, ArmOpMem, ArmOperandType, ArmShift};
use capstone::arch::DetailsArchInsn;
use capstone::{Capstone, Insn};
use jni::objects::{JObject, JValue};
use jni::sys::jint;
use jni::JNIEnv;

use crate::obj::boxing::{boxed_byte, boxed_double, boxed_int};
use crate::obj::obj::Obj;
use crate::util::{AutoObject, JObjectResult, JResult};
use crate::writer::arm::{ArmInstructionWriter, ShiftType};
use crate::writer::InstructionWriter;

/// Writer for ARM instructions.
pub struct Arm32Writer;

impl ArmInstructionWriter<ArmShift, ArmOpMem, ArmOperandType, ArmInsnDetail<'_>> for Arm32Writer {
    fn get_shift(&self, shift: ArmShift) -> (ShiftType, u32) {
        match shift {
            ArmShift::Invalid => (0, 0),
            ArmShift::Asr(v) => (1, v),
            ArmShift::Lsl(v) => (2, v),
            ArmShift::Lsr(v) => (3, v),
            ArmShift::Ror(v) => (4, v),
            ArmShift::Rrx(v) => (5, v),
            ArmShift::AsrReg(v) => (6, v.0 as u32),
            ArmShift::LslReg(v) => (7, v.0 as u32),
            ArmShift::LsrReg(v) => (8, v.0 as u32),
            ArmShift::RorReg(v) => (9, v.0 as u32),
            ArmShift::RrxReg(v) => (10, v.0 as u32),
        }
    }

    fn create_memory_operand<'jni>(
        &self,
        env: &mut JNIEnv<'jni>,
        operand: &ArmOpMem,
    ) -> JObjectResult<'jni> {
        let obj = env.new_object(
            self.get_memory_operand_class(),
            "(IIII)V",
            &[
                JValue::Int(operand.base().0 as jint),
                JValue::Int(operand.index().0 as jint),
                JValue::Int(operand.scale()),
                JValue::Int(operand.disp()),
            ],
        )?;
        Ok(env.auto_local(obj))
    }

    fn write_operand_value(&self, dst: &mut Obj, op: &ArmOperandType) -> JResult<()> {
        let env = dst.env();

        let value = match op {
            ArmOperandType::Invalid => None,
            ArmOperandType::Reg(v) => Some((1, boxed_int(env, v.0 as i32)?)),
            ArmOperandType::Imm(v) => Some((2, boxed_int(env, *v)?)),
            ArmOperandType::Mem(v) => Some((3, self.create_memory_operand(env, v)?)),
            ArmOperandType::Fp(v) => Some((4, boxed_double(env, *v)?)),
            ArmOperandType::Cimm(v) => Some((5, boxed_int(env, *v)?)),
            ArmOperandType::Pimm(v) => Some((6, boxed_int(env, *v)?)),
            ArmOperandType::Setend(v) => Some((7, boxed_byte(env, *v as i8)?)),
            ArmOperandType::SysReg(v) => Some((8, boxed_int(env, v.0 as i32)?)),
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
        detail: &ArmInsnDetail<'_>,
    ) -> JResult<Vec<AutoObject<'jni>>> {
        let mut objects = Vec::new();

        for ref op in detail.operands() {
            let object = self.create_operand_object(env)?;

            let mut op_dst = Obj::from(env, &object);

            let vector_index = op.vector_index.unwrap_or(u32::MAX);
            op_dst.set_int("vectorIndex", vector_index as i32)?;

            op_dst.set_bool("isSubtracted", op.subtracted)?;

            let (shift_type, value) = self.get_shift(op.shift);
            op_dst.set_byte("shiftType", shift_type)?;
            op_dst.set_long("shiftValue", value as i64)?;

            self.write_operand_value(&mut op_dst, &op.op_type)?;

            objects.push(object);
        }

        Ok(objects)
    }

    fn get_memory_operand_class(&self) -> &str {
        "Lorg/native4j/capstone/insn/arm/CsMemOperandArm32;"
    }

    fn get_operand_class(&self) -> &str {
        "Lorg/native4j/capstone/insn/arm/CsOperandArm32;"
    }
}

impl InstructionWriter for Arm32Writer {
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
        let arch = arch_detail.arm().ok_or("no detail")?;

        dst.set_bool("isUsermode", arch.usermode())?;
        dst.set_int("vectorSize", arch.vector_size())?;
        dst.set_byte("vectorData", arch.vector_data() as i8)?;
        dst.set_byte("cpsMode", arch.cps_mode() as i8)?;
        dst.set_byte("cpsFlag", arch.cps_flag() as i8)?;
        dst.set_byte("conditionCodes", arch.cc() as i8)?;
        dst.set_bool("updatesFlags", arch.update_flags())?;
        dst.set_bool("writebackRequired", arch.writeback())?;
        dst.set_byte("memBarrier", arch.mem_barrier() as i8)?;

        self.write_operands(&mut dst, arch)?;

        Ok(())
    }

    fn get_instruction_class(&self) -> &'static str {
        "Lorg/native4j/capstone/insn/arm/CsInsnArm32;"
    }
}
