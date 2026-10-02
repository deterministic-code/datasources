use super::condition_renderer::ConditionRenderer;
use super::i_condition_visitor::ConditionVisitor;
use super::statement::Statement;
use super::statement_renderer::{StatementRenderer, StatementVisitor};
use crate::i_condition::ICondition;
use crate::i_where::IWhere;

pub struct StatementTextRenderer {
    statements: Box<dyn StatementVisitor>,
    conditions: Box<dyn ConditionVisitor>,
}

impl StatementTextRenderer {
    pub fn new(
        statements: Option<Box<dyn StatementVisitor>>,
        conditions: Option<Box<dyn ConditionVisitor>>,
    ) -> Self {
        Self {
            statements: statements.unwrap_or_else(|| Box::new(StatementRenderer)),
            conditions: conditions.unwrap_or_else(|| Box::new(ConditionRenderer)),
        }
    }

    pub fn render(&self, statement: &dyn Statement) -> String {
        statement.accept(self)
    }

    pub fn render_condition(&self, condition: &dyn ICondition) -> String {
        condition.accept(self)
    }

    pub fn render_where(&self, where_clause: &dyn IWhere) -> String {
        self.render_condition(where_clause)
    }

    pub(crate) fn statements(&self) -> &dyn StatementVisitor {
        &*self.statements
    }

    pub(crate) fn conditions(&self) -> &dyn ConditionVisitor {
        &*self.conditions
    }
}
