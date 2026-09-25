#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoinKind {
    Inner,
    Left,
    Right,
    Full,
    Cross,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Join {
    pub kind: JoinKind,
    pub table: String,
    pub on: Option<String>,
}

impl Join {
    pub fn inner(
        table: impl Into<String>,
        left: impl Into<String>,
        right: impl Into<String>,
    ) -> Self {
        Self::equality(JoinKind::Inner, table, left, right)
    }

    pub fn left(
        table: impl Into<String>,
        left: impl Into<String>,
        right: impl Into<String>,
    ) -> Self {
        Self::equality(JoinKind::Left, table, left, right)
    }

    pub fn right(
        table: impl Into<String>,
        left: impl Into<String>,
        right: impl Into<String>,
    ) -> Self {
        Self::equality(JoinKind::Right, table, left, right)
    }

    pub fn full(
        table: impl Into<String>,
        left: impl Into<String>,
        right: impl Into<String>,
    ) -> Self {
        Self::equality(JoinKind::Full, table, left, right)
    }

    pub fn cross(table: impl Into<String>) -> Self {
        Self {
            kind: JoinKind::Cross,
            table: table.into(),
            on: None,
        }
    }

    pub fn condition(kind: JoinKind, table: impl Into<String>, on: impl Into<String>) -> Self {
        Self {
            kind,
            table: table.into(),
            on: Some(on.into()),
        }
    }

    fn equality(
        kind: JoinKind,
        table: impl Into<String>,
        left: impl Into<String>,
        right: impl Into<String>,
    ) -> Self {
        let left = left.into();
        let right = right.into();
        Self {
            kind,
            table: table.into(),
            on: Some(format!("{left} = {right}")),
        }
    }
}
