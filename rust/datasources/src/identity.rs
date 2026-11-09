use crate::schema::Schema;

pub fn ids_from_schema(schema: &Schema) -> Vec<String> {
    schema
        .columns
        .iter()
        .filter(|(_, column)| column.is_id)
        .map(|(name, _)| name.clone())
        .collect()
}
