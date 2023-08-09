/*
 * Copyright 2023 The Native4J Authors
 *
 * Use of this source code is governed by the MIT license found in the LICENSE file.
 */
#[macro_export]
macro_rules! set_array_impl {
    ($array_type:ty, $name:literal, $sig:literal, $src_ty:ty, $env:expr, $obj:expr, $class:expr, $field:expr, $src:expr) => {
        let field_id = $env.get_field_id(&$class, $field, $sig)?;

        // Set the Java array to null if src is empty
        if $src.is_empty() {
            $env.set_field_unchecked($obj, field_id, JValue::Object(&JObject::null()))?;
            return Ok(());
        }

        let array = <$array_type>::from(
            $env.get_field_unchecked($obj, field_id, ReturnType::Array)?
                .l()?,
        );

        let required_size = $src.len() as i32;

        let size = if array.is_null() {
            None
        } else {
            Some($env.get_array_length(&array)?)
        };

        let (array, is_new_array) = if size.is_none() || size.unwrap() < required_size {
            paste! {
                ($env.[<new_ $name _array>](required_size)?, true)
            }
        } else {
            (array, false)
        };

        let mut elements: AutoElements<$src_ty> =
            unsafe { $env.get_array_elements(&array, ReleaseMode::CopyBack)? };

        // Copy elements
        for i in 0..required_size as usize {
            elements[i] = $src[i];
        }

        // We need to update the field if we've just allocated a new array
        if is_new_array {
            $env.set_field_unchecked($obj, field_id, JValue::Object(&array))?;
        }

        return Ok(());
    };
}
