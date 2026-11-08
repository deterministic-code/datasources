use crate::schema::{self, SqlTable};
use crate::spec;
use std::collections::HashMap;

pub fn from(types: &[SqlTable]) -> HashMap<String, Vec<Vec<String>>> {
    types.iter().filter_map(of).collect()
}

fn of(r#type: &SqlTable) -> Option<(String, Vec<Vec<String>>)> {
    let overlay = schema::overlay_of(r#type);
    let pk = spec::identity_columns(Some(&r#type.as_type()), Some(&overlay))
        .into_iter()
        .next()?;
    let mut groups = vec![vec![pk.clone()]];
    let mut add = |props: Vec<String>| {
        if props.is_empty() {
            return;
        }
        if groups.iter().any(|group| group == &props) {
            return;
        }
        groups.push(props);
    };
    for name in spec::unique_lookup_fields(&r#type.as_type(), Some(&overlay)) {
        if name != pk {
            add(vec![name]);
        }
    }
    for index in r#type.indexes.iter().filter(|index| index.is_unique) {
        add(index.fields.clone());
    }
    Some((r#type.name.clone(), groups))
}
