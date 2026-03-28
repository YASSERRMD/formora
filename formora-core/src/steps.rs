// Multi-step logic
// Handles step assignment and organization for multi-step forms

use crate::schema::{FormSchema, StepMeta};

/// Assign step indices to fields and populate the steps vector
/// This should be called during form build before rendering
pub fn assign_step_indices(schema: &mut FormSchema, step_titles: Vec<Option<String>>) {
    if !schema.multi_step {
        return;
    }

    let num_steps = step_titles.len();
    let fields_per_step = schema.fields.len() / num_steps;
    let mut current_step = 0;
    let mut field_count = 0;

    // Clear existing steps
    schema.steps.clear();

    for (_index, field) in schema.fields.iter_mut().enumerate() {
        field.step_index = Some(current_step);

        // Move to next step if we've filled this one
        field_count += 1;
        if field_count >= fields_per_step && current_step < num_steps - 1 {
            current_step += 1;
            field_count = 0;
        }
    }

    // Populate steps metadata
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

/// Get the step index for a given field ID
pub fn get_field_step(schema: &FormSchema, field_id: &str) -> Option<usize> {
    schema.fields.iter().find(|f| f.id == field_id)?.step_index
}
