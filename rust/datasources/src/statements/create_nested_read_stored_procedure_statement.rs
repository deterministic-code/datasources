use super::expression::Expression;
use super::identifier::Identifier;
use super::nested_where_argument::NestedWhereArgument;
use super::operator::Operator;
use super::statement::Statement;
use super::statement_text_renderer::StatementTextRenderer;
use super::Where;
use super::i_where::IWhere;
use super::nested_select_statement::NestedSelectStatement;
use crate::statements::SelectStatementOptions;
use crate::SelectStatement;

pub struct CreateNestedReadStoredProcedureStatement {
    name: String,
    nested: NestedSelectStatement,
    arguments: Vec<NestedWhereArgument>,
}

impl CreateNestedReadStoredProcedureStatement {
    pub fn new(
        name: impl Into<String>,
        nested: NestedSelectStatement,
        arguments: Vec<NestedWhereArgument>,
    ) -> Self {
        Self {
            name: name.into(),
            nested,
            arguments,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn nested(&self) -> &NestedSelectStatement {
        &self.nested
    }

    pub fn arguments(&self) -> &[NestedWhereArgument] {
        &self.arguments
    }

    pub fn select(&self) -> SelectStatement {
        with_arguments(&self.nested.select, &self.arguments)
    }
}

fn with_arguments(select: &SelectStatement, arguments: &[NestedWhereArgument]) -> SelectStatement {
    let mut clause = select
        .where_clause()
        .and_then(|item| item.as_any().downcast_ref::<Where>().cloned());
    for argument in arguments {
        let equals = Operator::Equals {
            left: Box::new(Expression::Identifier(Identifier::new(&argument.path))),
            right: Box::new(Expression::Identifier(Identifier::new(
                &argument.parameter.name,
            ))),
        };
        clause = Some(match clause {
            Some(existing) => Where::new(Operator::and(existing.condition, equals)),
            None => Where::new(equals),
        });
    }
    let field_refs: Vec<&str> = select.fields.iter().map(String::as_str).collect();
    let group: Vec<&str> = select.group_by.iter().map(String::as_str).collect();
    let aliases: Vec<Option<&str>> = select.aliases.iter().map(|alias| alias.as_deref()).collect();
    SelectStatement::new(SelectStatementOptions {
        distinct: select.distinct,
        joins: &select.joins,
        group_by: &group,
        order_by: &select.order_by,
        limit: select.limit,
        offset: select.offset,
        where_clause: clause.as_ref().map(|item| item as &dyn IWhere),
        having: select.having(),
        aliases: &aliases,
        ..SelectStatementOptions::new(&select.table, &field_refs)
    })
}

impl Statement for CreateNestedReadStoredProcedureStatement {
    fn accept(&self, text: &StatementTextRenderer) -> String {
        text.statements()
            .visit_create_nested_read_stored_procedure(self, text)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl crate::ReadInput for CreateNestedReadStoredProcedureStatement {
    fn as_statement(&self) -> Option<&dyn crate::Statement> {
        Some(self)
    }
}
