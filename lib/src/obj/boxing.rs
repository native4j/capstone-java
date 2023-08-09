/*
 * Copyright 2023 The Native4J Authors
 *
 * Use of this source code is governed by the MIT license found in the LICENSE file.
 */
#![allow(dead_code)]
use jni::errors::Error;
use jni::objects::{AutoLocal, JObject, JValue};
use jni::JNIEnv;
use paste::paste;

pub type BoxedResult<'jni> = Result<AutoLocal<'jni, JObject<'jni>>, Error>;
macro_rules! boxed_type {
    ($value_name:ident, $fn_name:ident, $t:ty, $class_name:literal, $sig:literal) => {
        paste! {
            pub fn [<boxed_ $fn_name>]<'jni>(env: &mut JNIEnv<'jni>, value: $t) -> BoxedResult<'jni> {
                let obj = env.new_object($class_name, $sig, &[JValue::$value_name(value)])?;
                Ok(env.auto_local(obj))
            }
        }
    }
}

boxed_type!(Byte, byte, i8, "java/lang/Byte", "(B)V");
boxed_type!(Short, short, i16, "java/lang/Short", "(S)V");
boxed_type!(Int, int, i32, "java/lang/Integer", "(I)V");
boxed_type!(Long, long, i64, "java/lang/Long", "(J)V");
boxed_type!(Double, double, f64, "java/lang/Double", "(D)V");
