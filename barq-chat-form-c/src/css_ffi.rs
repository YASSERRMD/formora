use std::os::raw::c_char;

use barq_chat_form_core::CssProfile;

use crate::helpers::{c_str_to_string, c_str_to_option, string_to_c};

// ─── Opaque handles ───────────────────────────────────────────────────────────

pub struct CssFrameworkHandle {
    pub value: String,
}

pub struct CssProfileHandle {
    pub profile: CssProfile,
}

// ─── CssFramework ─────────────────────────────────────────────────────────────

/// Create a Bootstrap 5 CSS framework handle
#[no_mangle]
pub extern "C" fn barq_css_bootstrap() -> *mut CssFrameworkHandle {
    Box::into_raw(Box::new(CssFrameworkHandle { value: "bootstrap".to_string() }))
}

/// Create a Tailwind CSS v3 framework handle
#[no_mangle]
pub extern "C" fn barq_css_tailwind() -> *mut CssFrameworkHandle {
    Box::into_raw(Box::new(CssFrameworkHandle { value: "tailwind".to_string() }))
}

/// Create a minimal custom styles framework handle
#[no_mangle]
pub extern "C" fn barq_css_custom() -> *mut CssFrameworkHandle {
    Box::into_raw(Box::new(CssFrameworkHandle { value: "custom".to_string() }))
}

/// Free a CssFrameworkHandle
#[no_mangle]
pub extern "C" fn barq_css_framework_free(fw: *mut CssFrameworkHandle) {
    if !fw.is_null() {
        unsafe { drop(Box::from_raw(fw)) };
    }
}

// ─── CssProfile ───────────────────────────────────────────────────────────────

/// Create a CssProfile from a framework handle (pass NULL for Bootstrap default)
#[no_mangle]
pub extern "C" fn barq_css_profile_new(fw: *const CssFrameworkHandle) -> *mut CssProfileHandle {
    let profile = if fw.is_null() {
        CssProfile::bootstrap()
    } else {
        match unsafe { &*fw }.value.as_str() {
            "tailwind" => CssProfile::tailwind(),
            "custom"   => CssProfile::custom(),
            _          => CssProfile::bootstrap(),
        }
    };
    Box::into_raw(Box::new(CssProfileHandle { profile }))
}

/// Create a CssProfile from a JSON object mapping CSS class keys to values.
/// Example: {"form_wrapper":"container","button_submit":"btn btn-primary"}
#[no_mangle]
pub extern "C" fn barq_css_profile_from_json(json: *const c_char) -> *mut CssProfileHandle {
    let map: std::collections::HashMap<String, String> = c_str_to_option(json)
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default();
    Box::into_raw(Box::new(CssProfileHandle {
        profile: CssProfile::from_hashmap(map),
    }))
}

/// Return a new profile with class names overridden by a JSON object.
/// The original profile is unchanged; a new handle is returned.
#[no_mangle]
pub extern "C" fn barq_css_profile_override(
    profile: *const CssProfileHandle,
    overrides_json: *const c_char,
) -> *mut CssProfileHandle {
    if profile.is_null() {
        return barq_css_profile_new(std::ptr::null());
    }
    let base = unsafe { &(*profile).profile };
    let overrides_map: std::collections::HashMap<String, String> = c_str_to_option(overrides_json)
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default();
    let override_profile = CssProfile::from_hashmap(overrides_map);
    let merged = CssProfile::merge(base, &override_profile);
    Box::into_raw(Box::new(CssProfileHandle { profile: merged }))
}

/// Free a CssProfileHandle
#[no_mangle]
pub extern "C" fn barq_css_profile_free(profile: *mut CssProfileHandle) {
    if !profile.is_null() {
        unsafe { drop(Box::from_raw(profile)) };
    }
}

// ─── Helper (used by form_ffi.rs) ─────────────────────────────────────────────

pub(crate) fn resolve_profile(fw: *const CssFrameworkHandle) -> CssProfile {
    if fw.is_null() {
        CssProfile::bootstrap()
    } else {
        match unsafe { &*fw }.value.as_str() {
            "tailwind" => CssProfile::tailwind(),
            "custom"   => CssProfile::custom(),
            _          => CssProfile::bootstrap(),
        }
    }
}
