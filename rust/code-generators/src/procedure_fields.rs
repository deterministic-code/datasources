use crate::schema::{self, SqlTable};
use crate::spec::{self, SpecTypeField};
use crate::write_inclusion;

pub fn pk_of(table: &SqlTable) -> SpecTypeField {
    let overlay = schema::overlay_of(table);
    let pk_name = spec::pk_name(&table.as_type(), Some(&overlay));
    table
        .fields
        .iter()
        .find(|f| f.name == pk_name)
        .or_else(|| table.fields.iter().find(|f| f.name == "id"))
        .cloned()
        .unwrap_or_else(|| SpecTypeField {
            name: "id".into(),
            type_name: "integer".into(),
            kind: "primitive".into(),
            base: "integer".into(),
            is_array: false,
            is_map: false,
            is_nullable: false,
            is_id: false,
            size: None,
            references: None,
            reference_pair: None,
            default_value: None,
            has_default: false,
            source: None,
            original: None,
        })
}

pub fn create_params(table: &SqlTable) -> Vec<SpecTypeField> {
    let type_ = table.as_type();
    let overlay = schema::overlay_of(table);
    table
        .fields
        .iter()
        .filter(|f| write_inclusion::include_in_create(f, &type_, Some(&overlay)))
        .cloned()
        .collect()
}

pub fn update_params(table: &SqlTable) -> Vec<SpecTypeField> {
    let type_ = table.as_type();
    let overlay = schema::overlay_of(table);
    table
        .fields
        .iter()
        .filter(|f| write_inclusion::include_in_update(f, &type_, Some(&overlay)))
        .cloned()
        .collect()
}
