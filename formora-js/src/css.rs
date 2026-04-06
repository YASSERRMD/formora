use js_sys::{Object, Reflect};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

use formora_core::CssProfile;

/// CSS framework selector for form styling
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct CssFramework {
    value: String,
}

#[wasm_bindgen]
impl CssFramework {
    #[wasm_bindgen(constructor)]
    pub fn new(value: String) -> CssFramework {
        CssFramework { value }
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> String {
        self.value.clone()
    }

    /// Bootstrap 5 framework preset
    pub fn bootstrap() -> CssFramework {
        CssFramework { value: "bootstrap".to_string() }
    }

    /// Tailwind CSS v3 framework preset
    pub fn tailwind() -> CssFramework {
        CssFramework { value: "tailwind".to_string() }
    }

    /// Minimal custom styles preset
    pub fn custom() -> CssFramework {
        CssFramework { value: "custom".to_string() }
    }
}

/// Resolve a framework name string to a CssProfile
pub fn resolve_profile(fw: &str) -> CssProfile {
    match fw {
        "tailwind" => CssProfile::tailwind(),
        "custom" => CssProfile::custom(),
        _ => CssProfile::bootstrap(),
    }
}

/// CSS class profile — 43 customizable class names
#[wasm_bindgen]
#[derive(Clone)]
pub struct JsCssProfile {
    pub(crate) profile: CssProfile,
}

#[wasm_bindgen]
impl JsCssProfile {
    /// Create a profile from a CssFramework preset (defaults to Bootstrap)
    #[wasm_bindgen(constructor)]
    pub fn new(framework: Option<CssFramework>) -> JsCssProfile {
        let fw = framework.unwrap_or_else(CssFramework::bootstrap);
        JsCssProfile { profile: resolve_profile(&fw.value) }
    }

    /// Create a profile from a plain JS object mapping CSS key names to class strings
    #[wasm_bindgen(js_name = fromObject)]
    pub fn from_object(obj: &Object) -> JsCssProfile {
        let keys = Object::keys(obj);
        let mut map = HashMap::new();
        for i in 0..keys.length() {
            if let Some(key) = keys.get(i).as_string() {
                if let Ok(val) = Reflect::get(obj, &JsValue::from_str(&key)) {
                    if let Some(s) = val.as_string() {
                        map.insert(key, s);
                    }
                }
            }
        }
        JsCssProfile { profile: CssProfile::from_hashmap(map) }
    }

    /// Return a new profile with specific class names overridden
    pub fn override_(&self, overrides: &Object) -> JsCssProfile {
        let keys = Object::keys(overrides);
        let mut map = HashMap::new();

        macro_rules! snapshot {
            ($field:ident) => {
                if !self.profile.$field.is_empty() {
                    map.insert(stringify!($field).to_string(), self.profile.$field.clone());
                }
            };
        }

        snapshot!(form_wrapper); snapshot!(form_title); snapshot!(form_description);
        snapshot!(step_wrapper); snapshot!(step_title);
        snapshot!(progress_bar_wrapper); snapshot!(progress_bar_fill); snapshot!(progress_bar_label);
        snapshot!(field_group); snapshot!(field_label); snapshot!(field_required_marker);
        snapshot!(field_help_text); snapshot!(field_error_message);
        snapshot!(input_text); snapshot!(input_email); snapshot!(input_number);
        snapshot!(input_textarea); snapshot!(input_select); snapshot!(input_date);
        snapshot!(input_file); snapshot!(input_range); snapshot!(input_range_value_display);
        snapshot!(input_checkbox_wrapper); snapshot!(input_checkbox); snapshot!(input_checkbox_label);
        snapshot!(input_radio_wrapper); snapshot!(input_radio); snapshot!(input_radio_label);
        snapshot!(multi_select_wrapper); snapshot!(multi_select_tag); snapshot!(multi_select_tag_remove);
        snapshot!(multi_select_dropdown); snapshot!(multi_select_option);
        snapshot!(input_error_state); snapshot!(input_valid_state); snapshot!(field_hidden);
        snapshot!(button_submit); snapshot!(button_next); snapshot!(button_back); snapshot!(button_wrapper);
        snapshot!(success_wrapper); snapshot!(success_message);
        snapshot!(error_wrapper); snapshot!(error_message);

        for i in 0..keys.length() {
            if let Some(key) = keys.get(i).as_string() {
                if let Ok(val) = Reflect::get(overrides, &JsValue::from_str(&key)) {
                    if let Some(s) = val.as_string() {
                        map.insert(key, s);
                    }
                }
            }
        }

        JsCssProfile { profile: CssProfile::from_hashmap(map) }
    }
}
