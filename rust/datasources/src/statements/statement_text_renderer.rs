use super::condition_renderer::ConditionRenderer;
use super::i_condition_visitor::ConditionVisitor;
use super::keyword_casing::KeywordCasing;
use super::procedure_source::ProcedureSource;
use super::statement::Statement;
use super::statement_naming::StatementNaming;
use super::statement_renderer::{StatementRenderer, StatementVisitor};
use super::i_condition::ICondition;
use super::i_where::IWhere;
use std::sync::Arc;

pub type ApplyKeywords = fn(&str, &dyn Fn(&str) -> String) -> String;

pub struct StatementTextRenderer {
    statements: Arc<dyn StatementVisitor>,
    conditions: Arc<dyn ConditionVisitor>,
    naming: StatementNaming,
    apply_keywords_fn: Option<ApplyKeywords>,
}

impl StatementTextRenderer {
    pub fn new(
        statements: Option<Box<dyn StatementVisitor>>,
        conditions: Option<Box<dyn ConditionVisitor>>,
    ) -> Self {
        Self {
            statements: Arc::from(statements.unwrap_or_else(|| Box::new(StatementRenderer))),
            conditions: Arc::from(conditions.unwrap_or_else(|| Box::new(ConditionRenderer))),
            naming: StatementNaming::sql(),
            apply_keywords_fn: None,
        }
    }

    pub fn with_keyword_apply(mut self, apply: ApplyKeywords) -> Self {
        self.apply_keywords_fn = Some(apply);
        self
    }

    pub fn with_naming(&self, naming: StatementNaming) -> Self {
        Self {
            statements: self.statements.clone(),
            conditions: self.conditions.clone(),
            naming,
            apply_keywords_fn: self.apply_keywords_fn,
        }
    }

    pub fn naming(&self) -> &StatementNaming {
        &self.naming
    }

    pub fn apply_keywords(&self, sql: &str) -> String {
        if self.naming.keyword_casing == KeywordCasing::Lower {
            if let Some(apply) = self.apply_keywords_fn {
                return apply(sql, &|word| self.naming.keyword(word));
            }
        }
        sql.into()
    }

    pub fn procedures(&self, source: &ProcedureSource) -> Vec<Box<dyn Statement>> {
        self.statements.procedures(source)
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
