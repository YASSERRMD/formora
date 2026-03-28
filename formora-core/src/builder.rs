use crate::schema::{FormSchema, FieldSchema};
use crate::css::CssProfile;

/// Fluent builder for constructing FormSchema
pub struct FormBuilder {
    pub schema: FormSchema,
    pub step_titles: Vec<Option<String>>,
    current_step_index: Option<usize>,
}

impl FormBuilder {
    pub fn new(form_id: String) -> Self {
        FormBuilder {
            schema: FormSchema::new(form_id),
            step_titles: Vec::new(),
            current_step_index: None,
        }
    }

    pub fn title(mut self, title: String) -> Self {
        self.schema.title = Some(title);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.schema.description = Some(description);
        self
    }

    pub fn css(mut self, framework: String, profile: CssProfile) -> Self {
        self.schema.css_framework = framework;
        self.schema.css_profile = profile;
        self
    }

    pub fn submit_label(mut self, label: String) -> Self {
        self.schema.submit_label = label;
        self
    }

    pub fn success_message(mut self, message: String) -> Self {
        self.schema.success_message = message;
        self
    }

    pub fn step(mut self, title: Option<String>) -> Self {
        self.schema.multi_step = true;
        self.step_titles.push(title);
        self.current_step_index = Some(self.step_titles.len() - 1);
        self
    }

    pub fn add_field(mut self, mut field: FieldSchema) -> Self {
        if self.schema.multi_step && self.current_step_index.is_some() {
            field.step_index = self.current_step_index;
        } else if self.schema.multi_step {
            // Optional fallback if add_field is called before step()
            field.step_index = Some(0);
        }
        self.schema.fields.push(field);
        self
    }

    pub fn build(mut self) -> FormSchema {
        if self.schema.multi_step {
            crate::steps::assign_step_indices(&mut self.schema, self.step_titles.clone());
        }
        self.schema
    }
}
