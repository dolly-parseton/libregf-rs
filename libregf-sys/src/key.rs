use crate::{handle_err_and_option, value::RegfValue, RegfError};
use std::ptr;

#[derive(Debug)]
pub struct RegfKey {
    pub inner: *mut isize,
}

impl Default for RegfKey {
    fn default() -> Self {
        Self {
            inner: ptr::null_mut(),
        }
    }
}

impl Drop for RegfKey {
    fn drop(&mut self) {
        let mut error = None;
        unsafe { unsafe_fn::_key_free(self, &mut error) };
        if let Some(error) = error {
            eprintln!("{}", error);
        }
    }
}

impl RegfKey {
    pub fn get_value_by_name(&self, name: &str) -> Result<RegfValue, RegfError> {
        RegfValue::value_by_name(self, name)
    }
    pub fn get_value_by_index(&self, index: usize) -> Result<RegfValue, RegfError> {
        RegfValue::value_by_index(self, index)
    }
    pub fn get_number_of_values(&self) -> Result<usize, RegfError> {
        RegfValue::count(self)
    }
    pub fn get_sub_keys(&self) -> Result<Vec<Self>, RegfError> {
        let mut sub_keys = None;
        let mut error = None;
        unsafe { unsafe_fn::_key_sub_keys(self, &mut sub_keys, &mut error) };
        handle_err_and_option(
            sub_keys,
            error,
            RegfError::function_returned_none("_key_sub_keys"),
        )
    }
    pub fn get_sub_key(&self, index: usize) -> Result<Self, RegfError> {
        let mut sub_keys = None;
        let mut error = None;
        unsafe { unsafe_fn::_key_sub_key_by_index(self, index as i32, &mut sub_keys, &mut error) };
        handle_err_and_option(
            sub_keys,
            error,
            RegfError::function_returned_none("_key_sub_keys"),
        )
    }
    pub fn get_sub_keys_len(&self) -> Result<usize, RegfError> {
        let mut sub_keys_n = None;
        let mut error = None;
        unsafe { unsafe_fn::_key_sub_keys_n(self, &mut sub_keys_n, &mut error) };
        handle_err_and_option(
            sub_keys_n,
            error,
            RegfError::function_returned_none("_key_sub_keys_n"),
        )
    }
    pub fn get_last_written(&self) -> Result<u64, RegfError> {
        let mut timestamp = None;
        let mut error = None;
        unsafe { unsafe_fn::_key_last_written(self, &mut timestamp, &mut error) };
        handle_err_and_option(
            timestamp,
            error,
            RegfError::function_returned_none("_key_last_written"),
        )
    }
    pub fn get_name(&self) -> Result<String, RegfError> {
        let mut name = None;
        let mut error = None;
        unsafe { unsafe_fn::_key_name(self, &mut name, &mut error) };
        handle_err_and_option(name, error, RegfError::function_returned_none("_key_name"))
    }
    pub fn get_class_name(&self) -> Result<String, RegfError> {
        let mut name = None;
        let mut error = None;
        unsafe { unsafe_fn::_key_class_name(self, &mut name, &mut error) };
        handle_err_and_option(
            name,
            error,
            RegfError::function_returned_none("_key_class_name"),
        )
    }
    pub fn get_security_descriptor(&self) -> Result<String, RegfError> {
        let mut security_descriptor = None;
        let mut error = None;
        unsafe { unsafe_fn::_key_security_descriptor(self, &mut security_descriptor, &mut error) };
        handle_err_and_option(
            security_descriptor,
            error,
            RegfError::function_returned_none("_key_security_descriptor"),
        )
    }
}

mod unsafe_fn {
    //
    use super::*;
    use crate::*;
    //
    pub unsafe fn _key_free(key: &mut RegfKey, error: &mut Option<RegfError>) {
        let mut err = ptr::null_mut();
        if libregf_key_free(&mut key.inner, &mut err) == -1 {
            *error = RegfError::from_ptr(err);
        }
    }
    //
    pub unsafe fn _key_last_written(
        key: &RegfKey,
        timestamp: &mut Option<u64>,
        error: &mut Option<RegfError>,
    ) {
        let mut ts = 0;
        let mut err = ptr::null_mut();
        match libregf_key_get_last_written_time(key.inner, &mut ts, &mut err) {
            -1 => *error = RegfError::from_ptr(err),
            _ => *timestamp = Some(ts),
        }
    }
    //
    read_string!(
        _key_name,
        RegfKey,
        "",
        libregf_key_get_name_size,
        libregf_key_get_name
    );
    //
    read_string!(
        _key_class_name,
        RegfKey,
        "",
        libregf_key_get_class_name_size,
        libregf_key_get_class_name
    );
    //
    read_string!(
        _key_security_descriptor,
        RegfKey,
        "",
        libregf_key_get_security_descriptor_size,
        libregf_key_get_security_descriptor
    );
    //
    pub unsafe fn _key_sub_key_by_index(
        key: &RegfKey,
        index: i32,
        sub_key: &mut Option<RegfKey>,
        error: &mut Option<RegfError>,
    ) {
        let mut err = ptr::null_mut();
        let mut sub_key_ptr = ptr::null_mut();
        match libregf_key_get_sub_key(key.inner, index, &mut sub_key_ptr, &mut err) {
            -1 => *error = RegfError::from_ptr(err),
            _ => *sub_key = Some(RegfKey { inner: sub_key_ptr }),
        }
    }
    //
    pub unsafe fn _key_sub_keys(
        key: &RegfKey,
        sub_keys: &mut Option<Vec<RegfKey>>,
        error: &mut Option<RegfError>,
    ) {
        let mut err = ptr::null_mut();
        let mut size = 0;
        match libregf_key_get_number_of_sub_keys(key.inner, &mut size, &mut err) {
            -1 => *error = RegfError::from_ptr(err),
            _ => {
                *sub_keys = Some(Vec::new());
                for i in 0..size {
                    let mut sub_key_ptr = ptr::null_mut();
                    match libregf_key_get_sub_key(key.inner, i, &mut sub_key_ptr, &mut err) {
                        -1 => *error = RegfError::from_ptr(err),
                        _ => {
                            sub_keys
                                .as_mut()
                                .map(|k| k.push(RegfKey { inner: sub_key_ptr }));
                        }
                    }
                }
            }
        }
    }
    //
    pub unsafe fn _key_sub_keys_n(
        key: &RegfKey,
        sub_keys_n: &mut Option<usize>,
        error: &mut Option<RegfError>,
    ) {
        let mut err = ptr::null_mut();
        let mut size = 0;
        match libregf_key_get_number_of_sub_keys(key.inner, &mut size, &mut err) {
            -1 => *error = RegfError::from_ptr(err),
            _ => *sub_keys_n = Some(size as usize),
        }
    }
}
