use crate::schema::{FormSchema, FieldSchema};
use crate::css::CssProfile;

pub struct FormBuilder {
    schema: FormSchema,
}

impl FormBuilder {
    pub fn new(form_id: String) -> Self {
        FormBuilder {
            schema: FormSchema::new(form_id),
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

    pub fn multi_step(mut self, multi_step: bool) -> Self {
        self.schema.multi_step = multi_step;
        self
    }

    pub fn add_field(mut self, field: FieldSchema) -> Self {
        self.schema.fields.push(field);
        self
    }

    pub fn build(self) -> FormSchema {
        self.schema
    }
}
