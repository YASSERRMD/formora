// Validation rules serialization
// Serializes validation rules into JSON for the data-rules attribute

use crate::schema::ValidationRule;

/// Serialize validation rules to a compact JSON string
pub fn serialize_rules(rules: &[ValidationRule]) -> String {
    serde_json::to_string(rules).unwrap_or_else(|_| "[]".to_string())
}

/// Get default error message for a rule type
pub fn default_message(rule_type: &str, value: Option<&serde_json::Value>) -> String {
    match rule_type {
        "required" => "This field is required".to_string(),
        "min_length" => {
            if let Some(serde_json::Value::Number(n)) = value {
                format!("Minimum {} characters required", n)
            } else {
                "Minimum length required".to_string()
            }
        }
        "max_length" => {
            if let Some(serde_json::Value::Number(n)) = value {
                format!("Maximum {} characters allowed", n)
            } else {
                "Maximum length exceeded".to_string()
            }
        }
        "min" => {
            if let Some(serde_json::Value::Number(n)) = value {
                format!("Value must be at least {}", n)
            } else {
                "Value too small".to_string()
            }
        }
        "max" => {
            if let Some(serde_json::Value::Number(n)) = value {
                format!("Value must be at most {}", n)
            } else {
                "Value too large".to_string()
            }
        }
        "regex" => "Invalid format".to_string(),
        "email" => "Please enter a valid email address".to_string(),
        _ => "Invalid value".to_string(),
    }
}

/// Helper functions to create validation rules
pub struct Rule;

impl Rule {
    pub fn required(message: Option<String>) -> ValidationRule {
        ValidationRule {
            rule_type: "required".to_string(),
            value: None,
            message,
        }
    }

    pub fn min_length(n: u64, message: Option<String>) -> ValidationRule {
        ValidationRule {
            rule_type: "min_length".to_string(),
            value: Some(serde_json::Value::Number(n.into())),
            message,
        }
    }

    pub fn max_length(n: u64, message: Option<String>) -> ValidationRule {
        ValidationRule {
            rule_type: "max_length".to_string(),
            value: Some(serde_json::Value::Number(n.into())),
            message,
        }
    }

    pub fn min(n: f64, message: Option<String>) -> ValidationRule {
        ValidationRule {
            rule_type: "min".to_string(),
            value: Some(serde_json::json!(n)),
            message,
        }
    }

    pub fn max(n: f64, message: Option<String>) -> ValidationRule {
        ValidationRule {
            rule_type: "max".to_string(),
            value: Some(serde_json::json!(n)),
            message,
        }
    }

    pub fn regex(pattern: String, message: Option<String>) -> ValidationRule {
        ValidationRule {
            rule_type: "regex".to_string(),
            value: Some(serde_json::Value::String(pattern)),
            message,
        }
    }

    pub fn email(message: Option<String>) -> ValidationRule {
        ValidationRule {
            rule_type: "email".to_string(),
            value: None,
            message,
        }
    }
}
