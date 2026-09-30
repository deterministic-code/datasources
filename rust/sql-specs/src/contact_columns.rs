use datasources::{Casing, ChangeCasingMapper, DictionaryMapper, Mapper, TransformMapper};
use std::sync::Arc;

pub fn snake() -> ChangeCasingMapper {
    ChangeCasingMapper::new(Casing::Pascal, Casing::Snake)
}

pub fn columns() -> TransformMapper<String, String> {
    let references = Arc::new(DictionaryMapper::new([
        ("Contact".to_string(), "contact_id".to_string()),
        ("AddressType".to_string(), "address_type_id".to_string()),
        ("ContactType".to_string(), "contact_type_id".to_string()),
        ("ContactSource".to_string(), "contact_source_id".to_string()),
        ("PhoneType".to_string(), "phone_type_id".to_string()),
        ("Line1".to_string(), "line1".to_string()),
        ("Line2".to_string(), "line2".to_string()),
    ]));
    let snake = Arc::new(snake());
    let forward_references = references.clone();
    let forward_snake = snake.clone();
    let reverse_references = references.clone();
    let reverse_snake = snake;
    TransformMapper::new(
        move |name: String| {
            let mapped = Mapper::map(forward_references.as_ref(), name.clone());
            if mapped != name {
                mapped
            } else {
                Mapper::map(forward_snake.as_ref(), name)
            }
        },
        move |name: String| {
            let from = Mapper::reverse_map(reverse_references.as_ref(), name.clone());
            if from != name {
                from
            } else {
                Mapper::reverse_map(reverse_snake.as_ref(), name)
            }
        },
    )
}

pub fn contact_entity() -> DictionaryMapper<String> {
    DictionaryMapper::new([("Contact".to_string(), "contacts_base".to_string())])
}

pub fn references() -> Vec<String> {
    ["Contact", "AddressType", "ContactType", "ContactSource", "PhoneType"]
        .into_iter()
        .map(str::to_string)
        .collect()
}
