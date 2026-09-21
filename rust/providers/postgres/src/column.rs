pub struct PostgresColumn {
    pub name: String,
    pub type_name: String,
    pub id: bool,
}

impl PostgresColumn {
    pub fn new(name: impl Into<String>, type_name: impl Into<String>, id: bool) -> Self {
        Self {
            name: name.into(),
            type_name: type_name.into(),
            id,
        }
    }
}
