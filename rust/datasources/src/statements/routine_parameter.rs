use crate::converters::SpecField;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RoutineParameter {
    pub name: String,
    pub r#type: String,
    pub field: Option<SpecField>,
}

impl RoutineParameter {
    pub fn new(name: impl Into<String>, r#type: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            r#type: r#type.into(),
            field: None,
        }
    }

    pub fn with_field(self, field: SpecField) -> Self {
        Self {
            field: Some(field),
            ..self
        }
    }
}
