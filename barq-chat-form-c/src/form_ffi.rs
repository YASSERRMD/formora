use std::os::raw::{c_char, c_int};

use barq_chat_form_core::{
    CssProfile, Condition, FieldSchema, FieldType, FormSchema, SelectOption, ValidationRule,
};

use crate::css_ffi::{CssFrameworkHandle, CssProfileHandle, resolve_profile};
use crate::helpers::{c_str_to_option, c_str_to_string, parse_json_array, parse_json_value, string_to_c};

// ─── Opaque handle ─────────────────────────────────────────────────────────────

pub struct FormHandle {
    pub schema: FormSchema,
}

// ─── Lifecycle ─────────────────────────────────────────────────────────────────

/// Create a new Form. Pass NULL for `id` to auto-generate a UUID.
#[no_mangle]
pub extern "C" fn barq_form_new(id: *const c_char) -> *mut FormHandle {
    let form_id = c_str_to_option(id)
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    Box::into_raw(Box::new(FormHandle { schema: FormSchema::new(form_id) }))
}

/// Free a FormHandle
#[no_mangle]
pub extern "C" fn barq_form_free(form: *mut FormHandle) {
    if !form.is_null() {
        unsafe { drop(Box::from_raw(form)) };
    }
}

// ─── Builder methods (each returns the same handle for chaining) ───────────────

/// Set the form title
#[no_mangle]
pub extern "C" fn barq_form_title(form: *mut FormHandle, text: *const c_char) -> *mut FormHandle {
    if form.is_null() { return form; }
    unsafe { (*form).schema.title = c_str_to_option(text); }
    form
}

/// Set the form description
#[no_mangle]
pub extern "C" fn barq_form_description(form: *mut FormHandle, text: *const c_char) -> *mut FormHandle {
    if form.is_null() { return form; }
    unsafe { (*form).schema.description = c_str_to_option(text); }
    form
}

/// Set CSS via a framework handle (pass NULL for Bootstrap default)
#[no_mangle]
pub extern "C" fn barq_form_css_framework(
    form: *mut FormHandle,
    fw: *const CssFrameworkHandle,
) -> *mut FormHandle {
    if form.is_null() { return form; }
    unsafe { (*form).schema.css_profile = resolve_profile(fw); }
    form
}

/// Set CSS via a profile handle
#[no_mangle]
pub extern "C" fn barq_form_css_profile(
    form: *mut FormHandle,
    profile: *const CssProfileHandle,
) -> *mut FormHandle {
    if form.is_null() || profile.is_null() { return form; }
    unsafe { (*form).schema.css_profile = (*profile).profile.clone(); }
    form
}

/// Add a step (enables multi-step mode). Pass NULL for title to omit it.
#[no_mangle]
pub extern "C" fn barq_form_step(form: *mut FormHandle, title: *const c_char) -> *mut FormHandle {
    if form.is_null() { return form; }
    unsafe {
        (*form).schema.multi_step = true;
        let idx = (*form).schema.steps.len();
        (*form).schema.steps.push(barq_chat_form_core::schema::StepMeta {
            index: idx,
            title: c_str_to_option(title),
            field_ids: vec![],
        });
    }
    form
}

/// Set the submit button label
#[no_mangle]
pub extern "C" fn barq_form_submit_label(form: *mut FormHandle, text: *const c_char) -> *mut FormHandle {
    if form.is_null() { return form; }
    let label = c_str_to_string(text);
    if !label.is_empty() {
        unsafe { (*form).schema.submit_label = label; }
    }
    form
}

/// Set the success message shown after submission
#[no_mangle]
pub extern "C" fn barq_form_success_message(form: *mut FormHandle, text: *const c_char) -> *mut FormHandle {
    if form.is_null() { return form; }
    let msg = c_str_to_string(text);
    if !msg.is_empty() {
        unsafe { (*form).schema.success_message = msg; }
    }
    form
}

/// Render the form to an HTML string. Caller must free with barq_free_string().
#[no_mangle]
pub extern "C" fn barq_form_build(form: *const FormHandle) -> *mut c_char {
    if form.is_null() { return string_to_c(String::new()); }
    let html = barq_chat_form_core::renderer::render(unsafe { &(*form).schema });
    string_to_c(html)
}

/// Return the form schema as a JSON string. Caller must free with barq_free_string().
#[no_mangle]
pub extern "C" fn barq_form_schema_json(form: *const FormHandle) -> *mut c_char {
    if form.is_null() { return string_to_c("{}".to_string()); }
    let json = serde_json::to_string(unsafe { &(*form).schema }).unwrap_or_else(|_| "{}".to_string());
    string_to_c(json)
}

// ─── Field helpers ─────────────────────────────────────────────────────────────

fn parse_rules(rules_json: *const c_char) -> Vec<ValidationRule> {
    parse_json_array(rules_json)
        .into_iter()
        .filter_map(|v| serde_json::from_value::<ValidationRule>(v).ok())
        .collect()
}

fn parse_show_if(show_if_json: *const c_char) -> Option<Condition> {
    let v = parse_json_value(show_if_json);
    if v.is_null() { None } else { serde_json::from_value::<Condition>(v).ok() }
}

fn parse_options(options_json: *const c_char) -> Vec<SelectOption> {
    parse_json_array(options_json)
        .into_iter()
        .filter_map(|item| {
            let arr = item.as_array()?;
            Some(SelectOption {
                label: arr.get(0)?.as_str()?.to_string(),
                value: arr.get(1)?.as_str()?.to_string(),
            })
        })
        .collect()
}

fn push_field(form: *mut FormHandle, field: FieldSchema) -> *mut FormHandle {
    if form.is_null() { return form; }
    unsafe { (*form).schema.fields.push(field); }
    form
}

// ─── Field methods ─────────────────────────────────────────────────────────────
//
// Common parameters:
//   rules_json   – JSON array of ValidationRule objects, or NULL
//   show_if_json – JSON Condition object, or NULL
//   required     – 1 = required, 0 = optional

/// Add a text input field
#[no_mangle]
pub extern "C" fn barq_form_text(
    form: *mut FormHandle,
    id: *const c_char, label: *const c_char,
    placeholder: *const c_char, required: c_int,
    help_text: *const c_char, default_val: *const c_char,
    rules_json: *const c_char, show_if_json: *const c_char,
) -> *mut FormHandle {
    push_field(form, FieldSchema {
        id: c_str_to_string(id), field_type: FieldType::Text, label: c_str_to_string(label),
        placeholder: c_str_to_option(placeholder),
        default_value: c_str_to_option(default_val).map(|v| serde_json::json!(v)),
        options: None, min: None, max: None, step: None, rows: None, accept: None,
        required: required != 0, help_text: c_str_to_option(help_text),
        rules: parse_rules(rules_json), show_if: parse_show_if(show_if_json), step_index: None,
    })
}

/// Add an email input field
#[no_mangle]
pub extern "C" fn barq_form_email(
    form: *mut FormHandle,
    id: *const c_char, label: *const c_char,
    required: c_int, help_text: *const c_char,
    default_val: *const c_char,
    rules_json: *const c_char, show_if_json: *const c_char,
) -> *mut FormHandle {
    push_field(form, FieldSchema {
        id: c_str_to_string(id), field_type: FieldType::Email, label: c_str_to_string(label),
        placeholder: None,
        default_value: c_str_to_option(default_val).map(|v| serde_json::json!(v)),
        options: None, min: None, max: None, step: None, rows: None, accept: None,
        required: required != 0, help_text: c_str_to_option(help_text),
        rules: parse_rules(rules_json), show_if: parse_show_if(show_if_json), step_index: None,
    })
}

/// Add a number input field. Pass NaN (use barq_nan()) for absent min/max.
#[no_mangle]
pub extern "C" fn barq_form_number(
    form: *mut FormHandle,
    id: *const c_char, label: *const c_char,
    min: f64, max: f64,
    required: c_int, help_text: *const c_char,
    default_val: *const c_char,
    rules_json: *const c_char, show_if_json: *const c_char,
) -> *mut FormHandle {
    push_field(form, FieldSchema {
        id: c_str_to_string(id), field_type: FieldType::Number, label: c_str_to_string(label),
        placeholder: None,
        default_value: c_str_to_option(default_val)
            .and_then(|v| v.parse::<f64>().ok())
            .map(|n| serde_json::json!(n)),
        options: None,
        min: if min.is_nan() { None } else { Some(min) },
        max: if max.is_nan() { None } else { Some(max) },
        step: None, rows: None, accept: None,
        required: required != 0, help_text: c_str_to_option(help_text),
        rules: parse_rules(rules_json), show_if: parse_show_if(show_if_json), step_index: None,
    })
}

/// Add a textarea field. Pass 0 for rows to use default.
#[no_mangle]
pub extern "C" fn barq_form_textarea(
    form: *mut FormHandle,
    id: *const c_char, label: *const c_char,
    rows: u32, placeholder: *const c_char,
    required: c_int, help_text: *const c_char,
    default_val: *const c_char,
    rules_json: *const c_char, show_if_json: *const c_char,
) -> *mut FormHandle {
    push_field(form, FieldSchema {
        id: c_str_to_string(id), field_type: FieldType::Textarea, label: c_str_to_string(label),
        placeholder: c_str_to_option(placeholder),
        default_value: c_str_to_option(default_val).map(|v| serde_json::json!(v)),
        options: None, min: None, max: None, step: None,
        rows: if rows == 0 { None } else { Some(rows) },
        accept: None,
        required: required != 0, help_text: c_str_to_option(help_text),
        rules: parse_rules(rules_json), show_if: parse_show_if(show_if_json), step_index: None,
    })
}

/// Add a select (dropdown) field.
/// options_json: JSON array of [label, value] pairs, e.g. [["Label","val"],...]
#[no_mangle]
pub extern "C" fn barq_form_select(
    form: *mut FormHandle,
    id: *const c_char, label: *const c_char,
    options_json: *const c_char,
    required: c_int, help_text: *const c_char,
    default_val: *const c_char,
    rules_json: *const c_char, show_if_json: *const c_char,
) -> *mut FormHandle {
    push_field(form, FieldSchema {
        id: c_str_to_string(id), field_type: FieldType::Select, label: c_str_to_string(label),
        placeholder: None,
        default_value: c_str_to_option(default_val).map(|v| serde_json::json!(v)),
        options: Some(parse_options(options_json)),
        min: None, max: None, step: None, rows: None, accept: None,
        required: required != 0, help_text: c_str_to_option(help_text),
        rules: parse_rules(rules_json), show_if: parse_show_if(show_if_json), step_index: None,
    })
}

/// Add a multi-select field.
/// options_json: JSON array of [label, value] pairs
/// default_val_json: JSON array of strings, or NULL
#[no_mangle]
pub extern "C" fn barq_form_multi_select(
    form: *mut FormHandle,
    id: *const c_char, label: *const c_char,
    options_json: *const c_char,
    required: c_int, help_text: *const c_char,
    default_val_json: *const c_char,
    rules_json: *const c_char, show_if_json: *const c_char,
) -> *mut FormHandle {
    let default_value = c_str_to_option(default_val_json)
        .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok());
    push_field(form, FieldSchema {
        id: c_str_to_string(id), field_type: FieldType::MultiSelect, label: c_str_to_string(label),
        placeholder: None, default_value,
        options: Some(parse_options(options_json)),
        min: None, max: None, step: None, rows: None, accept: None,
        required: required != 0, help_text: c_str_to_option(help_text),
        rules: parse_rules(rules_json), show_if: parse_show_if(show_if_json), step_index: None,
    })
}

/// Add a checkbox field
#[no_mangle]
pub extern "C" fn barq_form_checkbox(
    form: *mut FormHandle,
    id: *const c_char, label: *const c_char,
    default_val: c_int,
    help_text: *const c_char,
    rules_json: *const c_char, show_if_json: *const c_char,
) -> *mut FormHandle {
    push_field(form, FieldSchema {
        id: c_str_to_string(id), field_type: FieldType::Checkbox, label: c_str_to_string(label),
        placeholder: None,
        default_value: Some(serde_json::json!(default_val != 0)),
        options: None, min: None, max: None, step: None, rows: None, accept: None,
        required: false, help_text: c_str_to_option(help_text),
        rules: parse_rules(rules_json), show_if: parse_show_if(show_if_json), step_index: None,
    })
}

/// Add a radio button group.
/// options_json: JSON array of [label, value] pairs
#[no_mangle]
pub extern "C" fn barq_form_radio(
    form: *mut FormHandle,
    id: *const c_char, label: *const c_char,
    options_json: *const c_char,
    required: c_int, help_text: *const c_char,
    default_val: *const c_char,
    rules_json: *const c_char, show_if_json: *const c_char,
) -> *mut FormHandle {
    push_field(form, FieldSchema {
        id: c_str_to_string(id), field_type: FieldType::Radio, label: c_str_to_string(label),
        placeholder: None,
        default_value: c_str_to_option(default_val).map(|v| serde_json::json!(v)),
        options: Some(parse_options(options_json)),
        min: None, max: None, step: None, rows: None, accept: None,
        required: required != 0, help_text: c_str_to_option(help_text),
        rules: parse_rules(rules_json), show_if: parse_show_if(show_if_json), step_index: None,
    })
}

/// Add a date picker
#[no_mangle]
pub extern "C" fn barq_form_date(
    form: *mut FormHandle,
    id: *const c_char, label: *const c_char,
    required: c_int, help_text: *const c_char,
    default_val: *const c_char,
    rules_json: *const c_char, show_if_json: *const c_char,
) -> *mut FormHandle {
    push_field(form, FieldSchema {
        id: c_str_to_string(id), field_type: FieldType::Date, label: c_str_to_string(label),
        placeholder: None,
        default_value: c_str_to_option(default_val).map(|v| serde_json::json!(v)),
        options: None, min: None, max: None, step: None, rows: None, accept: None,
        required: required != 0, help_text: c_str_to_option(help_text),
        rules: parse_rules(rules_json), show_if: parse_show_if(show_if_json), step_index: None,
    })
}

/// Add a range slider. Pass NaN for step to use default.
#[no_mangle]
pub extern "C" fn barq_form_range(
    form: *mut FormHandle,
    id: *const c_char, label: *const c_char,
    min: f64, max: f64, step: f64,
    default_val: f64,
    help_text: *const c_char, show_if_json: *const c_char,
) -> *mut FormHandle {
    push_field(form, FieldSchema {
        id: c_str_to_string(id), field_type: FieldType::Range, label: c_str_to_string(label),
        placeholder: None,
        default_value: if default_val.is_nan() { None } else { Some(serde_json::json!(default_val)) },
        options: None, min: Some(min), max: Some(max),
        step: if step.is_nan() { None } else { Some(step) },
        rows: None, accept: None, required: false,
        help_text: c_str_to_option(help_text),
        rules: vec![], show_if: parse_show_if(show_if_json), step_index: None,
    })
}

/// Add a file upload field.
/// accept_json: JSON array of MIME types/extensions, or NULL
#[no_mangle]
pub extern "C" fn barq_form_file(
    form: *mut FormHandle,
    id: *const c_char, label: *const c_char,
    accept_json: *const c_char,
    required: c_int, help_text: *const c_char,
    rules_json: *const c_char, show_if_json: *const c_char,
) -> *mut FormHandle {
    let accept: Option<Vec<String>> = c_str_to_option(accept_json)
        .and_then(|s| serde_json::from_str(&s).ok());
    push_field(form, FieldSchema {
        id: c_str_to_string(id), field_type: FieldType::File, label: c_str_to_string(label),
        placeholder: None, default_value: None, options: None,
        min: None, max: None, step: None, rows: None, accept,
        required: required != 0, help_text: c_str_to_option(help_text),
        rules: parse_rules(rules_json), show_if: parse_show_if(show_if_json), step_index: None,
    })
}

/// Add a hidden field (carries context, not shown to user)
#[no_mangle]
pub extern "C" fn barq_form_hidden(
    form: *mut FormHandle,
    id: *const c_char,
    value: *const c_char,
) -> *mut FormHandle {
    push_field(form, FieldSchema {
        id: c_str_to_string(id), field_type: FieldType::Hidden, label: String::new(),
        placeholder: None,
        default_value: Some(serde_json::json!(c_str_to_string(value))),
        options: None, min: None, max: None, step: None, rows: None, accept: None,
        required: false, help_text: None, rules: vec![], show_if: None, step_index: None,
    })
}

// ─── Rule helpers (return JSON strings) ───────────────────────────────────────

/// Returns rule JSON for "required". Pass NULL for message to use default.
#[no_mangle]
pub extern "C" fn barq_rule_required(message: *const c_char) -> *mut c_char {
    let r = serde_json::json!({"rule_type":"required","value":null,"message":c_str_to_option(message)});
    string_to_c(r.to_string())
}

/// Returns rule JSON for "min_length"
#[no_mangle]
pub extern "C" fn barq_rule_min_length(n: u32, message: *const c_char) -> *mut c_char {
    let r = serde_json::json!({"rule_type":"min_length","value":n,"message":c_str_to_option(message)});
    string_to_c(r.to_string())
}

/// Returns rule JSON for "max_length"
#[no_mangle]
pub extern "C" fn barq_rule_max_length(n: u32, message: *const c_char) -> *mut c_char {
    let r = serde_json::json!({"rule_type":"max_length","value":n,"message":c_str_to_option(message)});
    string_to_c(r.to_string())
}

/// Returns rule JSON for "min"
#[no_mangle]
pub extern "C" fn barq_rule_min(n: f64, message: *const c_char) -> *mut c_char {
    let r = serde_json::json!({"rule_type":"min","value":n,"message":c_str_to_option(message)});
    string_to_c(r.to_string())
}

/// Returns rule JSON for "max"
#[no_mangle]
pub extern "C" fn barq_rule_max(n: f64, message: *const c_char) -> *mut c_char {
    let r = serde_json::json!({"rule_type":"max","value":n,"message":c_str_to_option(message)});
    string_to_c(r.to_string())
}

/// Returns rule JSON for "regex"
#[no_mangle]
pub extern "C" fn barq_rule_regex(pattern: *const c_char, message: *const c_char) -> *mut c_char {
    let r = serde_json::json!({"rule_type":"regex","value":c_str_to_string(pattern),"message":c_str_to_option(message)});
    string_to_c(r.to_string())
}

/// Returns rule JSON for "email"
#[no_mangle]
pub extern "C" fn barq_rule_email(message: *const c_char) -> *mut c_char {
    let r = serde_json::json!({"rule_type":"email","value":null,"message":c_str_to_option(message)});
    string_to_c(r.to_string())
}

// ─── Condition helper ──────────────────────────────────────────────────────────

/// Returns condition JSON: {"field_id":..., "operator":..., "value":...}
/// value_json: any valid JSON value (string, number, bool, array)
#[no_mangle]
pub extern "C" fn barq_condition(
    field_id: *const c_char,
    operator: *const c_char,
    value_json: *const c_char,
) -> *mut c_char {
    let c = serde_json::json!({
        "field_id": c_str_to_string(field_id),
        "operator": c_str_to_string(operator),
        "value": parse_json_value(value_json),
    });
    string_to_c(c.to_string())
}
