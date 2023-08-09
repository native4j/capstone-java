/*
 * Copyright 2023 The Native4J Authors
 *
 * Use of this source code is governed by the MIT license found in the LICENSE file.
 */
use jni::objects::AutoElements;
use jni::objects::JShortArray;
use jni::objects::ReleaseMode;
use jni::objects::{JClass, JObject, JValue};
use jni::signature::ReturnType;
use jni::strings::JNIString;
use jni::JNIEnv;
use paste::paste;

use crate::set_array_impl;
use crate::util::JResult;

/// Java object wrapper.
pub struct Obj<'jni, 'a> {
    env: &'a mut JNIEnv<'jni>,
    obj: &'a JObject<'jni>,
    obj_class: JClass<'jni>,
}

impl<'jni, 'a> Obj<'jni, 'a> {
    pub fn from(env: &'a mut JNIEnv<'jni>, obj: &'a JObject<'jni>) -> Obj<'jni, 'a> {
        let obj_class = env.get_object_class(obj).unwrap();
        Obj {
            env,
            obj,
            obj_class,
        }
    }

    pub fn env<'b>(&'b mut self) -> &'b mut JNIEnv<'jni> {
        self.env
    }

    /// Set a byte field.
    pub fn set_byte<S>(&mut self, field: S, value: i8) -> JResult<()>
    where
        S: Into<JNIString>,
    {
        self.set_primitive_field(field, JValue::Byte(value))
    }

    // Set a bool field.
    pub fn set_bool<S>(&mut self, field: S, value: bool) -> JResult<()>
    where
        S: Into<JNIString>,
    {
        self.set_primitive_field(field, JValue::Bool(value as u8))
    }

    /// Set an int field.
    pub fn set_int<S>(&mut self, field: S, value: i32) -> JResult<()>
    where
        S: Into<JNIString>,
    {
        self.set_primitive_field(field, JValue::Int(value))
    }

    /// Set an Object field.
    pub fn set_object<S>(&mut self, field: S, sig: &str, value: &JObject<'jni>) -> JResult<()>
    where
        S: Into<JNIString>,
    {
        self.set_field(field, sig, JValue::Object(value))
    }

    /// Set a long field.
    pub fn set_long<S>(&mut self, field: S, value: i64) -> JResult<()>
    where
        S: Into<JNIString>,
    {
        self.set_primitive_field(field, JValue::Long(value))
    }

    /// Set a string field.
    pub fn set_str<S>(&mut self, field: S, value: S) -> JResult<()>
    where
        S: Into<JNIString>,
    {
        let field_id =
            self.env
                .get_field_id(&self.obj_class, field.into(), "Ljava/lang/String;")?;
        let str = self.env.auto_local(self.env.new_string(value)?);
        self.env
            .set_field_unchecked(self.obj, field_id, JValue::Object(str.as_ref()))
    }

    /// Create a short array and set it to a field.
    pub fn set_short_array<S>(&mut self, field: S, src: &[i16]) -> JResult<()>
    where
        S: Into<JNIString>,
    {
        set_array_impl!(
            JShortArray,
            "short",
            "[S",
            i16,
            self.env,
            self.obj,
            self.obj_class,
            field,
            src
        );
    }

    /// Create an object array and set it to a field.
    pub fn set_object_array<'b, S, O>(
        &mut self,
        field: S,
        element_class: &str,
        src: &[O],
    ) -> JResult<()>
    where
        S: Into<JNIString>,
        O: AsRef<JObject<'b>>,
    {
        let array = {
            let array =
                self.env
                    .new_object_array(src.len() as i32, element_class, JObject::null())?;
            self.env.auto_local(array)
        };

        for (i, obj) in src.iter().enumerate() {
            self.env.set_object_array_element(&array, i as i32, obj)?;
        }

        self.set_field(
            field,
            &format!("[{}", element_class),
            JValue::Object(&array),
        )
    }

    /// Set a primitive field.
    fn set_primitive_field<S>(&mut self, field: S, value: JValue) -> JResult<()>
    where
        S: Into<JNIString>,
    {
        let field_id =
            self.env
                .get_field_id(&self.obj_class, field, Self::primitive_sig(&value))?;
        self.env.set_field_unchecked(self.obj, field_id, value)?;
        Ok(())
    }

    /// Set a field.
    fn set_field<S>(&mut self, field: S, sig: &str, value: JValue) -> JResult<()>
    where
        S: Into<JNIString>,
    {
        let field_id = self.env.get_field_id(&self.obj_class, field, sig)?;
        self.env.set_field_unchecked(self.obj, field_id, value)?;
        Ok(())
    }

    /// Get the signature of a primitive JValue.
    fn primitive_sig(value: &JValue) -> &'static str {
        match value {
            JValue::Object(_) => panic!("use set_field instead"),
            JValue::Byte(_) => "B",
            JValue::Char(_) => "C",
            JValue::Short(_) => "S",
            JValue::Int(_) => "I",
            JValue::Long(_) => "J",
            JValue::Bool(_) => "Z",
            JValue::Float(_) => "F",
            JValue::Double(_) => "D",
            _ => "?",
        }
    }
}

/// Get the ordinal of an enum object.
pub fn get_enum_ordinal(env: &mut JNIEnv, enum_obj: &JObject) -> Option<i32> {
    let result = env.call_method(enum_obj, "ordinal", "()I", &[]);
    if result.is_err() {
        return None;
    }
    Some(result.unwrap().i().unwrap())
}
