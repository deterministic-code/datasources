use super::check_constraint::CheckConstraint;
use super::foreign_key::ForeignKey;
use super::primary_key::PrimaryKey;
use super::unique_constraint::UniqueConstraint;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TableConstraint {
    PrimaryKey(PrimaryKey),
    ForeignKey(ForeignKey),
    Unique(UniqueConstraint),
    Check(CheckConstraint),
}

impl From<PrimaryKey> for TableConstraint {
    fn from(value: PrimaryKey) -> Self {
        Self::PrimaryKey(value)
    }
}

impl From<ForeignKey> for TableConstraint {
    fn from(value: ForeignKey) -> Self {
        Self::ForeignKey(value)
    }
}

impl From<UniqueConstraint> for TableConstraint {
    fn from(value: UniqueConstraint) -> Self {
        Self::Unique(value)
    }
}

impl From<CheckConstraint> for TableConstraint {
    fn from(value: CheckConstraint) -> Self {
        Self::Check(value)
    }
}
