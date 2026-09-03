#[derive(Clone)]
pub struct Enrichment {
    pub into: String,
    pub from: String,
    pub by: String,
    pub select: Option<String>,
    pub many: bool,
}

impl Enrichment {
    pub fn new(
        into: impl Into<String>,
        from: impl Into<String>,
        by: impl Into<String>,
        select: Option<String>,
        many: bool,
    ) -> Self {
        Self {
            into: into.into(),
            from: from.into(),
            by: by.into(),
            select,
            many,
        }
    }
}
