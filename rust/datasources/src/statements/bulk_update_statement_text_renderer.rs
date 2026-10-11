use super::bulk_update_statement::BulkUpdateStatement;
use crate::parameter::Parameter;
use crate::value::Value;
use std::collections::HashMap;

pub struct BulkUpdateStatementTextRenderer;

impl BulkUpdateStatementTextRenderer {
    pub fn render(&self, statement: &BulkUpdateStatement) -> String {
        self.render_cells(statement, cell)
    }

    pub fn render_cells(
        &self,
        statement: &BulkUpdateStatement,
        cell: fn(&str, usize, &Value) -> String,
    ) -> String {
        let assigns = statement
            .set()
            .iter()
            .map(|column| format!("{column} = data.{column}"))
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "UPDATE {} SET {assigns} FROM {} WHERE {}",
            statement.table(),
            source(statement, cell),
            on(statement)
        )
    }

    pub fn render_join(&self, statement: &BulkUpdateStatement) -> String {
        let assigns = statement
            .set()
            .iter()
            .map(|column| format!("{}.{column} = data.{column}", statement.table()))
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "UPDATE {} JOIN {} ON {} SET {assigns}",
            statement.table(),
            source(statement, cell),
            on(statement)
        )
    }
}

pub fn on(statement: &BulkUpdateStatement) -> String {
    statement
        .ids()
        .iter()
        .map(|column| format!("{}.{column} = data.{column}", statement.table()))
        .collect::<Vec<_>>()
        .join(" AND ")
}

pub fn source(
    statement: &BulkUpdateStatement,
    cell: fn(&str, usize, &Value) -> String,
) -> String {
    let columns: Vec<&String> = statement.set().iter().chain(statement.ids()).collect();
    let selects = statement
        .rows()
        .iter()
        .enumerate()
        .map(|(index, row)| {
            let by_name: HashMap<_, _> = row
                .fields
                .iter()
                .chain(row.ids.iter())
                .map(|field: &Parameter| (field.name.as_str(), &field.value))
                .collect();
            format!(
                "SELECT {}",
                columns
                    .iter()
                    .map(|column| cell(
                        column,
                        index,
                        by_name.get(column.as_str()).copied().unwrap_or(&Value::Null)
                    ))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })
        .collect::<Vec<_>>()
        .join(" UNION ALL ");
    format!("({selects}) AS data")
}

pub fn cell(column: &str, index: usize, _value: &Value) -> String {
    format!("${column}__{index} AS {column}")
}

impl Default for BulkUpdateStatementTextRenderer {
    fn default() -> Self {
        Self
    }
}
