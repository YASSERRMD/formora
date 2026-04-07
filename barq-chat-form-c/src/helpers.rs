use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Convert a raw C string pointer to a Rust String.
/// Returns an empty string if the pointer is null or invalid UTF-8.
pub(crate) fn c_str_to_string(ptr: *const c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }
    unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() }
}

/// Convert a raw C string pointer to an Option<String>.
/// Returns None if the pointer is null.
pub(crate) fn c_str_to_option(ptr: *const c_char) -> Option<String> {
    if ptr.is_null() {
        None
    } else {
        Some(c_str_to_string(ptr))
    }
}

/// Convert a Rust String to a heap-allocated C string.
/// The caller is responsible for freeing it with barq_free_string().
pub(crate) fn string_to_c(s: String) -> *mut c_char {
    CString::new(s).unwrap_or_default().into_raw()
}

/// Parse a nullable JSON C string into a serde_json::Value array.
/// Returns an empty Vec on null or parse error.
pub(crate) fn parse_json_array(ptr: *const c_char) -> Vec<serde_json::Value> {
    c_str_to_option(ptr)
        .and_then(|s| serde_json::from_str::<Vec<serde_json::Value>>(&s).ok())
        .unwrap_or_default()
}

/// Parse a nullable JSON C string into a serde_json::Value.
/// Returns Null on null or parse error.
pub(crate) fn parse_json_value(ptr: *const c_char) -> serde_json::Value {
    c_str_to_option(ptr)
        .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok())
        .unwrap_or(serde_json::Value::Null)
}

/// Free a C string that was allocated by barq-chat-form.
/// Must be called on every string returned by the barq-chat-form C API.
#[no_mangle]
pub extern "C" fn barq_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe { drop(CString::from_raw(s)) };
    }
}
