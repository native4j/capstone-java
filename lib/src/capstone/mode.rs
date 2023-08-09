/*
 * Copyright 2023 The Native4J Authors
 *
 * Use of this source code is governed by the MIT license found in the LICENSE file.
 */
use std::fmt::{Display, Formatter};

use jni::objects::JObject;
use jni::JNIEnv;

use crate::capstone::mode::CapstoneMode::{ARM32, ARM64};
use crate::obj::obj;

#[derive(Debug)]
pub enum CapstoneMode {
    ARM32 = 0,
    ARM64 = 1,
}

impl CapstoneMode {
    pub fn from(env: &mut JNIEnv, object: &JObject) -> Option<CapstoneMode> {
        let ord = obj::get_enum_ordinal(env, object);
        match ord {
            Some(0) => Some(ARM32),
            Some(1) => Some(ARM64),
            _ => None,
        }
    }
}

impl Display for CapstoneMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ARM32 => f.write_str("ARM32"),
            ARM64 => f.write_str("ARM64"),
        }
    }
}
