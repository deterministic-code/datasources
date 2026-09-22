use datasources::{Mapper, TransformMapper};

pub fn names() -> TransformMapper<String, String> {
    TransformMapper::new(
        |name: String| match name.as_str() {
            "name" => "ugly_name".into(),
            "role_id" => "bad_role_id".into(),
            other => other.into(),
        },
        |name: String| match name.as_str() {
            "ugly_name" => "name".into(),
            "bad_role_id" => "role_id".into(),
            other => other.into(),
        },
    )
}

pub fn map(name: &str) -> String {
    Mapper::map(&names(), name.to_string())
}

pub fn reverse_map(name: &str) -> String {
    Mapper::reverse_map(&names(), name.to_string())
}
