use crate::schema::Condition;

pub fn serialize_condition(condition: &Condition) -> String {
    serde_json::to_string(condition).unwrap_or_else(|_| "{}".to_string())
}

pub fn evaluate_condition(condition: &Condition, field_value: &serde_json::Value) -> bool {
    match condition.operator.as_str() {
        "eq" => {
            let cond_val_str = value_to_string(&condition.value);
            let field_val_str = value_to_string(field_value);
            cond_val_str == field_val_str
        }
        "neq" => {
            let cond_val_str = value_to_string(&condition.value);
            let field_val_str = value_to_string(field_value);
            cond_val_str != field_val_str
        }
        "contains" => {
            let field_val_str = value_to_string(field_value);
            field_val_str.contains(&value_to_string(&condition.value))
        }
        "gt" => {
            let cond_num = value_to_f64(&condition.value);
            let field_num = value_to_f64(field_value);
            match (field_num, cond_num) {
                (Some(f), Some(c)) => f > c,
                _ => false,
            }
        }
        "lt" => {
            let cond_num = value_to_f64(&condition.value);
            let field_num = value_to_f64(field_value);
            match (field_num, cond_num) {
                (Some(f), Some(c)) => f < c,
                _ => false,
            }
        }
        "in_list" => {
            if let Some(arr) = condition.value.as_array() {
                let field_val_str = value_to_string(field_value);
                arr.iter().any(|v| value_to_string(v) == field_val_str)
            } else {
                false
            }
        }
        _ => false,
    }
}

fn value_to_string(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        _ => value.to_string(),
    }
}

fn value_to_f64(value: &serde_json::Value) -> Option<f64> {
    match value {
        serde_json::Value::Number(n) => n.as_f64(),
        serde_json::Value::String(s) => s.parse::<f64>().ok(),
        serde_json::Value::Bool(b) => Some(if *b { 1.0 } else { 0.0 }),
        _ => None,
    }
}
