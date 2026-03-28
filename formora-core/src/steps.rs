use crate::schema::{FormSchema, StepMeta};

pub fn assign_step_indices(schema: &mut FormSchema, step_titles: Vec<Option<String>>) {
    if !schema.multi_step {
        return;
    }

    let num_steps = step_titles.len();
    let fields_per_step = schema.fields.len() / num_steps;
    let mut current_step = 0;
    let mut field_count = 0;

    schema.steps.clear();

    for (_index, field) in schema.fields.iter_mut().enumerate() {
        field.step_index = Some(current_step);

        field_count += 1;
        if field_count >= fields_per_step && current_step < num_steps - 1 {
            current_step += 1;
            field_count = 0;
        }
    }

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
