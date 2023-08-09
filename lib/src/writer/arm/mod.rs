/*
 * Copyright 2023 The Native4J Authors
 *
 * Use of this source code is governed by the MIT license found in the LICENSE file.
 */
use capstone::{Insn, InsnDetail};
use jni::JNIEnv;

use crate::obj::obj::Obj;
use crate::util::{AutoObject, JObjectResult, JResult};
use crate::writer::InstructionWriter;

type ShiftType = i8;

/// ARM-specific instruction writer.
trait ArmInstructionWriter<Shift, OpMem, OpType, ArchInsnDetail>: InstructionWriter {
    /// Get the shift type and value for the given shift object.
    fn get_shift(&self, shift: Shift) -> (ShiftType, u32);

    /// Create a memory operand Java object.
    fn create_memory_operand<'jni>(
        &self,
        env: &mut JNIEnv<'jni>,
        operand: &OpMem,
    ) -> JObjectResult<'jni>;

    /// Write the value of the given operand to the given object.
    fn write_operand_value(&self, dst: &mut Obj, op: &OpType) -> JResult<()>;

    /// Create a list of operand objects for the given instruction.
    fn create_operands<'jni>(
        &self,
        env: &mut JNIEnv<'jni>,
        detail: &ArchInsnDetail,
    ) -> JResult<Vec<AutoObject<'jni>>>;

    /// Get the class name of the memory operand object.
    fn get_memory_operand_class(&self) -> &str;

    /// Get the class name of the operand object.
    fn get_operand_class(&self) -> &str;

    /// Create an operand object.
    fn create_operand_object<'jni>(&self, env: &mut JNIEnv<'jni>) -> JObjectResult<'jni> {
        let obj = env.new_object(self.get_operand_class(), "()V", &[])?;
        Ok(env.auto_local(obj))
    }

    /// Write the operands of the given instruction to the given object.
    fn write_operands(&self, dst: &mut Obj, detail: &ArchInsnDetail) -> JResult<()> {
        let operands = self.create_operands(dst.env(), detail)?;
        dst.set_object_array("operands", self.get_operand_class(), &operands)
    }

    /// Write the common fields of the given instruction to the given object.
    fn write_common(&self, dst: &mut Obj, detail: &InsnDetail, insn: &Insn) -> JResult<()> {
        dst.set_str("mnemonic", insn.mnemonic().unwrap())?;
        dst.set_str("operand", insn.op_str().unwrap())?;

        dst.set_int("instructionId", insn.id().0 as i32)?;
        dst.set_int("size", insn.len() as i32)?;

        dst.set_long("address", insn.address() as i64)?;

        let regs_read: Vec<i16> = detail.regs_read().iter().map(|r| r.0 as i16).collect();
        dst.set_short_array("regsRead", &regs_read)?;

        let regs_write: Vec<i16> = detail.regs_write().iter().map(|r| r.0 as i16).collect();
        dst.set_short_array("regsWrite", &regs_write)?;

        let groups: Vec<i16> = detail.groups().iter().map(|g| g.0 as i16).collect();
        dst.set_short_array("groups", &groups)
    }
}

pub mod arm32;
pub mod arm64;
