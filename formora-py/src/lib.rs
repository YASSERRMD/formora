use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;

use formora_core::{
    CssProfile, FieldType, FormSchema, FieldSchema, ValidationRule, Condition, SelectOption, StepMeta,
    parse, is_formora_message, FormResult,
    validator::serialize_rules,
    conditions::serialize_condition,
    builder::FormBuilder,
};

/// Python wrapper for CssFramework enum
#[pyclass]
#[derive(Clone, Debug)]
pub struct CssFramework {
    #[pyo3(get, set)]
    pub value: String,
}

#[pymethods]
impl CssFramework {
    #[new]
    fn new(value: String) -> Self {
        CssFramework { value }
    }

    #[staticmethod]
    fn bootstrap() -> Self {
        CssFramework { value: "bootstrap".to_string() }
    }

    #[staticmethod]
    fn tailwind() -> Self {
        CssFramework { value: "tailwind".to_string() }
    }

    #[staticmethod]
    fn custom() -> Self {
        CssFramework { value: "custom".to_string() }
    }
}

/// Python wrapper for CssProfile
#[pyclass]
#[derive(Clone)]
pub struct PyCssProfile {
    profile: CssProfile,
}

#[pymethods]
impl PyCssProfile {
    #[new]
    #[pyo3(signature = (framework=None))]
    fn new(framework: Option<CssFramework>) -> Self {
        let framework = framework.unwrap_or_else(CssFramework::bootstrap);
        let profile = match framework.value.as_str() {
            "bootstrap" => CssProfile::bootstrap(),
            "tailwind" => CssProfile::tailwind(),
            "custom" => CssProfile::custom(),
            _ => CssProfile::bootstrap(),
        };
        PyCssProfile { profile }
    }

    #[staticmethod]
    fn from_dict(d: &PyDict) -> PyResult<Self> {
        let mut map = HashMap::new();
        for (key, value) in d.iter() {
            let key_str = key.extract::<String>()?;
            let value_str = value.extract::<String>()?;
            map.insert(key_str, value_str);
        }
        Ok(PyCssProfile {
            profile: CssProfile::from_hashmap(map),
        })
    }

    #[pyo3(signature = (**kwargs))]
    fn override_(&self, py: Python<'_>, kwargs: Option<&PyDict>) -> PyResult<PyCssProfile> {
        let kwargs = kwargs.unwrap_or_else(|| PyDict::new(py));
        let mut map = HashMap::new();

        // Copy all current field values to map
        macro_rules! add_to_map {
            ($field:ident) => {{
                if !self.profile.$field.is_empty() {
                    map.insert(stringify!($field).to_string(), self.profile.$field.clone());
                }
            }};
        }

        add_to_map!(form_wrapper);
        add_to_map!(form_title);
        add_to_map!(form_description);
        add_to_map!(step_wrapper);
        add_to_map!(step_title);
        add_to_map!(progress_bar_wrapper);
        add_to_map!(progress_bar_fill);
        add_to_map!(progress_bar_label);
        add_to_map!(field_group);
        add_to_map!(field_label);
        add_to_map!(field_required_marker);
        add_to_map!(field_help_text);
        add_to_map!(field_error_message);
        add_to_map!(input_text);
        add_to_map!(input_email);
        add_to_map!(input_number);
        add_to_map!(input_textarea);
        add_to_map!(input_select);
        add_to_map!(input_date);
        add_to_map!(input_file);
        add_to_map!(input_range);
        add_to_map!(input_range_value_display);
        add_to_map!(input_checkbox_wrapper);
        add_to_map!(input_checkbox);
        add_to_map!(input_checkbox_label);
        add_to_map!(input_radio_wrapper);
        add_to_map!(input_radio);
        add_to_map!(input_radio_label);
        add_to_map!(multi_select_wrapper);
        add_to_map!(multi_select_tag);
        add_to_map!(multi_select_tag_remove);
        add_to_map!(multi_select_dropdown);
        add_to_map!(multi_select_option);
        add_to_map!(input_error_state);
        add_to_map!(input_valid_state);
        add_to_map!(field_hidden);
        add_to_map!(button_submit);
        add_to_map!(button_next);
        add_to_map!(button_back);
        add_to_map!(button_wrapper);
        add_to_map!(success_wrapper);
        add_to_map!(success_message);
        add_to_map!(error_wrapper);
        add_to_map!(error_message);

        // Add overrides
        for (key, value) in kwargs.iter() {
            let key_str = key.extract::<String>()?;
            let value_str = value.extract::<String>()?;
            map.insert(key_str, value_str);
        }

        Ok(PyCssProfile {
            profile: CssProfile::from_hashmap(map),
        })
    }
}

/// Python wrapper for Rule
#[pyclass]
#[derive(Clone)]
pub struct PyRule {
    rule: ValidationRule,
}

#[pymethods]
impl PyRule {
    #[staticmethod]
    fn required(message: Option<String>) -> Self {
        PyRule {
            rule: ValidationRule {
                rule_type: "required".to_string(),
                value: None,
                message,
            },
        }
    }

    #[staticmethod]
    fn min_length(n: u64, message: Option<String>) -> Self {
        PyRule {
            rule: ValidationRule {
                rule_type: "min_length".to_string(),
                value: Some(serde_json::json!(n)),
                message,
            },
        }
    }

    #[staticmethod]
    fn max_length(n: u64, message: Option<String>) -> Self {
        PyRule {
            rule: ValidationRule {
                rule_type: "max_length".to_string(),
                value: Some(serde_json::json!(n)),
                message,
            },
        }
    }

    #[staticmethod]
    fn min(n: f64, message: Option<String>) -> Self {
        PyRule {
            rule: ValidationRule {
                rule_type: "min".to_string(),
                value: Some(serde_json::json!(n)),
                message,
            },
        }
    }

    #[staticmethod]
    fn max(n: f64, message: Option<String>) -> Self {
        PyRule {
            rule: ValidationRule {
                rule_type: "max".to_string(),
                value: Some(serde_json::json!(n)),
                message,
            },
        }
    }

    #[staticmethod]
    fn regex(pattern: String, message: Option<String>) -> Self {
        PyRule {
            rule: ValidationRule {
                rule_type: "regex".to_string(),
                value: Some(serde_json::json!(pattern)),
                message,
            },
        }
    }

    #[staticmethod]
    fn email(message: Option<String>) -> Self {
        PyRule {
            rule: ValidationRule {
                rule_type: "email".to_string(),
                value: None,
                message,
            },
        }
    }
}

/// Python wrapper for Condition
#[pyclass]
#[derive(Clone)]
pub struct PyCondition {
    condition: Condition,
}

#[pymethods]
impl PyCondition {
    #[new]
    fn new(field_id: String, operator: String, value: Py<PyAny>) -> PyResult<Self> {
        // Convert Python value to serde_json::Value
        let json_value = python_to_json(value)?;
        Ok(PyCondition {
            condition: Condition {
                field_id,
                operator,
                value: json_value,
            },
        })
    }
}

/// Python wrapper for Form
#[pyclass]
pub struct PyForm {
    schema: FormSchema,
}

#[pymethods]
impl PyForm {
    #[new]
    #[pyo3(signature = (id=None))]
    fn new(id: Option<String>) -> Self {
        let form_id = id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        PyForm {
            schema: FormSchema::new(form_id),
        }
    }

    fn title(&mut self, text: String) -> PyResult<PyForm> {
        self.schema.title = Some(text);
        Ok(PyForm { schema: self.schema.clone() })
    }

    fn description(&mut self, text: String) -> PyResult<PyForm> {
        self.schema.description = Some(text);
        Ok(PyForm { schema: self.schema.clone() })
    }

    fn css(&mut self, py: Python<'_>, framework_or_profile: Py<PyAny>) -> PyResult<PyForm> {
        // Check if it's a CssFramework or CssProfile
        let profile = if let Ok(fw) = framework_or_profile.extract::<CssFramework>(py) {
            match fw.value.as_str() {
                "bootstrap" => CssProfile::bootstrap(),
                "tailwind" => CssProfile::tailwind(),
                "custom" => CssProfile::custom(),
                _ => CssProfile::bootstrap(),
            }
        } else if let Ok(profile) = framework_or_profile.extract::<PyCssProfile>(py) {
            profile.profile.clone()
        } else {
            CssProfile::bootstrap()
        };

        self.schema.css_profile = profile;
        Ok(PyForm { schema: self.schema.clone() })
    }

    fn step(&mut self, title: Option<String>) -> PyResult<PyForm> {
        self.schema.multi_step = true;
        self.schema.steps.push(formora_core::schema::StepMeta {
            index: self.schema.steps.len(),
            title,
            field_ids: vec![],
        });
        Ok(PyForm { schema: self.schema.clone() })
    }

    fn text(&mut self, id: String, label: String, placeholder: Option<String>, required: Option<bool>,
            help_text: Option<String>, default: Option<Py<PyAny>>, rules: Option<Vec<PyRule>>,
            show_if: Option<PyCondition>) -> PyResult<PyForm> {
        let default_value = default.map(|v| python_to_json(v)).transpose()?;
        let rule_vec = rules.unwrap_or_default().into_iter().map(|r| r.rule).collect();
        let show_if_cond = show_if.map(|c| c.condition);

        let field = FieldSchema {
            id,
            field_type: FieldType::Text,
            label,
            placeholder,
            default_value,
            options: None,
            min: None,
            max: None,
            step: None,
            rows: None,
            accept: None,
            required: required.unwrap_or(false),
            help_text,
            rules: rule_vec,
            show_if: show_if_cond,
            step_index: None,
        };

        self.schema.fields.push(field);
        Ok(PyForm { schema: self.schema.clone() })
    }

    fn email(&mut self, id: String, label: String, required: Option<bool>, help_text: Option<String>,
             default: Option<Py<PyAny>>, rules: Option<Vec<PyRule>>, show_if: Option<PyCondition>) -> PyResult<PyForm> {
        let default_value = default.map(|v| python_to_json(v)).transpose()?;
        let rule_vec = rules.unwrap_or_default().into_iter().map(|r| r.rule).collect();
        let show_if_cond = show_if.map(|c| c.condition);

        let field = FieldSchema {
            id,
            field_type: FieldType::Email,
            label,
            placeholder: None,
            default_value,
            options: None,
            min: None,
            max: None,
            step: None,
            rows: None,
            accept: None,
            required: required.unwrap_or(false),
            help_text,
            rules: rule_vec,
            show_if: show_if_cond,
            step_index: None,
        };

        self.schema.fields.push(field);
        Ok(PyForm { schema: self.schema.clone() })
    }

    fn number(&mut self, id: String, label: String, min: Option<f64>, max: Option<f64>,
              required: Option<bool>, help_text: Option<String>, default: Option<Py<PyAny>>,
              rules: Option<Vec<PyRule>>, show_if: Option<PyCondition>) -> PyResult<PyForm> {
        let default_value = default.map(|v| python_to_json(v)).transpose()?;
        let rule_vec = rules.unwrap_or_default().into_iter().map(|r| r.rule).collect();
        let show_if_cond = show_if.map(|c| c.condition);

        let field = FieldSchema {
            id,
            field_type: FieldType::Number,
            label,
            placeholder: None,
            default_value,
            options: None,
            min,
            max,
            step: None,
            rows: None,
            accept: None,
            required: required.unwrap_or(false),
            help_text,
            rules: rule_vec,
            show_if: show_if_cond,
            step_index: None,
        };

        self.schema.fields.push(field);
        Ok(PyForm { schema: self.schema.clone() })
    }

    fn textarea(&mut self, id: String, label: String, rows: Option<u32>, placeholder: Option<String>,
                required: Option<bool>, help_text: Option<String>, default: Option<Py<PyAny>>,
                rules: Option<Vec<PyRule>>, show_if: Option<PyCondition>) -> PyResult<PyForm> {
        let default_value = default.map(|v| python_to_json(v)).transpose()?;
        let rule_vec = rules.unwrap_or_default().into_iter().map(|r| r.rule).collect();
        let show_if_cond = show_if.map(|c| c.condition);

        let field = FieldSchema {
            id,
            field_type: FieldType::Textarea,
            label,
            placeholder,
            default_value,
            options: None,
            min: None,
            max: None,
            step: None,
            rows,
            accept: None,
            required: required.unwrap_or(false),
            help_text,
            rules: rule_vec,
            show_if: show_if_cond,
            step_index: None,
        };

        self.schema.fields.push(field);
        Ok(PyForm { schema: self.schema.clone() })
    }

    fn select(&mut self, id: String, label: String, options: Vec<(String, String)>,
              required: Option<bool>, help_text: Option<String>, default: Option<Py<PyAny>>,
              rules: Option<Vec<PyRule>>, show_if: Option<PyCondition>) -> PyResult<PyForm> {
        let default_value = default.map(|v| python_to_json(v)).transpose()?;
        let rule_vec = rules.unwrap_or_default().into_iter().map(|r| r.rule).collect();
        let show_if_cond = show_if.map(|c| c.condition);

        let option_vec = options.into_iter().map(|(label, value)| SelectOption { label, value }).collect();

        let field = FieldSchema {
            id,
            field_type: FieldType::Select,
            label,
            placeholder: None,
            default_value,
            options: Some(option_vec),
            min: None,
            max: None,
            step: None,
            rows: None,
            accept: None,
            required: required.unwrap_or(false),
            help_text,
            rules: rule_vec,
            show_if: show_if_cond,
            step_index: None,
        };

        self.schema.fields.push(field);
        Ok(PyForm { schema: self.schema.clone() })
    }

    fn multi_select(&mut self, id: String, label: String, options: Vec<(String, String)>,
                    required: Option<bool>, help_text: Option<String>, default: Option<Py<PyAny>>,
                    rules: Option<Vec<PyRule>>, show_if: Option<PyCondition>) -> PyResult<PyForm> {
        let default_value = default.map(|v| python_to_json(v)).transpose()?;
        let rule_vec = rules.unwrap_or_default().into_iter().map(|r| r.rule).collect();
        let show_if_cond = show_if.map(|c| c.condition);

        let option_vec = options.into_iter().map(|(label, value)| SelectOption { label, value }).collect();

        let field = FieldSchema {
            id,
            field_type: FieldType::MultiSelect,
            label,
            placeholder: None,
            default_value,
            options: Some(option_vec),
            min: None,
            max: None,
            step: None,
            rows: None,
            accept: None,
            required: required.unwrap_or(false),
            help_text,
            rules: rule_vec,
            show_if: show_if_cond,
            step_index: None,
        };

        self.schema.fields.push(field);
        Ok(PyForm { schema: self.schema.clone() })
    }

    fn checkbox(&mut self, id: String, label: String, default: Option<bool>, help_text: Option<String>,
                rules: Option<Vec<PyRule>>, show_if: Option<PyCondition>) -> PyResult<PyForm> {
        let default_value = default.map(|v| serde_json::json!(v));
        let rule_vec = rules.unwrap_or_default().into_iter().map(|r| r.rule).collect();
        let show_if_cond = show_if.map(|c| c.condition);

        let field = FieldSchema {
            id,
            field_type: FieldType::Checkbox,
            label,
            placeholder: None,
            default_value,
            options: None,
            min: None,
            max: None,
            step: None,
            rows: None,
            accept: None,
            required: false,
            help_text,
            rules: rule_vec,
            show_if: show_if_cond,
            step_index: None,
        };

        self.schema.fields.push(field);
        Ok(PyForm { schema: self.schema.clone() })
    }

    fn radio(&mut self, id: String, label: String, options: Vec<(String, String)>,
             required: Option<bool>, help_text: Option<String>, default: Option<Py<PyAny>>,
             rules: Option<Vec<PyRule>>, show_if: Option<PyCondition>) -> PyResult<PyForm> {
        let default_value = default.map(|v| python_to_json(v)).transpose()?;
        let rule_vec = rules.unwrap_or_default().into_iter().map(|r| r.rule).collect();
        let show_if_cond = show_if.map(|c| c.condition);

        let option_vec = options.into_iter().map(|(label, value)| SelectOption { label, value }).collect();

        let field = FieldSchema {
            id,
            field_type: FieldType::Radio,
            label,
            placeholder: None,
            default_value,
            options: Some(option_vec),
            min: None,
            max: None,
            step: None,
            rows: None,
            accept: None,
            required: required.unwrap_or(false),
            help_text,
            rules: rule_vec,
            show_if: show_if_cond,
            step_index: None,
        };

        self.schema.fields.push(field);
        Ok(PyForm { schema: self.schema.clone() })
    }

    fn date(&mut self, id: String, label: String, required: Option<bool>, help_text: Option<String>,
            default: Option<Py<PyAny>>, rules: Option<Vec<PyRule>>, show_if: Option<PyCondition>) -> PyResult<PyForm> {
        let default_value = default.map(|v| python_to_json(v)).transpose()?;
        let rule_vec = rules.unwrap_or_default().into_iter().map(|r| r.rule).collect();
        let show_if_cond = show_if.map(|c| c.condition);

        let field = FieldSchema {
            id,
            field_type: FieldType::Date,
            label,
            placeholder: None,
            default_value,
            options: None,
            min: None,
            max: None,
            step: None,
            rows: None,
            accept: None,
            required: required.unwrap_or(false),
            help_text,
            rules: rule_vec,
            show_if: show_if_cond,
            step_index: None,
        };

        self.schema.fields.push(field);
        Ok(PyForm { schema: self.schema.clone() })
    }

    fn range(&mut self, id: String, label: String, min: f64, max: f64, step: Option<f64>,
             default: Option<f64>, help_text: Option<String>, show_if: Option<PyCondition>) -> PyResult<PyForm> {
        let default_value = default.map(|v| serde_json::json!(v));
        let show_if_cond = show_if.map(|c| c.condition);

        let field = FieldSchema {
            id,
            field_type: FieldType::Range,
            label,
            placeholder: None,
            default_value,
            options: None,
            min: Some(min),
            max: Some(max),
            step,
            rows: None,
            accept: None,
            required: false,
            help_text,
            rules: Vec::new(),
            show_if: show_if_cond,
            step_index: None,
        };

        self.schema.fields.push(field);
        Ok(PyForm { schema: self.schema.clone() })
    }

    fn file(&mut self, id: String, label: String, accept: Option<Vec<String>>,
            required: Option<bool>, help_text: Option<String>, rules: Option<Vec<PyRule>>,
            show_if: Option<PyCondition>) -> PyResult<PyForm> {
        let rule_vec = rules.unwrap_or_default().into_iter().map(|r| r.rule).collect();
        let show_if_cond = show_if.map(|c| c.condition);

        let field = FieldSchema {
            id,
            field_type: FieldType::File,
            label,
            placeholder: None,
            default_value: None,
            options: None,
            min: None,
            max: None,
            step: None,
            rows: None,
            accept,
            required: required.unwrap_or(false),
            help_text,
            rules: rule_vec,
            show_if: show_if_cond,
            step_index: None,
        };

        self.schema.fields.push(field);
        Ok(PyForm { schema: self.schema.clone() })
    }

    fn hidden(&mut self, id: String, value: String) -> PyResult<PyForm> {
        let default_value = Some(serde_json::json!(value));

        let field = FieldSchema {
            id,
            field_type: FieldType::Hidden,
            label: String::new(),
            placeholder: None,
            default_value,
            options: None,
            min: None,
            max: None,
            step: None,
            rows: None,
            accept: None,
            required: false,
            help_text: None,
            rules: Vec::new(),
            show_if: None,
            step_index: None,
        };

        self.schema.fields.push(field);
        Ok(PyForm { schema: self.schema.clone() })
    }

    fn submit_label(&mut self, text: String) -> PyResult<PyForm> {
        self.schema.submit_label = text;
        Ok(PyForm { schema: self.schema.clone() })
    }

    fn success_message(&mut self, text: String) -> PyResult<PyForm> {
        self.schema.success_message = text;
        Ok(PyForm { schema: self.schema.clone() })
    }

    fn build(&self) -> PyResult<String> {
        Ok(formora_core::renderer::render(&self.schema))
    }
}

/// Python wrapper for FormResult
#[pyclass]
pub struct PyFormResult {
    result: FormResult,
}

#[pymethods]
impl PyFormResult {
    #[getter]
    fn form_id(&self) -> String {
        self.result.form_id.clone()
    }

    #[getter]
    fn data(&self, py: Python) -> PyResult<PyObject> {
        let val = serde_json::to_value(&self.result.data).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        json_to_python(py, &val)
    }

    #[getter]
    fn typed_data(&self, py: Python) -> PyResult<PyObject> {
        let val = serde_json::to_value(&self.result.typed_data).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        json_to_python(py, &val)
    }

    fn as_text(&self) -> String {
        self.result.as_text()
    }
}

/// Parse a formora message from Python
#[pyfunction]
fn parse_message(message: String) -> PyResult<Option<PyFormResult>> {
    match parse(&message) {
        Some(result) => Ok(Some(PyFormResult { result })),
        None => Ok(None),
    }
}

/// Check if a message is a formora message from Python
#[pyfunction]
fn is_formora(message: String) -> bool {
    is_formora_message(&message)
}

/// Helper: Convert Python value to serde_json::Value
fn python_to_json(value: Py<PyAny>) -> PyResult<serde_json::Value> {
    Python::with_gil(|py| {
        if value.is_none(py) {
            Ok(serde_json::Value::Null)
        } else if let Ok(s) = value.extract::<String>(py) {
            Ok(serde_json::Value::String(s))
        } else if let Ok(n) = value.extract::<i64>(py) {
            Ok(serde_json::Value::Number(n.into()))
        } else if let Ok(n) = value.extract::<f64>(py) {
            Ok(serde_json::json!(n))
        } else if let Ok(b) = value.extract::<bool>(py) {
            Ok(serde_json::Value::Bool(b))
        } else if let Ok(list) = value.extract::<Vec<Py<PyAny>>>(py) {
            let arr: PyResult<Vec<serde_json::Value>> = list.into_iter().map(|v| python_to_json(v)).collect();
            Ok(serde_json::Value::Array(arr?))
        } else if let Ok(dict) = value.extract::<HashMap<String, Py<PyAny>>>(py) {
            let map: PyResult<HashMap<String, serde_json::Value>> = dict
                .into_iter()
                .map(|(k, v)| Ok((k, python_to_json(v)?)))
                .collect();
            Ok(serde_json::Value::Object(map?.into_iter().collect()))
        } else {
            // Fallback: try to convert to string
            let s = value.as_ref(py).str()?.to_string();
            Ok(serde_json::Value::String(s))
        }
    })
}

/// Helper: Convert serde_json::Value to Python object
fn json_to_python(py: Python, value: &serde_json::Value) -> PyResult<PyObject> {
    match value {
        serde_json::Value::Null => Ok(py.None()),
        serde_json::Value::Bool(b) => Ok(b.into_py(py)),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.into_py(py))
            } else if let Some(f) = n.as_f64() {
                Ok(f.into_py(py))
            } else {
                Ok(n.to_string().into_py(py))
            }
        }
        serde_json::Value::String(s) => Ok(s.into_py(py)),
        serde_json::Value::Array(arr) => {
            let py_list: PyResult<Vec<PyObject>> = arr.iter().map(|v| json_to_python(py, v)).collect();
            Ok(py_list?.into_py(py))
        }
        serde_json::Value::Object(obj) => {
            let py_dict = PyDict::new(py);
            for (k, v) in obj {
                py_dict.set_item(k, json_to_python(py, v)?)?;
            }
            Ok(py_dict.into())
        }
    }
}

/// The formora Python module
#[pymodule]
fn formora(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<CssFramework>()?;
    m.add_class::<PyCssProfile>()?;
    m.add_class::<PyRule>()?;
    m.add_class::<PyCondition>()?;
    m.add_class::<PyForm>()?;
    m.add_class::<PyFormResult>()?;
    m.add_function(wrap_pyfunction!(parse_message, m)?)?;
    m.add_function(wrap_pyfunction!(is_formora, m)?)?;
    Ok(())
}
