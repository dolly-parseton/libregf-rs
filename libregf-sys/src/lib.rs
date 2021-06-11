#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub mod file;
pub mod key;
pub mod value;

use std::{error, fmt, ptr};

pub fn handle_err_and_option<T, E>(v: Option<T>, e: Option<E>, o: E) -> Result<T, E> {
    match (e, v) {
        (Some(error), _) => Err(error),
        (None, Some(t)) => Ok(t),
        (None, None) => Err(o),
    }
}

#[derive(Debug)]
pub struct RegfError {
    inner: *mut isize,
    err: String,
}

impl RegfError {
    /// Creates a RegfError struct using the error pointer, if the error pointer does not reference a valid error returns None. Might want to change this in future.
    pub fn from_ptr(ptr: *mut isize) -> Option<Self> {
        let mut err = None;
        unsafe { unsafe_fn::_error_read(ptr, &mut err) };
        err.map(|err| Self { inner: ptr, err })
    }

    pub fn function_returned_none(func: &str) -> Self {
        RegfError {
            inner: ptr::null_mut(),
            err: format!(
                "unsafe function '{}' failed, name string returned is None.",
                func
            ),
        }
    }
}

impl error::Error for RegfError {}

impl fmt::Display for RegfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RegfError: {}", self.err)
    }
}

mod unsafe_fn {
    //
    use super::*;
    //
    pub unsafe fn _error_read(mut ptr: *mut isize, _error: &mut Option<String>) {
        // let err_string = ptr::null_mut();
        // let err_string_n = 0;
        let len = libregf_error_backtrace_fprint(ptr, stderr);
        println!("Err len: {:?} / {:?}", len, ptr);
        // use std::convert::TryInto;
        // let len = libregf_error_backtrace_sprint(ptr, err_string, 141);
        // println!("Err len: {:?} / {:?}", len, err_string);
        // println!("Err: {:?}", ptr);
        // if len == -1 {
        //     *error = Some(String::from(
        //         "Unable to create error message from error pointer.",
        //     ))
        // } else {
        //     *error = match ffi::CString::from_raw(err_string).into_string() {
        //         Ok(s) => Some(s),
        //         Err(_) => Some(String::from(
        //             "Unable to create error message string from raw.",
        //         )),
        //     }
        // }
        libregf_error_free(&mut ptr);
    }
}
