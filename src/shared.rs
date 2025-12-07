use std::{ffi::CStr, str::Utf8Error};

use gl::types::GLenum;

pub fn get_string(value: GLenum) -> Result<String, Utf8Error> {
    unsafe {
        let raw_ptr = gl::GetString(value);
        CStr::from_ptr(raw_ptr as _)
            .to_str()
            .map(|it| it.to_string())
    }
}