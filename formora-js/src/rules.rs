use wasm_bindgen::prelude::*;

use formora_core::{Condition, ValidationRule};

use crate::helpers::js_to_json;

/// Validation rule for a form field
#[wasm_bindgen]
#[derive(Clone)]
pub struct Rule {
    pub(crate) rule: ValidationRule,
}

#[wasm_bindgen]
impl Rule {
    /// Field must have a non-empty value
    pub fn required(message: Option<String>) -> Rule {
        Rule {
            rule: ValidationRule { rule_type: "required".to_string(), value: None, message },
        }
    }

    /// String must be at least `n` characters
    #[wasm_bindgen(js_name = minLength)]
    pub fn min_length(n: u32, message: Option<String>) -> Rule {
        Rule {
            rule: ValidationRule {
                rule_type: "min_length".to_string(),
                value: Some(serde_json::json!(n)),
                message,
            },
        }
    }

    /// String must be at most `n` characters
    #[wasm_bindgen(js_name = maxLength)]
    pub fn max_length(n: u32, message: Option<String>) -> Rule {
        Rule {
            rule: ValidationRule {
                rule_type: "max_length".to_string(),
                value: Some(serde_json::json!(n)),
                message,
            },
        }
    }

    /// Numeric value must be >= n
    pub fn min(n: f64, message: Option<String>) -> Rule {
        Rule {
            rule: ValidationRule {
                rule_type: "min".to_string(),
                value: Some(serde_json::json!(n)),
                message,
            },
        }
    }

    /// Numeric value must be <= n
    pub fn max(n: f64, message: Option<String>) -> Rule {
        Rule {
            rule: ValidationRule {
                rule_type: "max".to_string(),
                value: Some(serde_json::json!(n)),
                message,
            },
        }
    }

    /// Value must match the given regex pattern
    pub fn regex(pattern: String, message: Option<String>) -> Rule {
        Rule {
            rule: ValidationRule {
                rule_type: "regex".to_string(),
                value: Some(serde_json::json!(pattern)),
                message,
            },
        }
    }

    /// Value must be a valid email address
    pub fn email(message: Option<String>) -> Rule {
        Rule {
            rule: ValidationRule { rule_type: "email".to_string(), value: None, message },
        }
    }
}

/// Conditional visibility rule — show a field only when a condition is met
#[wasm_bindgen]
#[derive(Clone)]
pub struct JsCondition {
    pub(crate) condition: Condition,
}

#[wasm_bindgen]
impl JsCondition {
    /// Create a condition: show field when `field_id` `operator` `value`
    ///
    /// Operators: "eq" | "neq" | "contains" | "gt" | "lt" | "in_list"
    #[wasm_bindgen(constructor)]
    pub fn new(field_id: String, operator: String, value: JsValue) -> JsCondition {
        JsCondition {
            condition: Condition {
                field_id,
                operator,
                value: js_to_json(&value),
            },
        }
    }
}
