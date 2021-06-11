use crate::{RegfError, RegfFile, RegfKey};
use std::{ffi, ptr};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub unsafe fn _error_read(mut ptr: *mut libregf_error_t, error: &mut Option<String>) {
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

pub unsafe fn _file_open(
    file: &mut RegfFile,
    error: &mut Option<RegfError>,
    cstring_path: ffi::CString,
) {
    let chars: *mut i8 = cstring_path.into_raw();
    let mut err: *mut libregf_error_t = ptr::null_mut();
    match libregf_check_file_signature(chars, &mut err) == 1 {
        true => match libregf_file_initialize(&mut file.inner, &mut err) == 1 {
            true => {
                if libregf_file_open(file.inner, chars, libregf_get_access_flags_read(), &mut err)
                    != 1
                {
                    *error = RegfError::from_ptr(err);
                }
            }
            false => *error = RegfError::from_ptr(err),
        },
        false => *error = RegfError::from_ptr(err), // do something with err
    }
}

pub unsafe fn _file_free(file: &mut RegfFile, error: &mut Option<RegfError>) {
    let mut err: *mut libregf_error_t = ptr::null_mut();
    if libregf_file_close(file.inner, &mut err) != 0 {
        *error = RegfError::from_ptr(err);
    }
}

pub unsafe fn _file_root(file: &RegfFile, root: &mut RegfKey, error: &mut Option<RegfError>) {
    let mut err: *mut libregf_error_t = ptr::null_mut();
    if libregf_file_get_root_key(file.inner, &mut root.inner, &mut err) != 1 {
        *error = RegfError::from_ptr(err);
    }
}

pub unsafe fn _key_free(key: &mut RegfKey, error: &mut Option<RegfError>) {
    let mut err: *mut libregf_error_t = ptr::null_mut();
    if libregf_key_free(&mut key.inner, &mut err) != 0 {
        *error = RegfError::from_ptr(err);
    }
}

pub unsafe fn _key_last_written(key: &RegfKey, timestamp: &mut u64, error: &mut Option<RegfError>) {
    let mut err: *mut libregf_error_t = ptr::null_mut();
    if libregf_key_get_last_written_time(key.inner, timestamp, &mut err) == -1 {
        *error = RegfError::from_ptr(err);
    }
}

pub unsafe fn _key_name(key: &RegfKey, name: &mut Option<String>, error: &mut Option<RegfError>) {
    let mut err: *mut libregf_error_t = ptr::null_mut();
    let mut size = 0;
    if libregf_key_get_name_size(key.inner, &mut size, &mut err) == -1 {
        println!("Bad name size {:?}", size);
        *error = RegfError::from_ptr(err);
    } else {
        println!("Name len {:?}", size);
        let name_ptr = ptr::null_mut();
        if libregf_key_get_name(key.inner, name_ptr, size, &mut err) == -1 {
            println!("Bad name {:?} / {:?} / {:?}", size, name_ptr, err);
            *error = RegfError::from_ptr(err);
        } else {
            if let Ok(s) = std::str::from_utf8(std::slice::from_raw_parts(name_ptr, size as usize))
            {
                *name = Some(s.to_string());
            }
        }
    }
}

pub unsafe fn _key_class_name(
    key: &RegfKey,
    name: &mut Option<Option<String>>,
    error: &mut Option<RegfError>,
) {
    let mut err: *mut libregf_error_t = ptr::null_mut();
    let mut size = 0;
    if libregf_key_get_class_name_size(key.inner, &mut size, &mut err) == -1 {
        println!("Bad name size {:?}", size);
        *error = RegfError::from_ptr(err);
    } else {
        println!("Name len {:?}", size);
        let name_ptr = ptr::null_mut();
        let res = libregf_key_get_class_name(key.inner, name_ptr, size, &mut err);
        if res == -1 {
            println!("Bad name {:?} / {:?} / {:?}", size, name_ptr, err);
            *error = RegfError::from_ptr(err);
        } else if res == 1 {
            if let Ok(s) = std::str::from_utf8(std::slice::from_raw_parts(name_ptr, size as usize))
            {
                *name = Some(Some(s.to_string()));
            }
        } else if res == 0 {
            *name = Some(None);
        }
    }
}

pub unsafe fn _key_security_descriptor(
    key: &RegfKey,
    security_descriptor: &mut Option<Option<String>>,
    error: &mut Option<RegfError>,
) {
    let mut err: *mut libregf_error_t = ptr::null_mut();
    let mut size = 0;
    if libregf_key_get_security_descriptor_size(key.inner, &mut size, &mut err) == -1 {
        println!("Bad name size {:?}", size);
        *error = RegfError::from_ptr(err);
    } else {
        println!("Name len {:?}", size);
        let security_descriptor_ptr = ptr::null_mut();
        let res =
            libregf_key_get_security_descriptor(key.inner, security_descriptor_ptr, size, &mut err);
        if res == -1 {
            println!(
                "Bad name {:?} / {:?} / {:?}",
                size, security_descriptor_ptr, err
            );
            *error = RegfError::from_ptr(err);
        } else if res == 1 {
            if let Ok(s) = std::str::from_utf8(std::slice::from_raw_parts(
                security_descriptor_ptr,
                size as usize,
            )) {
                *security_descriptor = Some(Some(s.to_string()));
            }
        } else if res == 0 {
            *security_descriptor = Some(None);
        }
    }
}
