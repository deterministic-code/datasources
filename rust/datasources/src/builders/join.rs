#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinKind {
    Inner,
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Join {
    pub kind: JoinKind,
    pub table: String,
    pub left: String,
    pub right: String,
}

impl Join {
    pub fn inner(
        table: impl Into<String>,
        left: impl Into<String>,
        right: impl Into<String>,
    ) -> Self {
        Self::new(JoinKind::Inner, table, left, right)
    }

    pub fn left(
        table: impl Into<String>,
        left: impl Into<String>,
        right: impl Into<String>,
    ) -> Self {
        Self::new(JoinKind::Left, table, left, right)
    }

    pub fn right(
        table: impl Into<String>,
        left: impl Into<String>,
        right: impl Into<String>,
    ) -> Self {
        Self::new(JoinKind::Right, table, left, right)
    }

    fn new(
        kind: JoinKind,
        table: impl Into<String>,
        left: impl Into<String>,
        right: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            table: table.into(),
            left: left.into(),
            right: right.into(),
        }
    }
}
