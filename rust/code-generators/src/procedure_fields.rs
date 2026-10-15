use crate::schema::{self, SqlTable};
use crate::spec::{self, SpecTypeField};

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

pub fn writable_non_audit(table: &SqlTable) -> Vec<SpecTypeField> {
    let pk = pk_of(table);
    table
        .fields
        .iter()
        .filter(|f| f.name != pk.name && f.name != "uuid" && f.name != "created" && f.name != "updated")
        .cloned()
        .collect()
}

pub fn create_params(table: &SqlTable) -> Vec<SpecTypeField> {
    let field = |name: &str| table.fields.iter().find(|f| f.name == name).cloned();
    let mut out = Vec::new();
    if let Some(uuid) = field("uuid") {
        out.push(uuid);
    }
    out.extend(writable_non_audit(table));
    if let Some(created) = field("created") {
        out.push(created);
    }
    if let Some(updated) = field("updated") {
        out.push(updated);
    }
    out
}
