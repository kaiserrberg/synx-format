//! SYNX C FFI — exposes synx_parse and synx_free for C/C++/Go/etc.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use synx_core::{parse, to_json};

/// Parse a SYNX string and return a JSON string.
/// Caller must free the result with `synx_free`.
///
/// # Safety
/// `input` must be a valid null-terminated UTF-8 C string.
#[no_mangle]
pub unsafe extern "C" fn synx_parse(input: *const c_char) -> *mut c_char {
    if input.is_null() {
        return std::ptr::null_mut();
    }
    let c_str = unsafe { CStr::from_ptr(input) };
    let text = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    let result = parse(text);
    let json = to_json(&result.root);
    match CString::new(json) {
        Ok(c) => c.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Parse a SYNX string with engine resolution (active mode) and return JSON.
/// Caller must free the result with `synx_free`.
///
/// # Safety
/// `input` must be a valid null-terminated UTF-8 C string.
#[no_mangle]
pub unsafe extern "C" fn synx_parse_active(input: *const c_char) -> *mut c_char {
    if input.is_null() {
        return std::ptr::null_mut();
    }
    let c_str = unsafe { CStr::from_ptr(input) };
    let text = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    let mut result = parse(text);
    if result.mode == synx_core::Mode::Active {
        synx_core::resolve(&mut result, &synx_core::Options::default());
    }
    let json = to_json(&result.root);
    match CString::new(json) {
        Ok(c) => c.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Free a string returned by synx_parse or synx_parse_active.
///
/// # Safety
/// `ptr` must be a pointer returned by `synx_parse` or `synx_parse_active`,
/// and must not have been previously freed.
#[no_mangle]
pub unsafe extern "C" fn synx_free(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe { drop(CString::from_raw(ptr)) };
    }
}
