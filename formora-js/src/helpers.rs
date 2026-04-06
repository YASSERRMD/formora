use js_sys::{Array, Object, Reflect};
use wasm_bindgen::prelude::*;

/// Convert a JS value to serde_json::Value
pub fn js_to_json(val: &JsValue) -> serde_json::Value {
    if val.is_null() || val.is_undefined() {
        serde_json::Value::Null
    } else if let Some(b) = val.as_bool() {
        serde_json::Value::Bool(b)
    } else if let Some(n) = val.as_f64() {
        serde_json::json!(n)
    } else if let Some(s) = val.as_string() {
        serde_json::Value::String(s)
    } else if Array::is_array(val) {
        let arr = Array::from(val);
        let vec: Vec<serde_json::Value> = (0..arr.length())
            .map(|i| js_to_json(&arr.get(i)))
            .collect();
        serde_json::Value::Array(vec)
    } else if val.is_object() {
        let obj = Object::from(val.clone());
        let keys = Object::keys(&obj);
        let mut map = serde_json::Map::new();
        for i in 0..keys.length() {
            if let Some(key) = keys.get(i).as_string() {
                let v = Reflect::get(&obj, &JsValue::from_str(&key))
                    .unwrap_or(JsValue::UNDEFINED);
                map.insert(key, js_to_json(&v));
            }
        }
        serde_json::Value::Object(map)
    } else {
        serde_json::Value::Null
    }
}

/// Convert serde_json::Value to a JS value
pub fn json_to_js(val: &serde_json::Value) -> JsValue {
    match val {
        serde_json::Value::Null => JsValue::NULL,
        serde_json::Value::Bool(b) => JsValue::from_bool(*b),
        serde_json::Value::Number(n) => {
            if let Some(f) = n.as_f64() {
                JsValue::from_f64(f)
            } else {
                JsValue::from_str(&n.to_string())
            }
        }
        serde_json::Value::String(s) => JsValue::from_str(s),
        serde_json::Value::Array(arr) => {
            let js_arr = Array::new();
            for item in arr {
                js_arr.push(&json_to_js(item));
            }
            js_arr.into()
        }
        serde_json::Value::Object(obj) => {
            let js_obj = Object::new();
            for (k, v) in obj {
                Reflect::set(&js_obj, &JsValue::from_str(k), &json_to_js(v))
                    .unwrap_or(false);
            }
            js_obj.into()
        }
    }
}
