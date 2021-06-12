use crate::{handle_err_and_option, key::RegfKey, RegfError};
use std::{mem, ptr};

#[derive(Debug)]
pub enum RegfType {
    Undefined = 0,
    String = 1,
    ExpandableString = 2,
    Binary = 3,
    Int32LE = 4,
    Int32BE = 5,
    SymbolicLink = 6,
    MultiValueString = 7,
    ResourceList = 8,
    FullResourceDescription = 9,
    ResourceRequirementsList = 10,
    Int64LE = 11,
}

impl From<u32> for RegfType {
    fn from(r#type: u32) -> Self {
        match r#type {
            1 => Self::String,
            2 => Self::ExpandableString,
            3 => Self::Binary,
            4 => Self::Int32LE,
            5 => Self::Int32BE,
            6 => Self::SymbolicLink,
            7 => Self::MultiValueString,
            8 => Self::ResourceList,
            9 => Self::FullResourceDescription,
            10 => Self::ResourceRequirementsList,
            11 => Self::Int64LE,
            _ => Self::Undefined,
        }
    }
}

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
    pub fn get_type(&self) -> Result<RegfType, RegfError> {
        let mut r#type = None;
        let mut error = None;
        unsafe { unsafe_fn::_value_type(self, &mut r#type, &mut error) };
        handle_err_and_option(
            r#type,
            error,
            RegfError::function_returned_none("_value_type"),
        )
    }
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
    pub fn get_binary(&self) -> Result<&[u8], RegfError> {
        let mut data = None;
        let mut error = None;
        unsafe { unsafe_fn::_value_binary(self, &mut data, &mut error) };
        handle_err_and_option(
            data,
            error,
            RegfError::function_returned_none("_value_binary"),
        )
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
    pub unsafe fn _value_binary(
        value: &RegfValue,
        binary: &mut Option<&[u8]>,
        error: &mut Option<RegfError>,
    ) {
        let mut err: *mut libregf_error_t = ptr::null_mut();
        let mut size = 0;
        let mut binary_ptr: [u8; 16383 * 4] = mem::zeroed();
        match libregf_value_get_value_binary_data_size(value.inner, &mut size, &mut err) {
            -1 => *error = RegfError::from_ptr(err),
            _ => match size == 0 {
                false => match libregf_value_get_value_binary_data(
                    value.inner,
                    binary_ptr.as_mut_ptr() as *mut u8,
                    size,
                    &mut err,
                ) {
                    -1 => *error = RegfError::from_ptr(err),
                    _ => {
                        *binary = Some(std::slice::from_raw_parts_mut(
                            binary_ptr.as_mut_ptr() as *mut u8,
                            size as usize,
                        ))
                    }
                },
                true => *binary = Some(&[]),
            },
        }
    }
    //
    read_string!(
        _value_string,
        libregf_value_get_value_utf8_string_size,
        libregf_value_get_value_utf8_string
    );
    //
    // pub unsafe fn _value_string(
    //     value: &RegfValue,
    //     string: &mut Option<String>,
    //     error: &mut Option<RegfError>,
    // ) {
    //     let mut err: *mut libregf_error_t = ptr::null_mut();
    //     let mut size = 0;
    //     let mut string_ptr: [u8; 265 * 4] = mem::zeroed();
    //     match libregf_value_get_value_utf8_string_size(value.inner, &mut size, &mut err) {
    //         -1 => *error = RegfError::from_ptr(err),
    //         _ => match size == 0 {
    //             false => match libregf_value_get_value_utf8_string(
    //                 value.inner,
    //                 string_ptr.as_mut_ptr() as *mut u8,
    //                 size,
    //                 &mut err,
    //             ) {
    //                 -1 => *error = RegfError::from_ptr(err),
    //                 _ => {
    //                     *string = std::str::from_utf8(&string_ptr[..size as usize])
    //                         .map(|s| s.to_string())
    //                         .ok()
    //                 }
    //             },
    //             true => *string = Some("".to_string()),
    //         },
    //     }
    // }
    //
    pub unsafe fn _value_multi_string(
        value: &RegfValue,
        strings: &mut Option<Vec<String>>,
        error: &mut Option<RegfError>,
    ) {
        let mut err: *mut libregf_error_t = ptr::null_mut();
        let mut n = 0;
        let mut strings_ptr: [*mut libregf_multi_string_t; 16383 * 4] = mem::zeroed();
        // let mut strings_ptr: *mut libregf_multi_string_t = ptr::null_mut();
        match libregf_multi_string_get_number_of_strings(value.inner, &mut n, &mut err) {
            -1 => *error = RegfError::from_ptr(err),
            _ => {
                match libregf_value_get_value_multi_string(
                    value.inner,
                    strings_ptr.as_mut_ptr(), // as *mut *mut libregf_multi_string_t,
                    &mut err,
                ) {
                    -1 => *error = RegfError::from_ptr(err),
                    _ => (), // string_ptrs = Some(Vec::from_raw_parts(strings_ptr, n as usize, n as usize)),
                }
            }
        }

        for i in 0..n {
            let mut size = 0;
            match libregf_multi_string_get_utf8_string_size(
                strings_ptr.as_mut_ptr() as *mut isize,
                i,
                &mut size,
                &mut err,
            ) {
                -1 => *error = RegfError::from_ptr(err),
                _ => {
                    let mut string_ptr: [u8; 16383 * 4] = mem::zeroed();
                    match libregf_multi_string_get_utf8_string(
                        strings_ptr[i as usize],
                        i,
                        string_ptr.as_mut_ptr() as *mut u8,
                        size,
                        &mut err,
                    ) {
                        -1 => *error = RegfError::from_ptr(err),
                        _ => {
                            if strings.is_none() {
                                *strings = Some(Vec::new())
                            }
                            if let Some(ref mut v) = strings {
                                if let Ok(s) = std::str::from_utf8(&string_ptr[..size as usize]) {
                                    v.push(s.to_string());
                                }
                            }
                        }
                    }
                }
            }
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
        r#type: &mut Option<RegfType>,
        error: &mut Option<RegfError>,
    ) {
        let mut type_value = 0;
        let mut err = ptr::null_mut();
        match libregf_value_get_value_type(value.inner, &mut type_value, &mut err) {
            -1 => *error = RegfError::from_ptr(err),
            _ => *r#type = Some(RegfType::from(type_value)),
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
        // https://github.com/libyal/libregf/blob/main/documentation/Windows%20NT%20Registry%20File%20(REGF)%20format.asciidoc#1-overview - 16383 * 4
        let mut name_ptr: [u8; 16383 * 4] = mem::zeroed();
        match libregf_value_get_name_size(value.inner, &mut size, &mut err) == 1 {
            false => *error = RegfError::from_ptr(err),
            true => match libregf_value_get_name(
                value.inner,
                name_ptr.as_mut_ptr() as *mut u8,
                size,
                &mut err,
            ) == 1
            {
                false => *error = RegfError::from_ptr(err),
                true => {
                    *name = match size == 0 {
                        true => Some("(default)".to_string()),
                        false => std::str::from_utf8(&name_ptr[..size as usize])
                            .map(|s| s.to_string())
                            .ok(),
                    }
                }
            },
        }
    }
}
