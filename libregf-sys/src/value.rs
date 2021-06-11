use crate::{handle_err_and_option, key::RegfKey, RegfError, LIBREGF_VALUE_TYPES};
use std::ptr;

#[derive(Debug)]
pub struct RegfValue {
    pub inner: *mut isize,
}

impl Default for RegfValue {
    fn default() -> Self {
        Self {
            inner: ptr::null_mut(),
        }
    }
}

impl Drop for RegfValue {
    fn drop(&mut self) {
        let mut error = None;
        unsafe { unsafe_fn::_value_free(self, &mut error) };
        if let Some(error) = error {
            eprintln!("{}", error);
        }
    }
}

impl RegfValue {
    //
    pub fn count(key: &RegfKey) -> Result<usize, RegfError> {
        let mut values: i32 = 0;
        let mut error = None;
        unsafe {
            unsafe_fn::_number_of_values(key, &mut values, &mut error);
        }
        match error {
            Some(e) => Err(e),
            None => Ok(values as usize),
        }
    }
    pub fn value_by_name(key: &RegfKey, name: &str) -> Result<Self, RegfError> {
        let mut value: RegfValue = RegfValue::default();
        let mut error = None;
        unsafe {
            unsafe_fn::_value_by_name(key, name, &mut value, &mut error);
        }
        match error {
            Some(e) => Err(e),
            None => Ok(value),
        }
    }
    pub fn value_by_index(key: &RegfKey, index: usize) -> Result<Self, RegfError> {
        let mut value: RegfValue = RegfValue::default();
        let mut error = None;
        unsafe {
            unsafe_fn::_value_by_index(key, index, &mut value, &mut error);
        }
        match error {
            Some(e) => Err(e),
            None => Ok(value),
        }
    }
    //
    pub fn get_name(&self) -> Result<String, RegfError> {
        let mut name = None;
        let mut error = None;
        unsafe { unsafe_fn::_value_name(self, &mut name, &mut error) };
        handle_err_and_option(
            name,
            error,
            RegfError::function_returned_none("_value_name"),
        )
    }
    //
    pub fn get_type(&self) -> Result<LIBREGF_VALUE_TYPES, RegfError> {
        let mut r#type = None;
        let mut error = None;
        unsafe { unsafe_fn::_value_type(self, &mut r#type, &mut error) };
        handle_err_and_option(
            r#type,
            error,
            RegfError::function_returned_none("_value_type"),
        )
    }
    //
    // pub fn get_data(&self) -> Result<Vec<u8>, RegfError> {
    //     //
    // }
    pub fn get_u32(&self) -> Result<u32, RegfError> {
        let mut data = None;
        let mut error = None;
        unsafe { unsafe_fn::_value_u32(self, &mut data, &mut error) };
        handle_err_and_option(data, error, RegfError::function_returned_none("_value_u32"))
    }
    pub fn get_u64(&self) -> Result<u64, RegfError> {
        let mut data = None;
        let mut error = None;
        unsafe { unsafe_fn::_value_u64(self, &mut data, &mut error) };
        handle_err_and_option(data, error, RegfError::function_returned_none("_value_u64"))
    }
    pub fn get_string(&self) -> Result<String, RegfError> {
        let mut string = None;
        let mut error = None;
        unsafe { unsafe_fn::_value_string(self, &mut string, &mut error) };
        handle_err_and_option(
            string,
            error,
            RegfError::function_returned_none("_value_string"),
        )
    }
    pub fn get_multi_string(&self) -> Result<Vec<String>, RegfError> {
        let mut strings = None;
        let mut error = None;
        unsafe { unsafe_fn::_value_multi_string(self, &mut strings, &mut error) };
        handle_err_and_option(
            strings,
            error,
            RegfError::function_returned_none("_value_multi_string"),
        )
    }
}

mod unsafe_fn {
    //
    use super::*;
    use crate::*;
    use std::ffi;
    //
    pub unsafe fn _value_free(value: &mut RegfValue, error: &mut Option<RegfError>) {
        let mut err = ptr::null_mut();
        if libregf_value_free(&mut value.inner, &mut err) == -1 {
            *error = RegfError::from_ptr(err);
        }
    }
    //
    pub unsafe fn _value_u32(
        value: &RegfValue,
        data: &mut Option<u32>,
        error: &mut Option<RegfError>,
    ) {
        let mut err = ptr::null_mut();
        let mut v = 0;
        match libregf_value_get_value_32bit(value.inner, &mut v, &mut err) {
            -1 => *error = RegfError::from_ptr(err),
            _ => *data = Some(v),
        }
    }
    //
    pub unsafe fn _value_u64(
        value: &RegfValue,
        data: &mut Option<u64>,
        error: &mut Option<RegfError>,
    ) {
        let mut err = ptr::null_mut();
        let mut v = 0;
        match libregf_value_get_value_64bit(value.inner, &mut v, &mut err) {
            -1 => *error = RegfError::from_ptr(err),
            _ => *data = Some(v),
        }
    }
    //
    pub unsafe fn _value_string(
        value: &RegfValue,
        string: &mut Option<String>,
        error: &mut Option<RegfError>,
    ) {
        let mut err: *mut libregf_error_t = ptr::null_mut();
        let mut size = 0;
        if libregf_value_get_value_utf8_string_size(value.inner, &mut size, &mut err) == -1 {
            *error = RegfError::from_ptr(err);
        } else {
            let string_ptr = ptr::null_mut();
            if libregf_value_get_value_utf8_string(value.inner, string_ptr, size, &mut err) == -1 {
                *error = RegfError::from_ptr(err);
            } else {
                if let Ok(s) =
                    std::str::from_utf8(std::slice::from_raw_parts(string_ptr, size as usize))
                {
                    *string = Some(s.to_string());
                }
            }
        }
    }
    //
    pub unsafe fn _value_multi_string(
        value: &RegfValue,
        strings: &mut Option<Vec<String>>,
        error: &mut Option<RegfError>,
    ) {
        let mut err: *mut libregf_error_t = ptr::null_mut();
        let mut n = 0;
        let strings_ptr = ptr::null_mut();
        match libregf_multi_string_get_number_of_strings(value.inner, &mut n, &mut err) {
            -1 => *error = RegfError::from_ptr(err),
            _ => match libregf_value_get_value_multi_string(value.inner, strings_ptr, &mut err) {
                -1 => *error = RegfError::from_ptr(err),
                _ => {
                    let mut raw = Vec::from_raw_parts(strings_ptr, n as usize, n as usize);
                    *strings = Some(
                        raw.drain(..)
                            .map(|r| {
                                ffi::CString::from_raw(*r as *mut i8)
                                    .to_str()
                                    .map(|s| s.to_string())
                            })
                            .filter(|s| s.is_ok())
                            .map(|s| s.unwrap())
                            .collect(),
                    )
                }
            },
        }
    }
    //
    pub unsafe fn _number_of_values(
        key: &RegfKey,
        number_of_values: &mut i32,
        error: &mut Option<RegfError>,
    ) {
        let mut err = ptr::null_mut();
        if libregf_key_get_number_of_values(key.inner, number_of_values, &mut err) == -1 {
            *error = RegfError::from_ptr(err);
        }
    }
    //
    pub unsafe fn _value_type(
        value: &RegfValue,
        r#type: &mut Option<LIBREGF_VALUE_TYPES>,
        error: &mut Option<RegfError>,
    ) {
        let mut type_value = 12;
        let mut err = ptr::null_mut();
        if libregf_value_get_value_type(value.inner, &mut type_value, &mut err) == -1 {
            *error = RegfError::from_ptr(err);
        } else {
            *r#type = match type_value {
                0 => Some(LIBREGF_VALUE_TYPES::LIBREGF_VALUE_TYPE_UNDEFINED),
                1 => Some(LIBREGF_VALUE_TYPES::LIBREGF_VALUE_TYPE_STRING),
                2 => Some(LIBREGF_VALUE_TYPES::LIBREGF_VALUE_TYPE_EXPANDABLE_STRING),
                3 => Some(LIBREGF_VALUE_TYPES::LIBREGF_VALUE_TYPE_BINARY_DATA),
                4 => Some(LIBREGF_VALUE_TYPES::LIBREGF_VALUE_TYPE_INTEGER_32BIT_LITTLE_ENDIAN),
                5 => Some(LIBREGF_VALUE_TYPES::LIBREGF_VALUE_TYPE_INTEGER_32BIT_BIG_ENDIAN),
                6 => Some(LIBREGF_VALUE_TYPES::LIBREGF_VALUE_TYPE_SYMBOLIC_LINK),
                7 => Some(LIBREGF_VALUE_TYPES::LIBREGF_VALUE_TYPE_MULTI_VALUE_STRING),
                8 => Some(LIBREGF_VALUE_TYPES::LIBREGF_VALUE_TYPE_RESOURCE_LIST),
                9 => Some(LIBREGF_VALUE_TYPES::LIBREGF_VALUE_TYPE_FULL_RESOURCE_DESCRIPTOR),
                10 => Some(LIBREGF_VALUE_TYPES::LIBREGF_VALUE_TYPE_RESOURCE_REQUIREMENTS_LIST),
                11 => Some(LIBREGF_VALUE_TYPES::LIBREGF_VALUE_TYPE_INTEGER_64BIT_LITTLE_ENDIAN),
                _ => None,
            };
        }
    }
    //
    pub unsafe fn _value_by_index(
        key: &RegfKey,
        index: usize,
        value: &mut RegfValue,
        error: &mut Option<RegfError>,
    ) {
        let mut err = ptr::null_mut();
        if libregf_key_get_value(key.inner, index as i32, &mut value.inner, &mut err) == -1 {
            *error = RegfError::from_ptr(err);
        }
    }
    //
    pub unsafe fn _value_by_name(
        key: &RegfKey,
        utf8_string: &str,
        value: &mut RegfValue,
        error: &mut Option<RegfError>,
    ) {
        let mut err = ptr::null_mut();
        if libregf_key_get_value_by_utf8_name(
            key.inner,
            utf8_string.as_bytes().as_ptr(),
            utf8_string.as_bytes().len() as u64,
            &mut value.inner,
            &mut err,
        ) == -1
        {
            *error = RegfError::from_ptr(err);
        }
    }
    //
    pub unsafe fn _value_name(
        value: &RegfValue,
        name: &mut Option<String>,
        error: &mut Option<RegfError>,
    ) {
        let mut err: *mut libregf_error_t = ptr::null_mut();
        let mut size = 0;
        if libregf_value_get_name_size(value.inner, &mut size, &mut err) == -1 {
            println!("Bad name size {:?}", size);
            *error = RegfError::from_ptr(err);
        } else {
            println!("Name len {:?}", size);
            let name_ptr = ptr::null_mut();
            if libregf_value_get_name(value.inner, name_ptr, size, &mut err) == -1 {
                println!("Bad name {:?} / {:?} / {:?}", size, name_ptr, err);
                *error = RegfError::from_ptr(err);
            } else {
                if let Ok(s) =
                    std::str::from_utf8(std::slice::from_raw_parts(name_ptr, size as usize))
                {
                    *name = Some(s.to_string());
                }
            }
        }
    }
}
