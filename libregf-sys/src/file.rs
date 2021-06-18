use crate::{key::*, *};
use std::{error, ffi, ptr};

#[derive(Debug)]
pub struct RegfFile {
    pub inner: *mut isize,
}

impl Default for RegfFile {
    fn default() -> Self {
        Self {
            inner: ptr::null_mut(),
        }
    }
}

impl RegfFile {
    /// Get a new RegFile handle using a string representing the absolute path to a valid registry file.
    pub fn open(path: &str) -> Result<Self, Box<dyn error::Error>> {
        println!("{}", path);
        let cstring_path = ffi::CString::new(path)?;
        let mut reg_file = Default::default();
        let mut error = None;
        // May fail so this is seperate and not in a match that will cause a crash (?), read up on unsafe a bit more.
        unsafe { unsafe_fn::_file_open(&mut reg_file, &mut error, cstring_path) };
        // If the inner to the error is still null the _file_open function was successful
        // * Could probably measure the reg_file.inner as well?
        match error {
            Some(e) => Err(e.into()),
            None => match reg_file.inner.is_null() {
                true => Err("unsafe function '_file_open' failed, inner is null.".into()),
                false => Ok(reg_file),
            },
        }
    }
    /// Get the root node of a registry file
    pub fn root_node(&self) -> Result<RegfKey, Box<dyn error::Error>> {
        let mut error = None;
        let mut root = RegfKey::default();
        unsafe { unsafe_fn::_file_root(&self, &mut root, &mut error) };
        match error {
            Some(e) => Err(e.into()),
            None => match root.inner.is_null() {
                true => Err("unsafe function '_file_root' failed, inner is null.".into()),
                false => Ok(root),
            },
        }
    }
}

impl Drop for RegfFile {
    fn drop(&mut self) {
        let mut error = None;
        unsafe { unsafe_fn::_file_free(self, &mut error) };
        if let Some(error) = error {
            eprintln!("{}", error);
        }
    }
}

mod unsafe_fn {
    //
    use super::*;
    //
    pub unsafe fn _file_open(
        file: &mut RegfFile,
        error: &mut Option<RegfError>,
        cstring_path: ffi::CString,
    ) {
        let chars: *mut i8 = cstring_path.into_raw();
        let mut err: *mut libregf_error_t = ptr::null_mut();
        match libregf_check_file_signature(chars, &mut err) == 1 {
            true => {
                // match libregf_file_is_corrupted(file.inner, &mut err) {
                //     -1 => *error = RegfError::from_ptr(err),
                //     _ =>
                match libregf_file_initialize(&mut file.inner, &mut err) == 1 {
                    true => {
                        match libregf_file_open(
                            file.inner,
                            chars,
                            libregf_get_access_flags_read(),
                            &mut err,
                        ) == -1
                        {
                            true => *error = RegfError::from_ptr(err),
                            false => (), // File inner is set upon successful run.
                        }
                    }
                    false => *error = RegfError::from_ptr(err),
                }
            }
            // },
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
}
