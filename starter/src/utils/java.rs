use std::{ptr::null_mut, slice};

use jni_sys::{jint, jstring, JNIEnv};

macro_rules! jcall {
    ($ctx:expr, $func_name:ident) => {
        unsafe {
            (*(*($ctx))).$func_name.expect(concat!(stringify!($func_name), " unavailable"))($ctx)
        }
    };
    ($ctx:expr, $func_name:ident, $($args:expr),*) => {
        unsafe {
            (*(*($ctx))).$func_name.expect(concat!(stringify!($func_name), " unavailable"))($ctx, $($args),*)
        }
    };
}

pub(crate) use jcall;

pub trait JStringExt {
    fn from_java_string(env: *mut JNIEnv, str: jstring) -> Self;
    fn to_java_string(&self, env: *mut JNIEnv) -> jstring;
}

impl JStringExt for String {
    fn from_java_string(env: *mut JNIEnv, str: jstring) -> Self {
        let length = jcall!(env, GetStringLength, str);
        let chars = jcall!(env, GetStringChars, str, null_mut());
        let result = String::from_utf16(unsafe { slice::from_raw_parts(chars, length as usize) });
        jcall!(env, ReleaseStringChars, str, chars);

        result.unwrap()
    }

    fn to_java_string(&self, env: *mut JNIEnv) -> jstring {
        let chars = self.encode_utf16().collect::<Vec<_>>();

        jcall!(env, NewString, chars.as_ptr(), chars.len() as jint)
    }
}
