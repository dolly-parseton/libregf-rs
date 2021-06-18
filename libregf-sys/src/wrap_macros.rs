#[macro_export]
macro_rules! read_string {
    ( $fn_name:ident, $inner_struct:ident, $empty_value:tt, $size_fn:ident, $data_fn:ident ) => {
        pub unsafe fn $fn_name(
            value: &$inner_struct,
            string: &mut Option<String>,
            error: &mut Option<crate::RegfError>,
        ) {
            let mut err: *mut libregf_error_t = std::ptr::null_mut();
            let mut size = 0;
            let mut string_ptr: [u8; 16383 * 4] = std::mem::zeroed();
            match $size_fn(value.inner, &mut size, &mut err) {
                -1 => *error = crate::RegfError::from_ptr(err),
                _ => match size == 0 {
                    false => match $data_fn(
                        value.inner,
                        string_ptr.as_mut_ptr() as *mut u8,
                        size,
                        &mut err,
                    ) {
                        -1 => *error = crate::RegfError::from_ptr(err),
                        _ => {
                            *string = std::str::from_utf8(&string_ptr[..size as usize])
                                .map(|s| s.to_string())
                                .ok()
                        }
                    },
                    true => *string = Some($empty_value.to_string()),
                },
            }
        }
    };
}

#[macro_export]
macro_rules! free_libregf {
    ( $fn_name:ident, $inner_struct:ty, $free_fn:ident ) => {
        pub unsafe fn $fn_name(value: $inner_struct, error: &mut Option<RegfError>) {
            let mut err = std::ptr::null_mut();
            if $free_fn(value, &mut err) == -1 {
                *error = crate::RegfError::from_ptr(err);
            }
        }
    };
}
