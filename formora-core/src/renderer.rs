// HTML Renderer
// Generates the complete HTML widget for a form schema
// Implementation will be completed in Phase 2

use crate::schema::FormSchema;
use crate::validator::serialize_rules;
use crate::conditions::serialize_condition;

/// Render a form schema to HTML
pub fn render(schema: &FormSchema) -> String {
    let mut html = String::new();

    // Outer wrapper
    let wrapper_class = &schema.css_profile.form_wrapper;
    if !wrapper_class.is_empty() {
        html.push_str(&format!(r#"<div class="{}" data-formora-id="{}">"#, wrapper_class, schema.form_id));
    } else {
        html.push_str(&format!(r#"<div data-formora-id="{}">"#, schema.form_id));
    }

    // CSS framework comment
    match schema.css_framework.as_str() {
        "bootstrap" => html.push_str(r#"<!-- formora:bootstrap -->"#),
        "tailwind" => html.push_str(r#"<!-- formora:tailwind -->"#),
        "custom" => {
            // Inject minimal styles for custom mode
            html.push_str(&crate::css::custom::minimal_inline_styles());
        }
        _ => {}
    }

    // Title
    if let Some(title) = &schema.title {
        let title_class = &schema.css_profile.form_title;
        if !title_class.is_empty() {
            html.push_str(&format!(r#"<div class="{}">{}</div>"#, title_class, escape_html(title)));
        } else {
            html.push_str(&format!(r#"<div>{}</div>"#, escape_html(title)));
        }
    }

    // Description
    if let Some(description) = &schema.description {
        let desc_class = &schema.css_profile.form_description;
        if !desc_class.is_empty() {
            html.push_str(&format!(r#"<div class="{}">{}</div>"#, desc_class, escape_html(description)));
        } else {
            html.push_str(&format!(r#"<div>{}</div>"#, escape_html(description)));
        }
    }

    // Progress bar for multi-step forms
    if schema.multi_step && !schema.steps.is_empty() {
        let progress_label_class = &schema.css_profile.progress_bar_label;
        let progress_wrapper_class = &schema.css_profile.progress_bar_wrapper;
        let progress_fill_class = &schema.css_profile.progress_bar_fill;

        if !progress_label_class.is_empty() {
            html.push_str(&format!(r#"<div class="{}">Step 1 of {}</div>"#, progress_label_class, schema.steps.len()));
        } else {
            html.push_str(&format!(r#"<div>Step 1 of {}</div>"#, schema.steps.len()));
        }

        if !progress_wrapper_class.is_empty() {
            html.push_str(&format!(
                r#"<div class="{}" style="height:6px">"#,
                progress_wrapper_class
            ));
        } else {
            html.push_str(r#"<div style="height:6px">"#);
        }

        if !progress_fill_class.is_empty() {
            html.push_str(&format!(
                r#"<div class="{}" id="{}-progress" style="width:0%"></div>"#,
                progress_fill_class, schema.form_id
            ));
        } else {
            html.push_str(&format!(
                r#"<div id="{}-progress" style="width:0%"></div>"#,
                schema.form_id
            ));
        }

        html.push_str("</div>");
    }

    // Form element
    html.push_str(r#"<form>"#);

    if schema.multi_step && !schema.steps.is_empty() {
        let step_wrapper_class = &schema.css_profile.step_wrapper;
        let step_title_class = &schema.css_profile.step_title;
        let button_wrapper_class = &schema.css_profile.button_wrapper;
        let button_back_class = &schema.css_profile.button_back;
        let button_next_class = &schema.css_profile.button_next;
        let button_submit_class = &schema.css_profile.button_submit;
        
        for (i, step) in schema.steps.iter().enumerate() {
            let display_style = if i == 0 { "" } else { r#" style="display:none""# };
            
            if !step_wrapper_class.is_empty() {
                html.push_str(&format!(
                    r#"<div class="{}" data-step="{}"{}>"#,
                    step_wrapper_class, i, display_style
                ));
            } else {
                html.push_str(&format!(
                    r#"<div data-step="{}"{}>"#,
                    i, display_style
                ));
            }
            
            if let Some(title) = &step.title {
                if !step_title_class.is_empty() {
                    html.push_str(&format!(r#"<div class="{}">{}</div>"#, step_title_class, escape_html(title)));
                } else {
                    html.push_str(&format!(r#"<div>{}</div>"#, escape_html(title)));
                }
            }
            
            for field in schema.fields.iter().filter(|f| f.step_index == Some(i)) {
                render_field(&mut html, schema, field);
            }
            
            if !button_wrapper_class.is_empty() {
                html.push_str(&format!(r#"<div class="{}">"#, button_wrapper_class));
            } else {
                html.push_str(r#"<div>"#);
            }
            
            if i > 0 {
                if !button_back_class.is_empty() {
                    html.push_str(&format!(r#"<button type="button" class="{}" data-action="prev">Back</button>"#, button_back_class));
                } else {
                    html.push_str(r#"<button type="button" data-action="prev">Back</button>"#);
                }
            }
            
            if i < schema.steps.len() - 1 {
                if !button_next_class.is_empty() {
                    html.push_str(&format!(r#"<button type="button" class="{}" data-action="next">Next</button>"#, button_next_class));
                } else {
                    html.push_str(r#"<button type="button" data-action="next">Next</button>"#);
                }
            } else {
                if !button_submit_class.is_empty() {
                    html.push_str(&format!(r#"<button type="submit" class="{}">{}</button>"#, button_submit_class, escape_html(&schema.submit_label)));
                } else {
                    html.push_str(&format!(r#"<button type="submit">{}</button>"#, escape_html(&schema.submit_label)));
                }
            }
            
            html.push_str("</div></div>");
        }
    } else {
        // Render fields sequentially
        for field in &schema.fields {
            render_field(&mut html, schema, field);
        }

        // Submit button
        let button_wrapper_class = &schema.css_profile.button_wrapper;
        let button_class = &schema.css_profile.button_submit;

        if !button_wrapper_class.is_empty() {
            html.push_str(&format!(r#"<div class="{}">"#, button_wrapper_class));
        } else {
            html.push_str(r#"<div>"#);
        }

        if !button_class.is_empty() {
            html.push_str(&format!(
                r#"<button type="submit" class="{}">{}</button>"#,
                button_class,
                escape_html(&schema.submit_label)
            ));
        } else {
            html.push_str(&format!(
                r#"<button type="submit">{}</button>"#,
                escape_html(&schema.submit_label)
            ));
        }

        html.push_str("</div>");
    }
    html.push_str("</form>");

    // Success and error messages
    let success_wrapper_class = &schema.css_profile.success_wrapper;
    let success_message_class = &schema.css_profile.success_message;
    let error_wrapper_class = &schema.css_profile.error_wrapper;
    let error_message_class = &schema.css_profile.error_message;

    if !success_wrapper_class.is_empty() {
        html.push_str(&format!(
            r#"<div class="{}" id="{}-success" style="display:none">"#,
            success_wrapper_class, schema.form_id
        ));
    } else {
        html.push_str(&format!(
            r#"<div id="{}-success" style="display:none">"#,
            schema.form_id
        ));
    }

    if !success_message_class.is_empty() {
        html.push_str(&format!(
            r#"<div class="{}">{}</div>"#,
            success_message_class,
            escape_html(&schema.success_message)
        ));
    } else {
        html.push_str(&format!(r#"<div>{}</div>"#, escape_html(&schema.success_message)));
    }

    html.push_str("</div>");

    if !error_wrapper_class.is_empty() {
        html.push_str(&format!(
            r#"<div class="{}" id="{}-error-box" style="display:none">"#,
            error_wrapper_class, schema.form_id
        ));
    } else {
        html.push_str(&format!(
            r#"<div id="{}-error-box" style="display:none">"#,
            schema.form_id
        ));
    }

    if !error_message_class.is_empty() {
        html.push_str(&format!(r#"<div class="{}"></div>"#, error_message_class));
    } else {
        html.push_str(r#"<div></div>"#);
    }

    html.push_str("</div>");
    html.push_str("</div>");

    // Inline JavaScript
    html.push_str(&render_javascript(schema));

    html
}

/// Render a single field
fn render_field(html: &mut String, schema: &FormSchema, field: &crate::schema::FieldSchema) {
    use crate::schema::FieldType;

    let field_group_class = &schema.css_profile.field_group;
    let field_label_class = &schema.css_profile.field_label;
    let field_required_marker_class = &schema.css_profile.field_required_marker;
    let field_help_text_class = &schema.css_profile.field_help_text;
    let field_error_message_class = &schema.css_profile.field_error_message;
    let field_hidden_class = &schema.css_profile.field_hidden;

    // Check initial visibility
    let is_visible = if let Some(_condition) = &field.show_if {
        // For now, default to visible if there's a condition
        // In a full implementation, we'd evaluate against default values
        true
    } else {
        true
    };

    let display_style = if is_visible { "" } else { "display:none" };
    let hidden_class_attr = if !is_visible && !field_hidden_class.is_empty() {
        format!(r#" {}"#, field_hidden_class)
    } else {
        String::new()
    };

    let data_condition_attr = if let Some(condition) = &field.show_if {
        format!(r#" data-condition='{}'"#, serialize_condition(condition))
    } else {
        String::new()
    };

    if !field_group_class.is_empty() {
        html.push_str(&format!(
            r#"<div class="{}" data-field-id="{}"{} style="{}{}">"#,
            field_group_class, field.id, data_condition_attr, hidden_class_attr, display_style
        ));
    } else {
        html.push_str(&format!(
            r#"<div data-field-id="{}"{} style="{}{}">"#,
            field.id, data_condition_attr, hidden_class_attr, display_style
        ));
    }

    // Label (except for hidden fields and checkbox)
    if field.field_type != FieldType::Hidden && field.field_type != FieldType::Checkbox {
        if !field_label_class.is_empty() {
            html.push_str(&format!(
                r#"<label class="{}" for="{}">{}"#,
                field_label_class, field.id, escape_html(&field.label)
            ));
        } else {
            html.push_str(&format!(
                r#"<label for="{}">{}"#,
                field.id, escape_html(&field.label)
            ));
        }

        if field.required {
            if !field_required_marker_class.is_empty() {
                html.push_str(&format!(r#"<span class="{}">*</span>"#, field_required_marker_class));
            } else {
                html.push_str(r#"<span>*</span>"#);
            }
        }

        html.push_str("</label>");
    }

    // Input element
    let data_rules = serialize_rules(&field.rules);
    render_input_element(html, schema, field, &data_rules);

    // Help text
    if let Some(help_text) = &field.help_text {
        if !field_help_text_class.is_empty() {
            html.push_str(&format!(
                r#"<div class="{}">{}</div>"#,
                field_help_text_class, escape_html(help_text)
            ));
        } else {
            html.push_str(&format!(r#"<div>{}</div>"#, escape_html(help_text)));
        }
    }

    // Error message
    if !field_error_message_class.is_empty() {
        html.push_str(&format!(
            r#"<div class="{}" id="{}-error" style="display:none"></div>"#,
            field_error_message_class, field.id
        ));
    } else {
        html.push_str(&format!(
            r#"<div id="{}-error" style="display:none"></div>"#,
            field.id
        ));
    }

    html.push_str("</div>");
}

/// Render the input element for a field
fn render_input_element(
    html: &mut String,
    schema: &FormSchema,
    field: &crate::schema::FieldSchema,
    data_rules: &str,
) {
    use crate::schema::FieldType;

    let default_value_str = field
        .default_value
        .as_ref()
        .map(|v| v.to_string())
        .unwrap_or_default();

    match &field.field_type {
        FieldType::Text => {
            let input_class = &schema.css_profile.input_text;
            html.push_str(&render_text_input(
                "text",
                &field.id,
                input_class,
                &field.placeholder,
                data_rules,
                &default_value_str,
            ));
        }
        FieldType::Email => {
            let input_class = &schema.css_profile.input_email;
            html.push_str(&render_text_input(
                "email",
                &field.id,
                input_class,
                &field.placeholder,
                data_rules,
                &default_value_str,
            ));
        }
        FieldType::Number => {
            let input_class = &schema.css_profile.input_number;
            let min = field.min.map(|n| n.to_string());
            let max = field.max.map(|n| n.to_string());
            html.push_str(&render_number_input(
                &field.id,
                input_class,
                &min,
                &max,
                data_rules,
                &default_value_str,
            ));
        }
        FieldType::Textarea => {
            let input_class = &schema.css_profile.input_textarea;
            let rows = field.rows.unwrap_or(3);
            html.push_str(&render_textarea(
                &field.id,
                input_class,
                rows,
                &field.placeholder,
                data_rules,
                &default_value_str,
            ));
        }
        FieldType::Select => {
            let input_class = &schema.css_profile.input_select;
            let options = field.options.as_ref().map(|o| o.as_slice()).unwrap_or(&[]);
            html.push_str(&render_select(
                &field.id,
                input_class,
                options,
                data_rules,
                &field.default_value,
            ));
        }
        FieldType::Checkbox => {
            let wrapper_class = &schema.css_profile.input_checkbox_wrapper;
            let input_class = &schema.css_profile.input_checkbox;
            let label_class = &schema.css_profile.input_checkbox_label;
            let checked = field.default_value.as_ref().and_then(|v| v.as_bool()).unwrap_or(false);
            html.push_str(&render_checkbox(
                &field.id,
                wrapper_class,
                input_class,
                label_class,
                &field.label,
                checked,
                data_rules,
            ));
        }
        FieldType::Radio => {
            let wrapper_class = &schema.css_profile.input_radio_wrapper;
            let input_class = &schema.css_profile.input_radio;
            let label_class = &schema.css_profile.input_radio_label;
            let options = field.options.as_ref().map(|o| o.as_slice()).unwrap_or(&[]);
            html.push_str(&render_radio(
                &field.id,
                wrapper_class,
                input_class,
                label_class,
                options,
                &field.default_value,
            ));
        }
        FieldType::Date => {
            let input_class = &schema.css_profile.input_date;
            html.push_str(&render_text_input(
                "date",
                &field.id,
                input_class,
                &field.placeholder,
                data_rules,
                &default_value_str,
            ));
        }
        FieldType::Range => {
            let input_class = &schema.css_profile.input_range;
            let min = field.min.unwrap_or(0.0);
            let max = field.max.unwrap_or(100.0);
            let step = field.step.unwrap_or(1.0);
            let default_val = field.default_value.as_ref().and_then(|v| v.as_f64()).unwrap_or(min);
            html.push_str(&render_range(
                &field.id,
                input_class,
                min,
                max,
                step,
                default_val,
                schema,
            ));
        }
        FieldType::File => {
            let input_class = &schema.css_profile.input_file;
            html.push_str(&render_text_input(
                "file",
                &field.id,
                input_class,
                &field.placeholder,
                data_rules,
                &default_value_str,
            ));
        }
        FieldType::Hidden => {
            html.push_str(&render_hidden(&field.id, &default_value_str));
        }
        FieldType::MultiSelect => {
            // Multi-select is handled as a custom widget
            let wrapper_class = &schema.css_profile.multi_select_wrapper;
            let dropdown_class = &schema.css_profile.multi_select_dropdown;
            let option_class = &schema.css_profile.multi_select_option;

            if !wrapper_class.is_empty() {
                html.push_str(&format!(
                    r#"<div class="{}" id="{}-container" data-rules='{}">"#,
                    wrapper_class, field.id, data_rules
                ));
            } else {
                html.push_str(&format!(
                    r#"<div id="{}-container" data-rules='{}">"#,
                    field.id, data_rules
                ));
            }

            // Add search input
            html.push_str(&format!(
                r#"<input type="text" id="{}-search" placeholder="Type to search..." autocomplete="off">"#,
                field.id
            ));

            html.push_str("</div>");

            // Add hidden input for actual value
            html.push_str(&format!(
                r#"<input type="hidden" id="{}" name="{}" value="[]">"#,
                field.id, field.id
            ));

            // Add dropdown with options
            let options = field.options.as_ref().map(|o| o.as_slice()).unwrap_or(&[]);
            if !dropdown_class.is_empty() {
                html.push_str(&format!(r#"<div class="{}" id="{}-dropdown" style="display:none">"#, dropdown_class, field.id));
            } else {
                html.push_str(&format!(r#"<div id="{}-dropdown" style="display:none">"#, field.id));
            }

            for option in options {
                if !option_class.is_empty() {
                    html.push_str(&format!(
                        r#"<div class="{}" data-value="{}">{}</div>"#,
                        option_class,
                        escape_html(&option.value),
                        escape_html(&option.label)
                    ));
                } else {
                    html.push_str(&format!(
                        r#"<div data-value="{}">{}</div>"#,
                        escape_html(&option.value),
                        escape_html(&option.label)
                    ));
                }
            }

            html.push_str("</div>");
        }
    }
}

/// Helper: Render text input
fn render_text_input(
    input_type: &str,
    id: &str,
    class: &str,
    placeholder: &Option<String>,
    data_rules: &str,
    value: &str,
) -> String {
    let placeholder_attr = placeholder
        .as_ref()
        .map(|p| format!(r#" placeholder="{}""#, escape_html(p)))
        .unwrap_or_default();

    if !class.is_empty() {
        format!(
            r#"<input type="{}" id="{}" name="{}" class="{}"{} data-rules='{}' value="{}">"#,
            input_type,
            id,
            id,
            class,
            placeholder_attr,
            data_rules,
            escape_html(value)
        )
    } else {
        format!(
            r#"<input type="{}" id="{}" name="{}"{} data-rules='{}' value="{}">"#,
            input_type,
            id,
            id,
            placeholder_attr,
            data_rules,
            escape_html(value)
        )
    }
}

/// Helper: Render number input
fn render_number_input(
    id: &str,
    class: &str,
    min: &Option<String>,
    max: &Option<String>,
    data_rules: &str,
    value: &str,
) -> String {
    let min_attr = min
        .as_ref()
        .map(|m| format!(r#" min="{}""#, m))
        .unwrap_or_default();
    let max_attr = max
        .as_ref()
        .map(|m| format!(r#" max="{}""#, m))
        .unwrap_or_default();

    if !class.is_empty() {
        format!(
            r#"<input type="number" id="{}" name="{}" class="{}"{}{} data-rules='{}' value="{}">"#,
            id, id, class, min_attr, max_attr, data_rules, escape_html(value)
        )
    } else {
        format!(
            r#"<input type="number" id="{}" name="{}"{}{} data-rules='{}' value="{}">"#,
            id, id, min_attr, max_attr, data_rules, escape_html(value)
        )
    }
}

/// Helper: Render textarea
fn render_textarea(
    id: &str,
    class: &str,
    rows: u32,
    placeholder: &Option<String>,
    data_rules: &str,
    value: &str,
) -> String {
    let placeholder_attr = placeholder
        .as_ref()
        .map(|p| format!(r#" placeholder="{}""#, escape_html(p)))
        .unwrap_or_default();

    if !class.is_empty() {
        format!(
            r#"<textarea id="{}" name="{}" class="{}" rows="{}"{} data-rules='{}'>{}</textarea>"#,
            id,
            id,
            class,
            rows,
            placeholder_attr,
            data_rules,
            escape_html(value)
        )
    } else {
        format!(
            r#"<textarea id="{}" name="{}" rows="{}"{} data-rules='{}'>{}</textarea>"#,
            id, id, rows, placeholder_attr, data_rules, escape_html(value)
        )
    }
}

/// Helper: Render select
fn render_select(
    id: &str,
    class: &str,
    options: &[crate::schema::SelectOption],
    data_rules: &str,
    default_value: &Option<serde_json::Value>,
) -> String {
    let mut html = String::new();

    if !class.is_empty() {
        html.push_str(&format!(
            r#"<select id="{}" name="{}" class="{}" data-rules='{}'>"#,
            id, id, class, data_rules
        ));
    } else {
        html.push_str(&format!(
            r#"<select id="{}" name="{}" data-rules='{}'>"#,
            id, id, data_rules
        ));
    }

    html.push_str(r#"<option value="">Select...</option>"#);

    for option in options {
        let selected = if let Some(default) = default_value {
            if default.as_str() == Some(&option.value) {
                " selected"
            } else {
                ""
            }
        } else {
            ""
        };
        html.push_str(&format!(
            r#"<option value="{}"{}>{}</option>"#,
            escape_html(&option.value),
            selected,
            escape_html(&option.label)
        ));
    }

    html.push_str("</select>");
    html
}

/// Helper: Render checkbox
fn render_checkbox(
    id: &str,
    wrapper_class: &str,
    input_class: &str,
    label_class: &str,
    label_text: &str,
    checked: bool,
    data_rules: &str,
) -> String {
    let checked_attr = if checked { r#" checked"# } else { "" };

    let mut html = String::new();

    if !wrapper_class.is_empty() {
        html.push_str(&format!(r#"<div class="{}">"#, wrapper_class));
    } else {
        html.push_str(r#"<div>"#);
    }

    if !input_class.is_empty() {
        html.push_str(&format!(
            r#"<input type="checkbox" id="{}" name="{}" class="{}"{} data-rules='{}'>"#,
            id, id, input_class, checked_attr, data_rules
        ));
    } else {
        html.push_str(&format!(
            r#"<input type="checkbox" id="{}" name="{}"{} data-rules='{}'>"#,
            id, id, checked_attr, data_rules
        ));
    }

    if !label_class.is_empty() {
        html.push_str(&format!(
            r#"<label class="{}" for="{}">{}</label>"#,
            label_class, id, escape_html(label_text)
        ));
    } else {
        html.push_str(&format!(r#"<label for="{}">{}</label>"#, id, escape_html(label_text)));
    }

    html.push_str("</div>");
    html
}

/// Helper: Render radio
fn render_radio(
    id: &str,
    wrapper_class: &str,
    input_class: &str,
    label_class: &str,
    options: &[crate::schema::SelectOption],
    default_value: &Option<serde_json::Value>,
) -> String {
    let mut html = String::new();

    for option in options {
        if !wrapper_class.is_empty() {
            html.push_str(&format!(r#"<div class="{}">"#, wrapper_class));
        } else {
            html.push_str(r#"<div>"#);
        }

        let checked = if let Some(default) = default_value {
            if default.as_str() == Some(&option.value) {
                r#" checked"#
            } else {
                ""
            }
        } else {
            ""
        };

        if !input_class.is_empty() {
            html.push_str(&format!(
                r#"<input type="radio" id="{}-{}" name="{}" value="{}" class="{}"{}>"#,
                id, escape_html(&option.value), id, escape_html(&option.value), input_class, checked
            ));
        } else {
            html.push_str(&format!(
                r#"<input type="radio" id="{}-{}" name="{}" value="{}"{}>"#,
                id, escape_html(&option.value), id, escape_html(&option.value), checked
            ));
        }

        if !label_class.is_empty() {
            html.push_str(&format!(
                r#"<label class="{}" for="{}-{}">{}</label>"#,
                label_class,
                id,
                escape_html(&option.value),
                escape_html(&option.label)
            ));
        } else {
            html.push_str(&format!(
                r#"<label for="{}-{}">{}</label>"#,
                id,
                escape_html(&option.value),
                escape_html(&option.label)
            ));
        }

        html.push_str("</div>");
    }

    html
}

/// Helper: Render range
fn render_range(
    id: &str,
    class: &str,
    min: f64,
    max: f64,
    step: f64,
    default: f64,
    schema: &FormSchema,
) -> String {
    let value_display_class = &schema.css_profile.input_range_value_display;

    let mut html = String::new();

    if !class.is_empty() {
        html.push_str(&format!(
            r#"<input type="range" id="{}" name="{}" class="{}" min="{}" max="{}" step="{}" value="{}" oninput="document.getElementById('{}-val').textContent=this.value">"#,
            id, id, class, min, max, step, default, id
        ));
    } else {
        html.push_str(&format!(
            r#"<input type="range" id="{}" name="{}" min="{}" max="{}" step="{}" value="{}" oninput="document.getElementById('{}-val').textContent=this.value">"#,
            id, id, min, max, step, default, id
        ));
    }

    if !value_display_class.is_empty() {
        html.push_str(&format!(
            r#"<div class="{}" id="{}-val">{}</div>"#,
            value_display_class, id, default
        ));
    } else {
        html.push_str(&format!(r#"<div id="{}-val">{}</div>"#, id, default));
    }

    html
}

/// Helper: Render hidden input
fn render_hidden(id: &str, value: &str) -> String {
    format!(r#"<input type="hidden" id="{}" name="{}" value="{}">"#, id, id, escape_html(value))
}

/// Escape HTML entities
fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

/// Render inline JavaScript
fn render_javascript(schema: &FormSchema) -> String {
    let is_multi_step = schema.multi_step;
    let steps_count = schema.steps.len();
    let form_id = &schema.form_id;
    let input_error_class = &schema.css_profile.input_error_state;
    let field_hidden_class = &schema.css_profile.field_hidden;
    let multi_select_tag_class = &schema.css_profile.multi_select_tag;
    let multi_select_tag_remove_class = &schema.css_profile.multi_select_tag_remove;
    let multi_select_option_class = &schema.css_profile.multi_select_option;

    // Build CSS class assignments as JS code
    let hidden_class_js = if field_hidden_class.is_empty() {
        "null".to_string()
    } else {
        format!("'{}'", field_hidden_class)
    };

    let error_class_js = if input_error_class.is_empty() {
        "null".to_string()
    } else {
        format!("'{}'", input_error_class)
    };

    let tag_class_js = if multi_select_tag_class.is_empty() {
        "''".to_string()
    } else {
        format!("'{}'", multi_select_tag_class)
    };

    let tag_remove_class_js = if multi_select_tag_remove_class.is_empty() {
        "''".to_string()
    } else {
        format!("'{}'", multi_select_tag_remove_class)
    };

    let option_class_js = if multi_select_option_class.is_empty() {
        "''".to_string()
    } else {
        format!("'{}'", multi_select_option_class)
    };

    format!(
        r#"<script>
(function(){{
  const formId = "{}";
  const isMultiStep = {};
  const stepsCount = {};
  const hiddenClass = {};
  const errorClass = {};
  const tagClass = {};
  const tagRemoveClass = {};
  const optionClass = {};
  let currentStep = 0;

  // Helper: Get widget element
  function getWidget() {{
    return document.querySelector('[data-formora-id="' + formId + '"]');
  }}

  // Helper: Get form element
  function getForm() {{
    const widget = getWidget();
    return widget ? widget.querySelector('form') : null;
  }}

  // Helper: Evaluate condition
  function evaluateCondition(condition, fieldValue) {{
    if (!condition) return true;
    const op = condition.operator;
    const condVal = condition.value;

    switch (op) {{
      case 'eq':
        return String(fieldValue) === String(condVal);
      case 'neq':
        return String(fieldValue) !== String(condVal);
      case 'contains':
        return String(fieldValue).includes(String(condVal));
      case 'gt':
        return parseFloat(fieldValue) > parseFloat(condVal);
      case 'lt':
        return parseFloat(fieldValue) < parseFloat(condVal);
      case 'in_list':
        return Array.isArray(condVal) && condVal.includes(fieldValue);
      default:
        return true;
    }}
  }}

  // Update field visibility based on conditions
  function updateFieldVisibility() {{
    const widget = getWidget();
    if (!widget) return;

    const fields = widget.querySelectorAll('[data-field-id]');
    fields.forEach(function(fieldGroup) {{
      const conditionAttr = fieldGroup.getAttribute('data-condition');
      if (!conditionAttr) return;

      try {{
        const condition = JSON.parse(conditionAttr);
        const referencedField = widget.querySelector('[name="' + condition.field_id + '"]');
        if (!referencedField) return;

        let fieldValue = referencedField.value;
        if (referencedField.type === 'checkbox') {{
          fieldValue = referencedField.checked;
        }}

        const isVisible = evaluateCondition(condition, fieldValue);

        if (isVisible) {{
          fieldGroup.style.display = '';
          if (hiddenClass) fieldGroup.classList.remove(hiddenClass);
        }} else {{
          fieldGroup.style.display = 'none';
          if (hiddenClass) fieldGroup.classList.add(hiddenClass);
          // Clear value when hidden
          const input = fieldGroup.querySelector('[name]');
          if (input) {{
            if (input.type === 'checkbox') {{
              input.checked = false;
            }} else {{
              input.value = '';
            }}
          }}
        }}
      }} catch (e) {{
        console.error('Error evaluating condition:', e);
      }}
    }});
  }}

  // Validate a single field
  function validateField(field) {{
    const rulesAttr = field.getAttribute('data-rules');
    if (!rulesAttr) return true;

    try {{
      const rules = JSON.parse(rulesAttr);
      const value = field.value;
      const fieldGroup = field.closest('[data-field-id]');
      const errorDiv = fieldGroup ? fieldGroup.querySelector('[id$="-error"]') : null;

      for (const rule of rules) {{
        if (!evaluateRule(rule, value)) {{
          if (errorDiv) {{
            errorDiv.textContent = rule.message || getDefaultMessage(rule);
            errorDiv.style.display = '';
          }}
          if (errorClass) field.classList.add(errorClass);
          return false;
        }}
      }}

      if (errorDiv) errorDiv.style.display = 'none';
      if (errorClass) field.classList.remove(errorClass);
      return true;
    }} catch (e) {{
      console.error('Error validating field:', e);
      return true;
    }}
  }}

  // Evaluate a validation rule
  function evaluateRule(rule, value) {{
    switch (rule.rule_type) {{
      case 'required':
        return value && value !== '' && value !== '[]';
      case 'min_length':
        return value.length >= rule.value;
      case 'max_length':
        return value.length <= rule.value;
      case 'min':
        return parseFloat(value) >= parseFloat(rule.value);
      case 'max':
        return parseFloat(value) <= parseFloat(rule.value);
      case 'regex':
        return new RegExp(rule.value).test(value);
      case 'email':
        return /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(value);
      default:
        return true;
    }}
  }}

  // Get default error message for rule
  function getDefaultMessage(rule) {{
    switch (rule.rule_type) {{
      case 'required': return 'This field is required';
      case 'min_length': return 'Minimum ' + rule.value + ' characters required';
      case 'max_length': return 'Maximum ' + rule.value + ' characters allowed';
      case 'min': return 'Value must be at least ' + rule.value;
      case 'max': return 'Value must be at most ' + rule.value;
      case 'regex': return 'Invalid format';
      case 'email': return 'Please enter a valid email address';
      default: return 'Invalid value';
    }}
  }}

  // Validate current step
  function validateCurrentStep() {{
    if (!isMultiStep) return true;

    const widget = getWidget();
    const stepDiv = widget.querySelector('[data-step="' + currentStep + '"]');
    if (!stepDiv) return true;

    const fields = stepDiv.querySelectorAll('input:not([type="hidden"]), select, textarea');
    let isValid = true;
    let firstError = null;

    fields.forEach(function(field) {{
      const fieldGroup = field.closest('[data-field-id]');
      if (fieldGroup && fieldGroup.style.display === 'none') return;

      if (!validateField(field)) {{
        isValid = false;
        if (!firstError) firstError = field;
      }}
    }});

    if (!isValid && firstError) {{
      firstError.scrollIntoView({{ behavior: 'smooth', block: 'center' }});
      firstError.focus();
    }}

    return isValid;
  }}

  // Update progress bar
  function updateProgress() {{
    if (!isMultiStep) return;

    const widget = getWidget();
    const progressBar = widget.getElementById(formId + '-progress');
    const progressLabel = widget.querySelector('.progress-bar-label');

    if (progressBar) {{
      const percent = ((currentStep + 1) / stepsCount) * 100;
      progressBar.style.width = percent + '%';
    }}

    if (progressLabel) {{
      progressLabel.textContent = 'Step ' + (currentStep + 1) + ' of ' + stepsCount;
    }}
  }}

  // Show step
  function showStep(stepIndex) {{
    const widget = getWidget();
    const steps = widget.querySelectorAll('[data-step]');

    steps.forEach(function(step, index) {{
      if (index === stepIndex) {{
        step.style.display = '';
      }} else {{
        step.style.display = 'none';
      }}
    }});

    currentStep = stepIndex;
    updateProgress();
  }}

  // Multi-select widget initialization
  function initMultiSelect() {{
    const widget = getWidget();
    const multiSelects = widget.querySelectorAll('[id$="-container"]');

    multiSelects.forEach(function(container) {{
      const fieldId = container.id.replace('-container', '');
      const hiddenInput = document.getElementById(fieldId);
      const dropdown = document.getElementById(fieldId + '-dropdown');
      const searchInput = document.getElementById(fieldId + '-search');

      let selectedValues = [];

      // Render tags
      function renderTags() {{
        // Clear existing tags
        const existingTags = container.querySelectorAll(tagClass || '.formora-multi-select-tag');
        existingTags.forEach(t => t.remove());

        selectedValues.forEach(function(val) {{
          const option = dropdown.querySelector('[data-value="' + val + '"]');
          const label = option ? option.textContent : val;

          const tag = document.createElement('span');
          if (tagClass) tag.className = tagClass;
          tag.innerHTML = label + '<span class="' + tagRemoveClass + '" data-value="' + val + '">&times;</span>';
          container.insertBefore(tag, searchInput);
        }});

        hiddenInput.value = JSON.stringify(selectedValues);
      }}

      // Show dropdown
      searchInput.addEventListener('focus', function() {{
        dropdown.style.display = '';
      }});

      // Filter options
      searchInput.addEventListener('input', function() {{
        const query = this.value.toLowerCase();
        const options = dropdown.querySelectorAll(optionClass || '.formora-multi-select-option');
        options.forEach(function(opt) {{
          const text = opt.textContent.toLowerCase();
          opt.style.display = text.includes(query) ? '' : 'none';
        }});
      }});

      // Select option
      dropdown.addEventListener('click', function(e) {{
        const option = e.target.closest(optionClass || '.formora-multi-select-option');
        if (!option) return;

        const value = option.getAttribute('data-value');
        if (!selectedValues.includes(value)) {{
          selectedValues.push(value);
          renderTags();
        }}
        searchInput.value = '';
      }});

      // Remove tag
      container.addEventListener('click', function(e) {{
        if (e.target.classList.contains(tagRemoveClass.replace('.', '') || 'formora-multi-select-remove')) {{
          const value = e.target.getAttribute('data-value');
          selectedValues = selectedValues.filter(v => v !== value);
          renderTags();
        }}
      }});

      // Hide dropdown on click outside
      document.addEventListener('click', function(e) {{
        if (!container.contains(e.target)) {{
          dropdown.style.display = 'none';
        }}
      }});
    }});
  }}

  // Serialize and send form data
  function submitForm(e) {{
    e.preventDefault();

    const form = getForm();
    const widget = getWidget();
    if (!form || !widget) return;

    // Validate all fields
    const fields = form.querySelectorAll('[name]');
    let isValid = true;
    let firstError = null;

    fields.forEach(function(field) {{
      const fieldGroup = field.closest('[data-field-id]');
      if (fieldGroup && fieldGroup.style.display === 'none') return;

      if (field.type !== 'hidden' && !validateField(field)) {{
        isValid = false;
        if (!firstError) firstError = field;
      }}
    }});

    if (!isValid) {{
      if (firstError) {{
        firstError.scrollIntoView({{ behavior: 'smooth', block: 'center' }});
        firstError.focus();
      }}
      return;
    }}

    // Collect data
    const data = {{}};
    fields.forEach(function(field) {{
      const fieldGroup = field.closest('[data-field-id]');
      if (fieldGroup && fieldGroup.style.display === 'none') return;

      const name = field.name;
      let value = field.value;

      if (field.type === 'checkbox') {{
        value = field.checked;
      }} else if (field.type === 'number' || field.type === 'range') {{
        value = parseFloat(value) || 0;
      }} else if (field.type === 'hidden') {{
        try {{
          value = JSON.parse(value);
        }} catch (e) {{
          // Keep as string
        }}
      }}

      data[name] = value;
    }});

    // Build result
    const result = {{
      form_id: formId,
      data: data
    }};

    // Serialize
    const message = '__formora__' + JSON.stringify(result);

    // Dispatch event
    window.dispatchEvent(new CustomEvent('formora:submit', {{
      detail: {{
        raw: message,
        parsed: data
      }}
    }}));

    // Hide form, show success
    form.style.display = 'none';
    const successDiv = document.getElementById(formId + '-success');
    if (successDiv) {{
      successDiv.style.display = '';
    }}
  }}

  // Initialize on DOM ready
  function init() {{
    const form = getForm();
    const widget = getWidget();
    if (!form || !widget) return;

    // Set up visibility updates on input changes
    widget.addEventListener('change', updateFieldVisibility);
    widget.addEventListener('input', updateFieldVisibility);

    // Initial visibility check
    updateFieldVisibility();

    // Multi-select widget
    initMultiSelect();

    // Form submission
    form.addEventListener('submit', submitForm);

    // Multi-step navigation
    if (isMultiStep) {{
      widget.addEventListener('click', function(e) {{
        const action = e.target.getAttribute('data-action');
        if (!action) return;

        e.preventDefault();

        if (action === 'next') {{
          if (validateCurrentStep()) {{
            showStep(currentStep + 1);
          }}
        }} else if (action === 'prev') {{
          showStep(currentStep - 1);
        }}
      }});

      // Show first step
      showStep(0);
    }}
  }}

  // Run initialization
  if (document.readyState === 'loading') {{
    document.addEventListener('DOMContentLoaded', init);
  }} else {{
    init();
  }}
}})();
</script>"#,
        form_id,
        is_multi_step,
        steps_count,
        hidden_class_js,
        error_class_js,
        tag_class_js,
        tag_remove_class_js,
        option_class_js
    )
}
