use std::ffi::CStr;
use crate::wiredtiger::api::*;
use std::os::raw::c_int;
use std::str;



pub unsafe fn get_error(code: c_int) -> String {
    let slice = CStr::from_ptr(wiredtiger_strerror(code));
    return str::from_utf8(slice.to_bytes()).unwrap().to_string();
}
