use std::os::raw::c_char;

use barq_chat_form_core::{parse, is_barq_message, FormResult};

use crate::helpers::string_to_c;

// ─── Opaque handle ─────────────────────────────────────────────────────────────

pub struct FormResultHandle {
    pub result: FormResult,
}

// ─── Parser functions ──────────────────────────────────────────────────────────

/// Parse a `__barq__{...}` message.
/// Returns NULL if the message is not a barq submission.
/// Free the handle with barq_result_free() when done.
#[no_mangle]
pub extern "C" fn barq_parse(message: *const c_char) -> *mut FormResultHandle {
    if message.is_null() { return std::ptr::null_mut(); }
    let msg = unsafe { std::ffi::CStr::from_ptr(message).to_string_lossy().into_owned() };
    match parse(&msg) {
        Some(result) => Box::into_raw(Box::new(FormResultHandle { result })),
        None => std::ptr::null_mut(),
    }
}

/// Returns 1 if the message is a barq submission, 0 otherwise.
#[no_mangle]
pub extern "C" fn barq_is_barq(message: *const c_char) -> i32 {
    if message.is_null() { return 0; }
    let msg = unsafe { std::ffi::CStr::from_ptr(message).to_string_lossy().into_owned() };
    if is_barq_message(&msg) { 1 } else { 0 }
}

// ─── FormResult accessors ──────────────────────────────────────────────────────

/// Free a FormResultHandle
#[no_mangle]
pub extern "C" fn barq_result_free(result: *mut FormResultHandle) {
    if !result.is_null() {
        unsafe { drop(Box::from_raw(result)) };
    }
}

/// Get the form_id from a result. Caller must free with barq_free_string().
#[no_mangle]
pub extern "C" fn barq_result_form_id(result: *const FormResultHandle) -> *mut c_char {
    if result.is_null() { return string_to_c(String::new()); }
    string_to_c(unsafe { (*result).result.form_id.clone() })
}

/// Get raw form data as a JSON string. Caller must free with barq_free_string().
#[no_mangle]
pub extern "C" fn barq_result_data_json(result: *const FormResultHandle) -> *mut c_char {
    if result.is_null() { return string_to_c("{}".to_string()); }
    let json = serde_json::to_string(&unsafe { &(*result).result }.data)
        .unwrap_or_else(|_| "{}".to_string());
    string_to_c(json)
}

/// Get type-coerced form data as a JSON string. Caller must free with barq_free_string().
#[no_mangle]
pub extern "C" fn barq_result_typed_data_json(result: *const FormResultHandle) -> *mut c_char {
    if result.is_null() { return string_to_c("{}".to_string()); }
    let json = serde_json::to_string(&unsafe { &(*result).result }.typed_data)
        .unwrap_or_else(|_| "{}".to_string());
    string_to_c(json)
}

/// Get a human-readable text summary of the result. Caller must free with barq_free_string().
#[no_mangle]
pub extern "C" fn barq_result_as_text(result: *const FormResultHandle) -> *mut c_char {
    if result.is_null() { return string_to_c(String::new()); }
    string_to_c(unsafe { (*result).result.as_text() })
}
