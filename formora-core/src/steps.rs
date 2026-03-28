use crate::schema::{FormSchema, StepMeta};

/// Assign step indices to fields and populate the steps vector
pub fn assign_step_indices(schema: &mut FormSchema, step_titles: Vec<Option<String>>) {
    if !schema.multi_step {
        return;
    }

    schema.steps.clear();

    for (idx, title) in step_titles.into_iter().enumerate() {
        let field_ids: Vec<String> = schema
            .fields
            .iter()
            .filter(|f| f.step_index == Some(idx))
            .map(|f| f.id.clone())
            .collect();

        schema.steps.push(StepMeta {
            index: idx,
            title,
            field_ids,
        });
    }
}

pub fn get_field_step(schema: &FormSchema, field_id: &str) -> Option<usize> {
    schema.fields.iter().find(|f| f.id == field_id)?.step_index
}
