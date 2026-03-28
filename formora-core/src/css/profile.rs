use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CssProfile {
    pub form_wrapper: String,
    pub form_title: String,
    pub form_description: String,
    pub step_wrapper: String,
    pub step_title: String,
    pub progress_bar_wrapper: String,
    pub progress_bar_fill: String,
    pub progress_bar_label: String,
    pub field_group: String,
    pub field_label: String,
    pub field_required_marker: String,
    pub field_help_text: String,
    pub field_error_message: String,
    pub input_text: String,
    pub input_email: String,
    pub input_number: String,
    pub input_textarea: String,
    pub input_select: String,
    pub input_date: String,
    pub input_file: String,
    pub input_range: String,
    pub input_range_value_display: String,
    pub input_checkbox_wrapper: String,
    pub input_checkbox: String,
    pub input_checkbox_label: String,
    pub input_radio_wrapper: String,
    pub input_radio: String,
    pub input_radio_label: String,
    pub multi_select_wrapper: String,
    pub multi_select_tag: String,
    pub multi_select_tag_remove: String,
    pub multi_select_dropdown: String,
    pub multi_select_option: String,
    pub input_error_state: String,
    pub input_valid_state: String,
    pub field_hidden: String,
    pub button_submit: String,
    pub button_next: String,
    pub button_back: String,
    pub button_wrapper: String,
    pub success_wrapper: String,
    pub success_message: String,
    pub error_wrapper: String,
    pub error_message: String,
}

impl CssProfile {
    pub fn bootstrap() -> Self {
        CssProfile {
            form_wrapper: String::new(),
            form_title: "h5 mb-3 fw-semibold".to_string(),
            form_description: "text-muted mb-3 small".to_string(),
            step_wrapper: String::new(),
            step_title: "h6 mb-2".to_string(),
            progress_bar_wrapper: "progress mb-4".to_string(),
            progress_bar_fill: "progress-bar".to_string(),
            progress_bar_label: "small text-muted mb-1".to_string(),
            field_group: "mb-3".to_string(),
            field_label: "form-label".to_string(),
            field_required_marker: "text-danger ms-1".to_string(),
            field_help_text: "form-text text-muted".to_string(),
            field_error_message: "invalid-feedback d-block".to_string(),
            input_text: "form-control".to_string(),
            input_email: "form-control".to_string(),
            input_number: "form-control".to_string(),
            input_textarea: "form-control".to_string(),
            input_select: "form-select".to_string(),
            input_date: "form-control".to_string(),
            input_file: "form-control".to_string(),
            input_range: "form-range".to_string(),
            input_range_value_display: "form-text text-muted".to_string(),
            input_checkbox_wrapper: "form-check".to_string(),
            input_checkbox: "form-check-input".to_string(),
            input_checkbox_label: "form-check-label".to_string(),
            input_radio_wrapper: "form-check".to_string(),
            input_radio: "form-check-input".to_string(),
            input_radio_label: "form-check-label".to_string(),
            multi_select_wrapper: "border rounded p-2 d-flex flex-wrap gap-1 min-height-38".to_string(),
            multi_select_tag: "badge bg-primary d-flex align-items-center gap-1".to_string(),
            multi_select_tag_remove: "btn-close btn-close-white ms-1".to_string(),
            multi_select_dropdown: "list-group position-absolute w-100 z-index-100".to_string(),
            multi_select_option: "list-group-item list-group-item-action py-1 px-2 small".to_string(),
            input_error_state: "is-invalid".to_string(),
            input_valid_state: "is-valid".to_string(),
            field_hidden: "d-none".to_string(),
            button_submit: "btn btn-primary".to_string(),
            button_next: "btn btn-primary".to_string(),
            button_back: "btn btn-secondary me-2".to_string(),
            button_wrapper: "d-flex justify-content-end mt-3 gap-2".to_string(),
            success_wrapper: "alert alert-success mt-3".to_string(),
            success_message: String::new(),
            error_wrapper: "alert alert-danger mt-3".to_string(),
            error_message: String::new(),
        }
    }

    pub fn tailwind() -> Self {
        CssProfile {
            form_wrapper: "space-y-4".to_string(),
            form_title: "text-lg font-semibold text-gray-800 dark:text-gray-100".to_string(),
            form_description: "text-sm text-gray-500 dark:text-gray-400".to_string(),
            step_wrapper: "space-y-4".to_string(),
            step_title: "text-base font-medium text-gray-700 dark:text-gray-200".to_string(),
            progress_bar_wrapper: "w-full bg-gray-200 rounded-full h-1.5 mb-4 dark:bg-gray-700".to_string(),
            progress_bar_fill: "bg-blue-600 h-1.5 rounded-full transition-all duration-300".to_string(),
            progress_bar_label: "text-xs text-gray-500 mb-1 dark:text-gray-400".to_string(),
            field_group: "flex flex-col gap-1".to_string(),
            field_label: "text-sm font-medium text-gray-700 dark:text-gray-300".to_string(),
            field_required_marker: "text-red-500 ml-0.5".to_string(),
            field_help_text: "text-xs text-gray-400 dark:text-gray-500".to_string(),
            field_error_message: "text-xs text-red-500 mt-0.5".to_string(),
            input_text: "w-full border border-gray-300 rounded-md px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-800 dark:text-gray-100".to_string(),
            input_email: "w-full border border-gray-300 rounded-md px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-800 dark:text-gray-100".to_string(),
            input_number: "w-full border border-gray-300 rounded-md px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-800 dark:text-gray-100".to_string(),
            input_textarea: "w-full border border-gray-300 rounded-md px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 resize-y dark:border-gray-600 dark:bg-gray-800 dark:text-gray-100".to_string(),
            input_select: "w-full border border-gray-300 rounded-md px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white dark:border-gray-600 dark:bg-gray-800 dark:text-gray-100".to_string(),
            input_date: "w-full border border-gray-300 rounded-md px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-800 dark:text-gray-100".to_string(),
            input_file: "w-full text-sm text-gray-500 file:mr-3 file:py-2 file:px-4 file:rounded file:border-0 file:text-sm file:font-medium file:bg-blue-50 file:text-blue-700 hover:file:bg-blue-100".to_string(),
            input_range: "w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer accent-blue-500".to_string(),
            input_range_value_display: "text-xs text-gray-500 text-right".to_string(),
            input_checkbox_wrapper: "flex items-center gap-2".to_string(),
            input_checkbox: "h-4 w-4 text-blue-600 border-gray-300 rounded focus:ring-blue-500".to_string(),
            input_checkbox_label: "text-sm text-gray-700 dark:text-gray-300".to_string(),
            input_radio_wrapper: "flex items-center gap-2".to_string(),
            input_radio: "h-4 w-4 text-blue-600 border-gray-300 focus:ring-blue-500".to_string(),
            input_radio_label: "text-sm text-gray-700 dark:text-gray-300".to_string(),
            multi_select_wrapper: "border border-gray-300 rounded-md p-2 flex flex-wrap gap-1 min-h-[40px] cursor-text dark:border-gray-600 dark:bg-gray-800".to_string(),
            multi_select_tag: "bg-blue-100 text-blue-800 text-xs px-2 py-0.5 rounded-full flex items-center gap-1 dark:bg-blue-900 dark:text-blue-200".to_string(),
            multi_select_tag_remove: "text-blue-500 hover:text-blue-700 font-bold leading-none dark:text-blue-400".to_string(),
            multi_select_dropdown: "absolute w-full bg-white border border-gray-200 rounded-md shadow-lg z-50 mt-1 dark:bg-gray-800 dark:border-gray-600".to_string(),
            multi_select_option: "px-3 py-2 text-sm hover:bg-blue-50 cursor-pointer dark:hover:bg-gray-700 dark:text-gray-200".to_string(),
            input_error_state: "border-red-500 focus:ring-red-500".to_string(),
            input_valid_state: "border-green-500 focus:ring-green-500".to_string(),
            field_hidden: "hidden".to_string(),
            button_submit: "bg-blue-600 text-white px-4 py-2 rounded-md text-sm font-medium hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500".to_string(),
            button_next: "bg-blue-600 text-white px-4 py-2 rounded-md text-sm font-medium hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500".to_string(),
            button_back: "bg-gray-100 text-gray-700 px-4 py-2 rounded-md text-sm font-medium hover:bg-gray-200 transition-colors mr-2 dark:bg-gray-700 dark:text-gray-200".to_string(),
            button_wrapper: "flex justify-end mt-4 gap-2".to_string(),
            success_wrapper: "bg-green-50 border border-green-200 text-green-800 rounded-md p-3 mt-3 text-sm dark:bg-green-900 dark:border-green-700 dark:text-green-200".to_string(),
            success_message: String::new(),
            error_wrapper: "bg-red-50 border border-red-200 text-red-800 rounded-md p-3 mt-3 text-sm dark:bg-red-900 dark:border-red-700 dark:text-red-200".to_string(),
            error_message: String::new(),
        }
    }

    pub fn custom() -> Self {
        CssProfile {
            form_wrapper: String::new(),
            form_title: String::new(),
            form_description: String::new(),
            step_wrapper: String::new(),
            step_title: String::new(),
            progress_bar_wrapper: String::new(),
            progress_bar_fill: String::new(),
            progress_bar_label: String::new(),
            field_group: String::new(),
            field_label: String::new(),
            field_required_marker: String::new(),
            field_help_text: String::new(),
            field_error_message: String::new(),
            input_text: String::new(),
            input_email: String::new(),
            input_number: String::new(),
            input_textarea: String::new(),
            input_select: String::new(),
            input_date: String::new(),
            input_file: String::new(),
            input_range: String::new(),
            input_range_value_display: String::new(),
            input_checkbox_wrapper: String::new(),
            input_checkbox: String::new(),
            input_checkbox_label: String::new(),
            input_radio_wrapper: String::new(),
            input_radio: String::new(),
            input_radio_label: String::new(),
            multi_select_wrapper: String::new(),
            multi_select_tag: String::new(),
            multi_select_tag_remove: String::new(),
            multi_select_dropdown: String::new(),
            multi_select_option: String::new(),
            input_error_state: String::new(),
            input_valid_state: String::new(),
            field_hidden: String::new(),
            button_submit: String::new(),
            button_next: String::new(),
            button_back: String::new(),
            button_wrapper: String::new(),
            success_wrapper: String::new(),
            success_message: String::new(),
            error_wrapper: String::new(),
            error_message: String::new(),
        }
    }

    pub fn from_hashmap(map: HashMap<String, String>) -> Self {
        let mut profile = CssProfile::custom();

        macro_rules! set_field {
            ($field:ident) => {
                if let Some(value) = map.get(stringify!($field)) {
                    profile.$field = value.clone();
                }
            };
        }

        set_field!(form_wrapper);
        set_field!(form_title);
        set_field!(form_description);
        set_field!(step_wrapper);
        set_field!(step_title);
        set_field!(progress_bar_wrapper);
        set_field!(progress_bar_fill);
        set_field!(progress_bar_label);
        set_field!(field_group);
        set_field!(field_label);
        set_field!(field_required_marker);
        set_field!(field_help_text);
        set_field!(field_error_message);
        set_field!(input_text);
        set_field!(input_email);
        set_field!(input_number);
        set_field!(input_textarea);
        set_field!(input_select);
        set_field!(input_date);
        set_field!(input_file);
        set_field!(input_range);
        set_field!(input_range_value_display);
        set_field!(input_checkbox_wrapper);
        set_field!(input_checkbox);
        set_field!(input_checkbox_label);
        set_field!(input_radio_wrapper);
        set_field!(input_radio);
        set_field!(input_radio_label);
        set_field!(multi_select_wrapper);
        set_field!(multi_select_tag);
        set_field!(multi_select_tag_remove);
        set_field!(multi_select_dropdown);
        set_field!(multi_select_option);
        set_field!(input_error_state);
        set_field!(input_valid_state);
        set_field!(field_hidden);
        set_field!(button_submit);
        set_field!(button_next);
        set_field!(button_back);
        set_field!(button_wrapper);
        set_field!(success_wrapper);
        set_field!(success_message);
        set_field!(error_wrapper);
        set_field!(error_message);

        profile
    }

    pub fn merge(base: &CssProfile, overrides: &CssProfile) -> CssProfile {
        let mut merged = base.clone();

        if !overrides.form_wrapper.is_empty() {
            merged.form_wrapper = overrides.form_wrapper.clone();
        }
        if !overrides.form_title.is_empty() {
            merged.form_title = overrides.form_title.clone();
        }
        if !overrides.form_description.is_empty() {
            merged.form_description = overrides.form_description.clone();
        }
        if !overrides.step_wrapper.is_empty() {
            merged.step_wrapper = overrides.step_wrapper.clone();
        }
        if !overrides.step_title.is_empty() {
            merged.step_title = overrides.step_title.clone();
        }
        if !overrides.progress_bar_wrapper.is_empty() {
            merged.progress_bar_wrapper = overrides.progress_bar_wrapper.clone();
        }
        if !overrides.progress_bar_fill.is_empty() {
            merged.progress_bar_fill = overrides.progress_bar_fill.clone();
        }
        if !overrides.progress_bar_label.is_empty() {
            merged.progress_bar_label = overrides.progress_bar_label.clone();
        }
        if !overrides.field_group.is_empty() {
            merged.field_group = overrides.field_group.clone();
        }
        if !overrides.field_label.is_empty() {
            merged.field_label = overrides.field_label.clone();
        }
        if !overrides.field_required_marker.is_empty() {
            merged.field_required_marker = overrides.field_required_marker.clone();
        }
        if !overrides.field_help_text.is_empty() {
            merged.field_help_text = overrides.field_help_text.clone();
        }
        if !overrides.field_error_message.is_empty() {
            merged.field_error_message = overrides.field_error_message.clone();
        }
        if !overrides.input_text.is_empty() {
            merged.input_text = overrides.input_text.clone();
        }
        if !overrides.input_email.is_empty() {
            merged.input_email = overrides.input_email.clone();
        }
        if !overrides.input_number.is_empty() {
            merged.input_number = overrides.input_number.clone();
        }
        if !overrides.input_textarea.is_empty() {
            merged.input_textarea = overrides.input_textarea.clone();
        }
        if !overrides.input_select.is_empty() {
            merged.input_select = overrides.input_select.clone();
        }
        if !overrides.input_date.is_empty() {
            merged.input_date = overrides.input_date.clone();
        }
        if !overrides.input_file.is_empty() {
            merged.input_file = overrides.input_file.clone();
        }
        if !overrides.input_range.is_empty() {
            merged.input_range = overrides.input_range.clone();
        }
        if !overrides.input_range_value_display.is_empty() {
            merged.input_range_value_display = overrides.input_range_value_display.clone();
        }
        if !overrides.input_checkbox_wrapper.is_empty() {
            merged.input_checkbox_wrapper = overrides.input_checkbox_wrapper.clone();
        }
        if !overrides.input_checkbox.is_empty() {
            merged.input_checkbox = overrides.input_checkbox.clone();
        }
        if !overrides.input_checkbox_label.is_empty() {
            merged.input_checkbox_label = overrides.input_checkbox_label.clone();
        }
        if !overrides.input_radio_wrapper.is_empty() {
            merged.input_radio_wrapper = overrides.input_radio_wrapper.clone();
        }
        if !overrides.input_radio.is_empty() {
            merged.input_radio = overrides.input_radio.clone();
        }
        if !overrides.input_radio_label.is_empty() {
            merged.input_radio_label = overrides.input_radio_label.clone();
        }
        if !overrides.multi_select_wrapper.is_empty() {
            merged.multi_select_wrapper = overrides.multi_select_wrapper.clone();
        }
        if !overrides.multi_select_tag.is_empty() {
            merged.multi_select_tag = overrides.multi_select_tag.clone();
        }
        if !overrides.multi_select_tag_remove.is_empty() {
            merged.multi_select_tag_remove = overrides.multi_select_tag_remove.clone();
        }
        if !overrides.multi_select_dropdown.is_empty() {
            merged.multi_select_dropdown = overrides.multi_select_dropdown.clone();
        }
        if !overrides.multi_select_option.is_empty() {
            merged.multi_select_option = overrides.multi_select_option.clone();
        }
        if !overrides.input_error_state.is_empty() {
            merged.input_error_state = overrides.input_error_state.clone();
        }
        if !overrides.input_valid_state.is_empty() {
            merged.input_valid_state = overrides.input_valid_state.clone();
        }
        if !overrides.field_hidden.is_empty() {
            merged.field_hidden = overrides.field_hidden.clone();
        }
        if !overrides.button_submit.is_empty() {
            merged.button_submit = overrides.button_submit.clone();
        }
        if !overrides.button_next.is_empty() {
            merged.button_next = overrides.button_next.clone();
        }
        if !overrides.button_back.is_empty() {
            merged.button_back = overrides.button_back.clone();
        }
        if !overrides.button_wrapper.is_empty() {
            merged.button_wrapper = overrides.button_wrapper.clone();
        }
        if !overrides.success_wrapper.is_empty() {
            merged.success_wrapper = overrides.success_wrapper.clone();
        }
        if !overrides.success_message.is_empty() {
            merged.success_message = overrides.success_message.clone();
        }
        if !overrides.error_wrapper.is_empty() {
            merged.error_wrapper = overrides.error_wrapper.clone();
        }
        if !overrides.error_message.is_empty() {
            merged.error_message = overrides.error_message.clone();
        }
        merged
    }
}

impl Default for CssProfile {
    fn default() -> Self {
        Self::bootstrap()
    }
}
