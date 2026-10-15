use crate::schema::{self, SqlTable};
use crate::spec;
use std::collections::HashMap;

pub fn from(types: &[SqlTable]) -> HashMap<String, Vec<String>> {
    types
        .iter()
        .filter_map(|r#type| {
            let overlay = schema::overlay_of(r#type);
            let pk = spec::pk_name(&r#type.as_type(), Some(&overlay));
            let names: Vec<String> = spec::unique_lookup_fields(&r#type.as_type(), Some(&overlay))
                .into_iter()
                .filter(|name| name != &pk)
                .collect();
            if names.is_empty() {
                None
            } else {
                Some((r#type.name.clone(), names))
            }
        })
        .collect()
}
