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

    // Render fields
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
    let is_visible = if let Some(condition) = &field.show_if {
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
            let checked = field.default_value.as_ref().map(|v| v.as_bool()).unwrap_or(false);
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
            let min = field.min.unwrap_or(0);
            let max = field.max.unwrap_or(100);
            let step = field.step.unwrap_or(1);
            let default_val = field.default_value.as_ref().and_then(|v| v.as_f64()).unwrap_or(min as f64);
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
            // This will be fully implemented in Phase 2
            let wrapper_class = &schema.css_profile.multi_select_wrapper;
            if !wrapper_class.is_empty() {
                html.push_str(&format!(
                    r#"<div class="{}" id="{}-container" data-rules='{}'>"#,
                    wrapper_class, field.id, data_rules
                ));
            } else {
                html.push_str(&format!(
                    r#"<div id="{}-container" data-rules='{}'>"#,
                    field.id, data_rules
                ));
            }
            html.push_str("</div>");
            html.push_str(&format!(r#"<input type="hidden" id="{}" name="{}" value="[]">"#, field.id, field.id));
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
    // This will be fully implemented in Phase 2
    // For now, return a basic script placeholder
    format!(
        r#"<script>
(function() {{
  const formId = "{}";
  // Interactive behavior will be added in Phase 2
  console.log('Formora widget loaded:', formId);
}})();
</script>"#,
        schema.form_id
    )
}
