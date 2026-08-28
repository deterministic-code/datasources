use datasources::converters::{identity_provider_types, use_provider, ProviderTypes};

pub fn types() -> ProviderTypes {
    identity_provider_types()
}

pub fn register() {
    use_provider("text", types());
}

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::converters::{mapping, reset, Cells};
    use datasources::{Record, Value};

    #[test]
    fn registers_identity_table() {
        reset();
        register();
        let mapping = mapping::<Cells>("text", "rust").unwrap();
        assert_eq!(mapping.source, "text");
        let mut row = Record::new();
        row.insert("col".into(), Value::from("x"));
        mapping.from(&mapping.to(&row).unwrap()).unwrap();
        assert!(types().types.is_empty());
    }
}
