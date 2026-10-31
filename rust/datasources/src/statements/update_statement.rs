use super::default_statement_text_renderer::DefaultStatementTextRenderer;
use super::statement::Statement;
use super::statement_text_renderer::StatementTextRenderer;
use super::update_statement_text_renderer::UpdateStatementTextRenderer;
use super::i_where::IWhere;
use crate::parameter::Parameter;
use super::where_text::boxed;

pub struct UpdateStatement {
    table: String,
    fields: Vec<Parameter>,
    where_clause: Option<Box<dyn IWhere>>,
}

impl UpdateStatement {
    pub fn parameter_list(
        fields: &[Parameter],
        where_clause: Option<&dyn IWhere>,
    ) -> Vec<Parameter> {
        UpdateStatementTextRenderer::new(DefaultStatementTextRenderer::instance())
            .compose("", where_clause, fields)
            .1
    }

    pub fn new(
        table: impl AsRef<str>,
        fields: &[Parameter],
        where_clause: Option<&dyn IWhere>,
    ) -> Self {
        Self {
            table: table.as_ref().to_string(),
            fields: fields.to_vec(),
            where_clause: boxed(where_clause),
        }
    }

    pub fn table(&self) -> &str {
        &self.table
    }

    pub fn fields(&self) -> &[Parameter] {
        &self.fields
    }

    pub fn where_clause(&self) -> Option<&dyn IWhere> {
        self.where_clause.as_deref()
    }

    pub fn parameters(&self) -> Vec<Parameter> {
        UpdateStatementTextRenderer::new(DefaultStatementTextRenderer::instance())
            .compose(
                &format!(
                    "UPDATE {} SET {}",
                    self.table,
                    self.fields
                        .iter()
                        .map(|field| format!("{} = ${}", field.name, field.name))
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
                self.where_clause.as_deref(),
                &self.fields,
            )
            .1
    }
}

impl crate::ReadInput for UpdateStatement {
    fn as_statement(&self) -> Option<&dyn crate::Statement> {
        Some(self)
    }
}

impl Statement for UpdateStatement {
    fn accept(&self, text: &StatementTextRenderer) -> String {
        text.statements().visit_update(self, text)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn kind(&self) -> super::statement::StatementKind {
        super::statement::StatementKind::Update
    }
    fn predicate(&self) -> Option<&dyn IWhere> {
        self.where_clause.as_deref()
    }
}
