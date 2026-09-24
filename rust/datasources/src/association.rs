#[derive(Clone)]
pub struct Association {
    pub parent: String,
    pub related: String,
}

impl Association {
    pub fn new(parent: impl Into<String>, related: impl Into<String>) -> Self {
        Self {
            parent: parent.into(),
            related: related.into(),
        }
    }
}
