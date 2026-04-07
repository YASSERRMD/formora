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
        crate::css::bootstrap::bootstrap()
    }

    pub fn tailwind() -> Self {
        crate::css::tailwind::tailwind()
    }

    pub fn custom() -> Self {
        crate::css::custom::custom()
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
