use js_sys::Array;
use wasm_bindgen::prelude::*;

use formora_core::{CssProfile, FieldSchema, FieldType, FormSchema, SelectOption};

use crate::css::{resolve_profile, CssFramework, JsCssProfile};
use crate::rules::{JsCondition, Rule};

/// Fluent form builder — mirrors Python's PyForm
#[wasm_bindgen]
pub struct Form {
    pub(crate) schema: FormSchema,
}

#[wasm_bindgen]
impl Form {
    /// Create a new form. Auto-generates a UUID if `id` is not provided.
    #[wasm_bindgen(constructor)]
    pub fn new(id: Option<String>) -> Form {
        let form_id = id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        Form { schema: FormSchema::new(form_id) }
    }

    /// Set the form title
    pub fn title(mut self, text: String) -> Form {
        self.schema.title = Some(text);
        self
    }

    /// Set the form description
    pub fn description(mut self, text: String) -> Form {
        self.schema.description = Some(text);
        self
    }

    /// Set styling via a CssFramework or JsCssProfile instance
    pub fn css(mut self, framework_or_profile: JsValue) -> Form {
        let profile = if let Some(fw) = framework_or_profile.clone().dyn_ref::<CssFramework>() {
            resolve_profile(&fw.value())
        } else if let Some(p) = framework_or_profile.dyn_ref::<JsCssProfile>() {
            p.profile.clone()
        } else {
            CssProfile::bootstrap()
        };
        self.schema.css_profile = profile;
        self
    }

    /// Add a step — enables multi-step mode
    pub fn step(mut self, title: Option<String>) -> Form {
        self.schema.multi_step = true;
        self.schema.steps.push(formora_core::schema::StepMeta {
            index: self.schema.steps.len(),
            title,
            field_ids: vec![],
        });
        self
    }

    /// Set the submit button label
    #[wasm_bindgen(js_name = submitLabel)]
    pub fn submit_label(mut self, text: String) -> Form {
        self.schema.submit_label = text;
        self
    }

    /// Set the success message shown after form submission
    #[wasm_bindgen(js_name = successMessage)]
    pub fn success_message(mut self, text: String) -> Form {
        self.schema.success_message = text;
        self
    }

    /// Render the form to an HTML string
    pub fn build(&self) -> String {
        formora_core::renderer::render(&self.schema)
    }

    // ── Field methods ─────────────────────────────────────────────────────────

    /// Add a single-line text input
    pub fn text(
        mut self,
        id: String,
        label: String,
        placeholder: Option<String>,
        required: Option<bool>,
        help_text: Option<String>,
        default: Option<String>,
        rules: Option<Vec<Rule>>,
        show_if: Option<JsCondition>,
    ) -> Form {
        let rule_vec = rules.unwrap_or_default().into_iter().map(|r| r.rule).collect();
        self.schema.fields.push(FieldSchema {
            id, field_type: FieldType::Text, label, placeholder,
            default_value: default.map(|v| serde_json::json!(v)),
            options: None, min: None, max: None, step: None, rows: None, accept: None,
            required: required.unwrap_or(false), help_text, rules: rule_vec,
            show_if: show_if.map(|c| c.condition), step_index: None,
        });
        self
    }

    /// Add an email input
    pub fn email(
        mut self,
        id: String,
        label: String,
        required: Option<bool>,
        help_text: Option<String>,
        default: Option<String>,
        rules: Option<Vec<Rule>>,
        show_if: Option<JsCondition>,
    ) -> Form {
        let rule_vec = rules.unwrap_or_default().into_iter().map(|r| r.rule).collect();
        self.schema.fields.push(FieldSchema {
            id, field_type: FieldType::Email, label, placeholder: None,
            default_value: default.map(|v| serde_json::json!(v)),
            options: None, min: None, max: None, step: None, rows: None, accept: None,
            required: required.unwrap_or(false), help_text, rules: rule_vec,
            show_if: show_if.map(|c| c.condition), step_index: None,
        });
        self
    }

    /// Add a number input
    pub fn number(
        mut self,
        id: String,
        label: String,
        min: Option<f64>,
        max: Option<f64>,
        required: Option<bool>,
        help_text: Option<String>,
        default: Option<f64>,
        rules: Option<Vec<Rule>>,
        show_if: Option<JsCondition>,
    ) -> Form {
        let rule_vec = rules.unwrap_or_default().into_iter().map(|r| r.rule).collect();
        self.schema.fields.push(FieldSchema {
            id, field_type: FieldType::Number, label, placeholder: None,
            default_value: default.map(|v| serde_json::json!(v)),
            options: None, min, max, step: None, rows: None, accept: None,
            required: required.unwrap_or(false), help_text, rules: rule_vec,
            show_if: show_if.map(|c| c.condition), step_index: None,
        });
        self
    }

    /// Add a multi-line textarea
    pub fn textarea(
        mut self,
        id: String,
        label: String,
        rows: Option<u32>,
        placeholder: Option<String>,
        required: Option<bool>,
        help_text: Option<String>,
        default: Option<String>,
        rules: Option<Vec<Rule>>,
        show_if: Option<JsCondition>,
    ) -> Form {
        let rule_vec = rules.unwrap_or_default().into_iter().map(|r| r.rule).collect();
        self.schema.fields.push(FieldSchema {
            id, field_type: FieldType::Textarea, label, placeholder,
            default_value: default.map(|v| serde_json::json!(v)),
            options: None, min: None, max: None, step: None, rows, accept: None,
            required: required.unwrap_or(false), help_text, rules: rule_vec,
            show_if: show_if.map(|c| c.condition), step_index: None,
        });
        self
    }

    /// Add a dropdown select.  `options` is a JS array of `[label, value]` pairs.
    pub fn select(
        mut self,
        id: String,
        label: String,
        options: Vec<JsValue>,
        required: Option<bool>,
        help_text: Option<String>,
        default: Option<String>,
        rules: Option<Vec<Rule>>,
        show_if: Option<JsCondition>,
    ) -> Form {
        let rule_vec = rules.unwrap_or_default().into_iter().map(|r| r.rule).collect();
        let option_vec = parse_options(options);
        self.schema.fields.push(FieldSchema {
            id, field_type: FieldType::Select, label, placeholder: None,
            default_value: default.map(|v| serde_json::json!(v)),
            options: Some(option_vec), min: None, max: None, step: None, rows: None, accept: None,
            required: required.unwrap_or(false), help_text, rules: rule_vec,
            show_if: show_if.map(|c| c.condition), step_index: None,
        });
        self
    }

    /// Add a multi-select field.  `options` is a JS array of `[label, value]` pairs.
    #[wasm_bindgen(js_name = multiSelect)]
    pub fn multi_select(
        mut self,
        id: String,
        label: String,
        options: Vec<JsValue>,
        required: Option<bool>,
        help_text: Option<String>,
        default: Option<Vec<String>>,
        rules: Option<Vec<Rule>>,
        show_if: Option<JsCondition>,
    ) -> Form {
        let rule_vec = rules.unwrap_or_default().into_iter().map(|r| r.rule).collect();
        let option_vec = parse_options(options);
        self.schema.fields.push(FieldSchema {
            id, field_type: FieldType::MultiSelect, label, placeholder: None,
            default_value: default.map(|v| serde_json::json!(v)),
            options: Some(option_vec), min: None, max: None, step: None, rows: None, accept: None,
            required: required.unwrap_or(false), help_text, rules: rule_vec,
            show_if: show_if.map(|c| c.condition), step_index: None,
        });
        self
    }

    /// Add a checkbox (boolean toggle)
    pub fn checkbox(
        mut self,
        id: String,
        label: String,
        default: Option<bool>,
        help_text: Option<String>,
        rules: Option<Vec<Rule>>,
        show_if: Option<JsCondition>,
    ) -> Form {
        let rule_vec = rules.unwrap_or_default().into_iter().map(|r| r.rule).collect();
        self.schema.fields.push(FieldSchema {
            id, field_type: FieldType::Checkbox, label, placeholder: None,
            default_value: default.map(|v| serde_json::json!(v)),
            options: None, min: None, max: None, step: None, rows: None, accept: None,
            required: false, help_text, rules: rule_vec,
            show_if: show_if.map(|c| c.condition), step_index: None,
        });
        self
    }

    /// Add a radio button group.  `options` is a JS array of `[label, value]` pairs.
    pub fn radio(
        mut self,
        id: String,
        label: String,
        options: Vec<JsValue>,
        required: Option<bool>,
        help_text: Option<String>,
        default: Option<String>,
        rules: Option<Vec<Rule>>,
        show_if: Option<JsCondition>,
    ) -> Form {
        let rule_vec = rules.unwrap_or_default().into_iter().map(|r| r.rule).collect();
        let option_vec = parse_options(options);
        self.schema.fields.push(FieldSchema {
            id, field_type: FieldType::Radio, label, placeholder: None,
            default_value: default.map(|v| serde_json::json!(v)),
            options: Some(option_vec), min: None, max: None, step: None, rows: None, accept: None,
            required: required.unwrap_or(false), help_text, rules: rule_vec,
            show_if: show_if.map(|c| c.condition), step_index: None,
        });
        self
    }

    /// Add a date picker
    pub fn date(
        mut self,
        id: String,
        label: String,
        required: Option<bool>,
        help_text: Option<String>,
        default: Option<String>,
        rules: Option<Vec<Rule>>,
        show_if: Option<JsCondition>,
    ) -> Form {
        let rule_vec = rules.unwrap_or_default().into_iter().map(|r| r.rule).collect();
        self.schema.fields.push(FieldSchema {
            id, field_type: FieldType::Date, label, placeholder: None,
            default_value: default.map(|v| serde_json::json!(v)),
            options: None, min: None, max: None, step: None, rows: None, accept: None,
            required: required.unwrap_or(false), help_text, rules: rule_vec,
            show_if: show_if.map(|c| c.condition), step_index: None,
        });
        self
    }

    /// Add a range slider
    pub fn range(
        mut self,
        id: String,
        label: String,
        min: f64,
        max: f64,
        step: Option<f64>,
        default: Option<f64>,
        help_text: Option<String>,
        show_if: Option<JsCondition>,
    ) -> Form {
        self.schema.fields.push(FieldSchema {
            id, field_type: FieldType::Range, label, placeholder: None,
            default_value: default.map(|v| serde_json::json!(v)),
            options: None, min: Some(min), max: Some(max), step, rows: None, accept: None,
            required: false, help_text, rules: vec![],
            show_if: show_if.map(|c| c.condition), step_index: None,
        });
        self
    }

    /// Add a file upload field.  `accept` is an array of MIME types or extensions.
    pub fn file(
        mut self,
        id: String,
        label: String,
        accept: Option<Vec<String>>,
        required: Option<bool>,
        help_text: Option<String>,
        rules: Option<Vec<Rule>>,
        show_if: Option<JsCondition>,
    ) -> Form {
        let rule_vec = rules.unwrap_or_default().into_iter().map(|r| r.rule).collect();
        self.schema.fields.push(FieldSchema {
            id, field_type: FieldType::File, label, placeholder: None,
            default_value: None, options: None, min: None, max: None, step: None,
            rows: None, accept, required: required.unwrap_or(false), help_text, rules: rule_vec,
            show_if: show_if.map(|c| c.condition), step_index: None,
        });
        self
    }

    /// Add a hidden field carrying context data (not displayed)
    pub fn hidden(mut self, id: String, value: String) -> Form {
        self.schema.fields.push(FieldSchema {
            id, field_type: FieldType::Hidden, label: String::new(), placeholder: None,
            default_value: Some(serde_json::json!(value)),
            options: None, min: None, max: None, step: None, rows: None, accept: None,
            required: false, help_text: None, rules: vec![],
            show_if: None, step_index: None,
        });
        self
    }
}

/// Parse a JS array of `[label, value]` pairs into SelectOption vec
fn parse_options(options: Vec<JsValue>) -> Vec<SelectOption> {
    options
        .into_iter()
        .filter_map(|item| {
            let arr = Array::from(&item);
            let label = arr.get(0).as_string()?;
            let value = arr.get(1).as_string()?;
            Some(SelectOption { label, value })
        })
        .collect()
}
