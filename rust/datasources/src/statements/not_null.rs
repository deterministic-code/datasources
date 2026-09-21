#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotNull {
    pub name: Option<String>,
}

impl NotNull {
    pub fn new(name: Option<&str>) -> Self {
        Self {
            name: name.map(str::to_string),
        }
    }
}
