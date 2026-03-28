use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::css::CssProfile;

/// Types of form fields
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FieldType {
    Text,
    Email,
    Number,
    Textarea,
    Select,
    MultiSelect,
    Checkbox,
    Radio,
    Date,
    Range,
    File,
    Hidden,
}

/// Validation rule for a field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    pub rule_type: String,
    pub value: Option<serde_json::Value>,
    pub message: Option<String>,
}

/// Condition for showing/hiding a field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub field_id: String,
    pub operator: String,
    pub value: serde_json::Value,
}

/// Option for select/radio fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectOption {
    pub label: String,
    pub value: String,
}

/// Field schema definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldSchema {
    pub id: String,
    pub field_type: FieldType,
    pub label: String,
    pub placeholder: Option<String>,
    pub default_value: Option<serde_json::Value>,
    pub options: Option<Vec<SelectOption>>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub step: Option<f64>,
    pub rows: Option<u32>,
    pub accept: Option<Vec<String>>,
    pub required: bool,
    pub help_text: Option<String>,
    pub rules: Vec<ValidationRule>,
    pub show_if: Option<Condition>,
    pub step_index: Option<usize>,
}

/// Metadata about a step in a multi-step form
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepMeta {
    pub index: usize,
    pub title: Option<String>,
    pub field_ids: Vec<String>,
}

/// Complete form schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormSchema {
    pub form_id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub fields: Vec<FieldSchema>,
    pub submit_label: String,
    pub success_message: String,
    pub multi_step: bool,
    pub steps: Vec<StepMeta>,
    pub css_framework: String,
    pub css_profile: CssProfile,
}

impl FormSchema {
    pub fn new(form_id: String) -> Self {
        FormSchema {
            form_id,
            title: None,
            description: None,
            fields: Vec::new(),
            submit_label: "Submit".to_string(),
            success_message: "Form submitted successfully!".to_string(),
            multi_step: false,
            steps: Vec::new(),
            css_framework: "bootstrap".to_string(),
            css_profile: CssProfile::default(),
        }
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn with_css(mut self, framework: String, profile: CssProfile) -> Self {
        self.css_framework = framework;
        self.css_profile = profile;
        self
    }

    pub fn with_submit_label(mut self, label: String) -> Self {
        self.submit_label = label;
        self
    }

    pub fn with_success_message(mut self, message: String) -> Self {
        self.success_message = message;
        self
    }

    pub fn with_multi_step(mut self, multi_step: bool) -> Self {
        self.multi_step = multi_step;
        self
    }

    pub fn add_field(&mut self, field: FieldSchema) {
        self.fields.push(field);
    }

    pub fn add_step(&mut self, step: StepMeta) {
        self.steps.push(step);
    }
}

impl Default for FormSchema {
    fn default() -> Self {
        Self::new("default_form".to_string())
    }
}
