use super::statement::Statement;
use super::statement_text_renderer::StatementTextRenderer;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoutineKind {
    Create,
    FindOne,
    FindAll,
    FindBy,
    Update,
    Delete,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RoutineParameter {
    pub name: String,
    pub r#type: String,
}

pub struct CreateRoutineStatement {
    name: String,
    kind: RoutineKind,
    table: String,
    columns: Vec<String>,
    parameters: Vec<RoutineParameter>,
    key: String,
    key_type: String,
    writable: Vec<String>,
    by_field: Option<String>,
    generated_key: bool,
    audit_updated: Option<String>,
}

impl CreateRoutineStatement {
    pub fn new(
        name: impl AsRef<str>,
        kind: RoutineKind,
        table: impl AsRef<str>,
        columns: Vec<String>,
        parameters: Vec<RoutineParameter>,
        key: impl AsRef<str>,
        key_type: impl AsRef<str>,
        writable: Vec<String>,
        by_field: Option<String>,
        generated_key: bool,
        audit_updated: Option<String>,
    ) -> Self {
        Self {
            name: name.as_ref().to_string(),
            kind,
            table: table.as_ref().to_string(),
            columns,
            parameters,
            key: key.as_ref().to_string(),
            key_type: key_type.as_ref().to_string(),
            writable,
            by_field,
            generated_key,
            audit_updated,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn kind(&self) -> RoutineKind {
        self.kind
    }

    pub fn table(&self) -> &str {
        &self.table
    }

    pub fn columns(&self) -> &[String] {
        &self.columns
    }

    pub fn parameters(&self) -> &[RoutineParameter] {
        &self.parameters
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn key_type(&self) -> &str {
        &self.key_type
    }

    pub fn writable(&self) -> &[String] {
        &self.writable
    }

    pub fn by_field(&self) -> Option<&str> {
        self.by_field.as_deref()
    }

    pub fn generated_key(&self) -> bool {
        self.generated_key
    }

    pub fn audit_updated(&self) -> Option<&str> {
        self.audit_updated.as_deref()
    }
}

impl Statement for CreateRoutineStatement {
    fn accept(&self, text: &StatementTextRenderer) -> String {
        text.statements().visit_create_routine(self, text)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl crate::ReadInput for CreateRoutineStatement {
    fn as_statement(&self) -> Option<&dyn crate::Statement> {
        Some(self)
    }
}
