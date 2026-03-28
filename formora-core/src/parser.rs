pub const FORMORA_PREFIX: &str = "__formora__";

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormResult {
    pub form_id: String,
    pub data: HashMap<String, serde_json::Value>,
    pub typed_data: HashMap<String, serde_json::Value>,
}

impl FormResult {
    pub fn as_text(&self) -> String {
        let parts: Vec<String> = self
            .typed_data
            .iter()
            .map(|(key, value)| {
                let value_str = match value {
                    serde_json::Value::String(s) => s.clone(),
                    serde_json::Value::Number(n) => n.to_string(),
                    serde_json::Value::Bool(b) => b.to_string(),
                    serde_json::Value::Array(arr) => {
                        let items: Vec<String> = arr
                            .iter()
                            .map(|v| v.as_str().unwrap_or(&v.to_string()).to_string())
                            .collect();
                        items.join(", ")
                    }
                    _ => value.to_string(),
                };
                format!("{}: {}", key, value_str)
            })
            .collect();
        parts.join(", ")
    }
}

pub fn is_formora_message(message: &str) -> bool {
    message.starts_with(FORMORA_PREFIX)
}

pub fn parse(message: &str) -> Option<FormResult> {
    if !is_formora_message(message) {
        return None;
    }

    let json_str = message.strip_prefix(FORMORA_PREFIX)?;
    let parsed: serde_json::Value = serde_json::from_str(json_str).ok()?;

    let form_id = parsed.get("form_id")?.as_str()?.to_string();
    let data_obj = parsed.get("data")?.as_object()?;

    let mut data = HashMap::new();
    let mut typed_data = HashMap::new();

    for (key, value) in data_obj {
        data.insert(key.clone(), value.clone());
        typed_data.insert(key.clone(), coerce_value(value));
    }

    Some(FormResult {
        form_id,
        data,
        typed_data,
    })
}

fn coerce_value(value: &serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Number(_) => value.clone(),
        serde_json::Value::Bool(_) => value.clone(),
        serde_json::Value::Array(_) => value.clone(),
        serde_json::Value::String(s) => {
            if let Ok(n) = s.parse::<f64>() {
                return serde_json::Value::Number(
                    serde_json::Number::from_f64(n).unwrap_or(serde_json::Number::from(0)),
                );
            }
            match s.to_lowercase().as_str() {
                "true" => return serde_json::Value::Bool(true),
                "false" => return serde_json::Value::Bool(false),
                _ => {}
            }
            if let Ok(arr) = serde_json::from_str::<Vec<serde_json::Value>>(s) {
                return serde_json::Value::Array(arr);
            }
            value.clone()
        }
        _ => value.clone(),
    }
}
