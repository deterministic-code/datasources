use crate::association::Association;

#[derive(Clone)]
pub struct Enrichment {
    pub into: String,
    pub from: String,
    pub by: String,
    pub select: Option<String>,
    pub many: bool,
    pub association: Option<Association>,
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
            association: None,
        }
    }

    pub fn linked(mut self, association: Association) -> Self {
        self.many = true;
        self.select = None;
        self.association = Some(association);
        self
    }
}
