use super::statement::Statement;
use crate::where_trait::Where;

pub trait InsertStatement: Statement {}
pub trait UpdateStatement: Statement {
    fn where_clause(&self) -> Option<&dyn Where>;
}
pub trait DeleteStatement: Statement {
    fn where_clause(&self) -> Option<&dyn Where>;
}
pub trait CreateTableStatement: Statement {}
pub trait StoredProcedureStatement: Statement {}

pub struct TextInsertStatement {
    text: String,
}

impl TextInsertStatement {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl Statement for TextInsertStatement {
    fn get_text(&self) -> String {
        self.text.clone()
    }
}

impl InsertStatement for TextInsertStatement {}

pub struct TextUpdateStatement {
    text: String,
    where_clause: Option<Box<dyn Where>>,
}

impl TextUpdateStatement {
    pub fn new(text: impl Into<String>, where_clause: Option<Box<dyn Where>>) -> Self {
        Self {
            text: text.into(),
            where_clause,
        }
    }
}

impl Statement for TextUpdateStatement {
    fn get_text(&self) -> String {
        self.text.clone()
    }
}

impl UpdateStatement for TextUpdateStatement {
    fn where_clause(&self) -> Option<&dyn Where> {
        self.where_clause.as_deref()
    }
}

pub struct TextDeleteStatement {
    text: String,
    where_clause: Option<Box<dyn Where>>,
}

impl TextDeleteStatement {
    pub fn new(text: impl Into<String>, where_clause: Option<Box<dyn Where>>) -> Self {
        Self {
            text: text.into(),
            where_clause,
        }
    }
}

impl Statement for TextDeleteStatement {
    fn get_text(&self) -> String {
        self.text.clone()
    }
}

impl DeleteStatement for TextDeleteStatement {
    fn where_clause(&self) -> Option<&dyn Where> {
        self.where_clause.as_deref()
    }
}

pub struct TextCreateTableStatement {
    text: String,
}

impl TextCreateTableStatement {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl Statement for TextCreateTableStatement {
    fn get_text(&self) -> String {
        self.text.clone()
    }
}

impl CreateTableStatement for TextCreateTableStatement {}

pub struct TextStoredProcedureStatement {
    text: String,
}

impl TextStoredProcedureStatement {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl Statement for TextStoredProcedureStatement {
    fn get_text(&self) -> String {
        self.text.clone()
    }
}

impl StoredProcedureStatement for TextStoredProcedureStatement {}
