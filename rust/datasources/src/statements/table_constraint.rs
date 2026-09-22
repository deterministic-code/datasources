use super::check_constraint::CheckConstraint;
use super::foreign_key::{ForeignKey, ReferentialAction};
use super::primary_key::PrimaryKey;
use super::unique_constraint::UniqueConstraint;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TableConstraint {
    PrimaryKey(PrimaryKey),
    ForeignKey(ForeignKey),
    Unique(UniqueConstraint),
    Check(CheckConstraint),
}

impl PrimaryKey {
    pub fn new(name: impl Into<String>, columns: &[&str]) -> TableConstraint {
        TableConstraint::PrimaryKey(Self {
            name: name.into(),
            columns: columns.iter().map(|column| (*column).to_string()).collect(),
        })
    }
}

impl ForeignKey {
    pub fn new(
        name: impl Into<String>,
        columns: &[&str],
        references_table: impl Into<String>,
        references_columns: &[&str],
        on_delete: Option<ReferentialAction>,
        on_update: Option<ReferentialAction>,
    ) -> TableConstraint {
        TableConstraint::ForeignKey(Self {
            name: name.into(),
            columns: columns.iter().map(|column| (*column).to_string()).collect(),
            references_table: references_table.into(),
            references_columns: references_columns
                .iter()
                .map(|column| (*column).to_string())
                .collect(),
            on_delete,
            on_update,
        })
    }
}

impl UniqueConstraint {
    pub fn new(name: impl Into<String>, columns: &[&str]) -> TableConstraint {
        TableConstraint::Unique(Self {
            name: name.into(),
            columns: columns.iter().map(|column| (*column).to_string()).collect(),
        })
    }
}

impl CheckConstraint {
    pub fn new(name: impl Into<String>, expression: impl Into<String>) -> TableConstraint {
        TableConstraint::Check(Self {
            name: name.into(),
            expression: expression.into(),
        })
    }
}
