/*
 * Copyright 2023 The Native4J Authors
 *
 * Use of this source code is governed by the MIT license found in the LICENSE file.
 */
use jni::errors::Error;
use jni::objects::{AutoLocal, JObject};

#[macro_export]
macro_rules! check_result {
    ($env:expr, $e:expr) => {
        if let Err(ref e) = $e {
            let str = $env.new_string(format!("{}:{}: {}", file!(), line!(), e.to_string()));
            if let Err(ref e) = str {
                eprintln!("failed to create string: {}", e);
            } else {
                return str.unwrap().into_raw();
            }
        }
    };
}

#[macro_export]
macro_rules! make_jstring {
    ($env:expr, $str:expr) => {
        $env.new_string($str).unwrap().into_raw()
    };
}

#[macro_export]
macro_rules! make_error {
    ($env:expr, $str:expr) => {
        $env.new_string(format!("{}:{}: {}", file!(), line!(), $str))
            .unwrap()
            .into_raw()
    };
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! dbgln {
    () => {
        println!();
    };
    ($($arg:tt)*) => {
        println!($($arg)*);
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! dbgln {
    () => {};
    ($($arg:tt)*) => {};
}

pub type JResult<T> = Result<T, Error>;
pub type AutoObject<'jni> = AutoLocal<'jni, JObject<'jni>>;
pub type JObjectResult<'jni> = JResult<AutoObject<'jni>>;
