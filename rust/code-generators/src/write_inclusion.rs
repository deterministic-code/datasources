use crate::spec::{DatasourceTable, SpecType, SpecTypeField};

pub fn resolve(
    create: Option<bool>,
    update: Option<bool>,
    identity: bool,
    fixed_id: bool,
    single_generated_id: bool,
) -> (bool, bool) {
    (
        create.unwrap_or(default_create(identity, fixed_id, single_generated_id)),
        update.unwrap_or(default_update(identity, fixed_id, single_generated_id)),
    )
}

pub fn of(field: &SpecTypeField, type_: &SpecType, table: Option<&DatasourceTable>) -> (bool, bool) {
    let ids = crate::spec::identity_columns(Some(type_), table);
    let overlay = table.and_then(|table| table.fields.iter().find(|item| item.name == field.name));
    let identity = ids.iter().any(|name| name == &field.name);
    let fixed_id = overlay.and_then(|item| item.is_fixed_id) == Some(true);
    let single_generated = identity && ids.len() == 1 && !fixed_id;
    resolve(
        overlay.and_then(|item| item.include_in_create),
        overlay.and_then(|item| item.include_in_update),
        identity,
        fixed_id,
        single_generated,
    )
}

pub fn include_in_create(field: &SpecTypeField, type_: &SpecType, table: Option<&DatasourceTable>) -> bool {
    of(field, type_, table).0
}

pub fn include_in_update(field: &SpecTypeField, type_: &SpecType, table: Option<&DatasourceTable>) -> bool {
    of(field, type_, table).1
}

fn default_create(identity: bool, fixed_id: bool, single_generated_id: bool) -> bool {
    !identity || fixed_id || !single_generated_id
}

fn default_update(identity: bool, fixed_id: bool, single_generated_id: bool) -> bool {
    !identity || !fixed_id && !single_generated_id
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spec;
    use std::collections::HashMap;

    #[test]
    fn defaults_omit_a_single_generated_id() {
        assert_eq!(resolve(None, None, true, false, true), (false, false));
    }

    #[test]
    fn defaults_include_a_fixed_id_on_create() {
        assert_eq!(resolve(None, None, true, true, false), (true, false));
    }

    #[test]
    fn authored_flags_win() {
        assert_eq!(resolve(Some(false), Some(false), false, false, false), (false, false));
    }

    #[test]
    fn parser_merges_inherited_datasource_overlays() {
        let spec = spec::parse(&HashMap::from([
            (
                spec::TYPES_YAML.into(),
                r#"
types:
  - base:
      fields:
        - id:
            type: integer
            is_id: true
        - created:
            type: datetime
  - item:
      tags:
        - datasource_type
      inherits: base
      fields:
        - name:
            type: string
"#
                .into(),
            ),
            (
                spec::DATASOURCE_YAML.into(),
                r#"
includes:
  - types:
      filter: tag == "datasource_type"
types:
  - base:
      fields:
        - created:
            include_in_create: false
            include_in_update: false
"#
                .into(),
            ),
        ]))
        .unwrap();
        let item = spec.expanded_types.iter().find(|type_| type_.name == "item").unwrap();
        let created = item.fields.iter().find(|field| field.name == "created").unwrap();
        let table = spec.datasource.iter().find(|table| table.name == "item").unwrap();
        assert!(!include_in_create(created, item, Some(table)));
        assert!(!include_in_update(created, item, Some(table)));
    }
}
