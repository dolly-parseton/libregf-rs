use crate::value::RegfValue;
use std::mem;
#[macro_export]
macro_rules! read_string {
    ( $fn_name:ident, $size_fn:ident, $data_fn:ident ) => {
        pub unsafe fn $fn_name(
            value: &RegfValue,
            string: &mut Option<String>,
            error: &mut Option<RegfError>,
        ) {
            let mut err: *mut libregf_error_t = ptr::null_mut();
            let mut size = 0;
            let mut string_ptr: [u8; 265 * 4] = mem::zeroed();
            match $size_fn(value.inner, &mut size, &mut err) {
                -1 => *error = RegfError::from_ptr(err),
                _ => match size == 0 {
                    false => match $data_fn(
                        value.inner,
                        string_ptr.as_mut_ptr() as *mut u8,
                        size,
                        &mut err,
                    ) {
                        -1 => *error = RegfError::from_ptr(err),
                        _ => {
                            *string = std::str::from_utf8(&string_ptr[..size as usize])
                                .map(|s| s.to_string())
                                .ok()
                        }
                    },
                    true => *string = Some("".to_string()),
                },
            }
        }
    };
}
