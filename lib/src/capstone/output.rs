/*
 * Copyright 2023 The Native4J Authors
 *
 * Use of this source code is governed by the MIT license found in the LICENSE file.
 */
use capstone::{Capstone, Instructions};
use jni::objects::{JObject, JObjectArray, JValue};
use jni::signature::ReturnType;
use jni::sys::jsize;
use jni::JNIEnv;
use std::sync::MutexGuard;

use crate::capstone::mode::CapstoneMode;
use crate::obj::obj::Obj;
use crate::util::JResult;
use crate::writer;

const BASE_INSTRUCTION_CLASS: &str = "Lorg/native4j/capstone/insn/CsInsn;";

pub struct CapstoneOutput<'jni, 'a> {
    env: &'a mut JNIEnv<'jni>,
    mode: &'a CapstoneMode,
    capstone: &'a MutexGuard<'a, Capstone>,
    result_object: &'a JObject<'jni>,
}

impl<'jni, 'a> CapstoneOutput<'jni, 'a> {
    pub fn new(
        env: &'a mut JNIEnv<'jni>,
        mode: &'a CapstoneMode,
        capstone: &'a MutexGuard<'a, Capstone>,
        result_object: &'a JObject<'jni>,
    ) -> CapstoneOutput<'jni, 'a> {
        CapstoneOutput {
            env,
            mode,
            capstone,
            result_object,
        }
    }

    pub fn copy_instructions(
        &mut self,
        instructions: Instructions,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let count = instructions.len();

        let mut result = Obj::from(self.env, self.result_object);
        result.set_int("instructionCount", count as i32)?;

        let writer = writer::create_writer(self.mode);
        let element_class_name = writer.get_instruction_class();

        let expected_class = self.env.find_class(element_class_name)?;

        let array = self.ensure_instructions_array(count as i32, element_class_name)?;

        for i in 0..count {
            let insn_object = {
                let instruction = self.env.get_object_array_element(&array, i as jsize)?;
                self.env.auto_local(instruction)
            };

            if insn_object.is_null() {
                return Err("instruction array was modified (null element)".into());
            }

            // We need to make sure that the element is the correct type
            let insn_class = {
                let class = self.env.get_object_class(&insn_object)?;
                self.env.auto_local(class)
            };

            if !self.env.is_same_object(insn_class, &expected_class)? {
                return Err(format!(
                    "instruction type mismatch, expected: {}",
                    element_class_name
                )
                .into());
            }

            writer.write(
                self.env,
                &insn_object,
                instructions.get(i).unwrap(),
                self.capstone,
            )?;
        }

        Ok(())
    }

    /// Ensure the 'instructions' field in the output object has enough space for the given number of instructions.
    fn ensure_instructions_array(
        &mut self,
        required_size: i32,
        element_insn_class: &str,
    ) -> JResult<JObjectArray<'jni>> {
        // Grab the 'instructions' field
        let field_id = {
            let result_class = self.env.get_object_class(self.result_object)?;
            self.env.get_field_id(
                result_class,
                "instructions",
                format!("[{}", BASE_INSTRUCTION_CLASS),
            )?
        };

        let instructions = JObjectArray::from(
            self.env
                .get_field_unchecked(self.result_object, field_id, ReturnType::Array)?
                .l()?,
        );

        // Check to see if there is enough space in the array if it isn't null
        if !instructions.is_null() {
            let array_size = self.env.get_array_length(&instructions)?;
            if array_size >= required_size {
                // We're able to reuse the existing array
                return Ok(instructions);
            }
        }

        let base_insn_class = self.env.find_class(BASE_INSTRUCTION_CLASS)?;

        // Allocate new array with required size
        let new_array =
            self.env
                .new_object_array(required_size, &base_insn_class, JObject::null())?;

        let element_insn_class = self.env.find_class(element_insn_class)?;

        // Allocate and construct each element and put them into the array
        for i in 0..required_size {
            let instruction = {
                let instruction = self.env.new_object(&element_insn_class, "()V", &[])?;
                self.env.auto_local(instruction)
            };

            self.env
                .set_object_array_element(&new_array, i, instruction)?;
        }

        // Update the 'instructions' field
        self.env
            .set_field_unchecked(self.result_object, field_id, JValue::Object(&new_array))?;

        Ok(new_array)
    }
}
