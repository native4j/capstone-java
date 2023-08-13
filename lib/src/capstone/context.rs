/*
 * Copyright 2023 The Native4J Authors
 *
 * Use of this source code is governed by the MIT license found in the LICENSE file.
 */
use std::sync::{Arc, Mutex, MutexGuard};

use capstone::Capstone;
use jni::objects::JObject;
use jni::JNIEnv;

use crate::capstone::mode::CapstoneMode;
use crate::util::JResult;

/// A Capstone instance that can be stored into a Java object.
pub struct CapstoneContext {
    pub capstone: Mutex<Capstone>,
    pub mode: CapstoneMode,
}

unsafe impl Send for CapstoneContext {}
unsafe impl Sync for CapstoneContext {}

const HANDLE_FIELD: &str = "_CsHandle";

impl CapstoneContext {
    pub fn new(capstone: Capstone, mode: CapstoneMode) -> Arc<CapstoneContext> {
        Arc::new(CapstoneContext {
            capstone: Mutex::new(capstone),
            mode,
        })
    }

    /// Clones an Arc out of the handle field of the given object.
    pub fn get(env: &mut JNIEnv, object: &JObject) -> JResult<Arc<CapstoneContext>> {
        let guard: MutexGuard<Arc<CapstoneContext>> =
            unsafe { env.get_rust_field(object, HANDLE_FIELD)? };
        Ok(guard.clone())
    }

    /// Surrenders ownership of the context instance to Java.
    pub fn surrender_instance(
        instance: Arc<CapstoneContext>,
        env: &mut JNIEnv,
        object: &JObject,
    ) -> JResult<()> {
        unsafe { env.set_rust_field(object, HANDLE_FIELD, instance) }
    }

    /// Takes ownership of the context instance from Java and drops it.
    pub fn drop_instance(env: &mut JNIEnv, object: &JObject) -> JResult<()> {
        let ctx: Arc<CapstoneContext> = unsafe { env.take_rust_field(object, HANDLE_FIELD)? };
        drop(ctx);
        Ok(())
    }
}
