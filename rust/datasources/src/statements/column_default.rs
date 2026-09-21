#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColumnDefault {
    pub value: String,
    pub name: Option<String>,
}

impl ColumnDefault {
    pub fn new(value: impl Into<String>, name: Option<&str>) -> Self {
        Self {
            value: value.into(),
            name: name.map(str::to_string),
        }
    }
}
