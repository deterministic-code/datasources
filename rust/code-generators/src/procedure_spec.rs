#[derive(Clone, Debug)]
pub struct ProcedureSpec {
    pub op: String,
    pub name: String,
    pub by_field: Option<String>,
}

impl ProcedureSpec {
    pub fn new(op: impl Into<String>, name: impl Into<String>, by_field: Option<String>) -> Self {
        Self {
            op: op.into(),
            name: name.into(),
            by_field,
        }
    }
}
