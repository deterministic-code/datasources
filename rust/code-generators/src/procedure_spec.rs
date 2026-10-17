#[derive(Clone, Debug)]
pub struct ProcedureSpec {
    pub op: String,
    pub name: String,
    pub by_properties: Vec<String>,
}

impl ProcedureSpec {
    pub fn new(op: impl Into<String>, name: impl Into<String>, by_properties: Vec<String>) -> Self {
        Self {
            op: op.into(),
            name: name.into(),
            by_properties,
        }
    }
}
