use crate::schema::FormSchema;

pub fn render(_schema: &FormSchema) -> String {
    // Full implementation coming in Phase 2
    r#"<div data-formora-id="form"><form><!-- Form rendering in Phase 2 --></form></div>"#.to_string()
}
