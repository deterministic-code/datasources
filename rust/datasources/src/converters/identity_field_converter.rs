use crate::converters::field_converter::FieldConverter;

pub fn identity_field_converter() -> FieldConverter {
    FieldConverter::new(|value| Ok(value.clone()), |value| Ok(value.clone()))
}
