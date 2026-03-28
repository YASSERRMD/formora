use crate::css::profile::CssProfile;

pub fn custom() -> CssProfile {

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


pub fn minimal_inline_styles() -> String {
    String::from("\n<style>\n.formora-widget { font-family: system-ui, sans-serif; }\n.formora-widget input, .formora-widget select, .formora-widget textarea { display: block; width: 100%; margin-bottom: 10px; padding: 8px; }\n.formora-widget button { padding: 8px 16px; cursor: pointer; }\n</style>\n")
}
