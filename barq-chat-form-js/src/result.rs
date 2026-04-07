use wasm_bindgen::prelude::*;

use barq_chat_form_core::{parse, is_barq_message};

use crate::helpers::json_to_js;

/// Parsed form submission result
#[wasm_bindgen]
pub struct FormResult {
    result: barq_chat_form_core::FormResult,
}

#[wasm_bindgen]
impl FormResult {
    /// The form ID this result belongs to
    #[wasm_bindgen(getter, js_name = formId)]
    pub fn form_id(&self) -> String {
        self.result.form_id.clone()
    }

    /// Raw form data as a JS object (all values as strings)
    #[wasm_bindgen(getter)]
    pub fn data(&self) -> JsValue {
        let val = serde_json::to_value(&self.result.data)
            .unwrap_or(serde_json::Value::Null);
        json_to_js(&val)
    }

    /// Type-coerced form data as a JS object (numbers, booleans, arrays inferred)
    #[wasm_bindgen(getter, js_name = typedData)]
    pub fn typed_data(&self) -> JsValue {
        let val = serde_json::to_value(&self.result.typed_data)
            .unwrap_or(serde_json::Value::Null);
        json_to_js(&val)
    }

    /// Human-readable formatted summary of the result
    #[wasm_bindgen(js_name = asText)]
    pub fn as_text(&self) -> String {
        self.result.as_text()
    }
}

/// Parse a `__barq__{...}` message into a FormResult.
/// Returns `undefined` if the message is not a barq submission.
#[wasm_bindgen(js_name = parseMessage)]
pub fn parse_message(message: String) -> Option<FormResult> {
    parse(&message).map(|result| FormResult { result })
}

/// Returns `true` if the message starts with the barq prefix
#[wasm_bindgen(js_name = isBarq)]
pub fn is_barq(message: String) -> bool {
    is_barq_message(&message)
}
