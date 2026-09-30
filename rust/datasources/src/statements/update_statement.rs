use super::statement::Statement;
use super::text::Text;
use crate::i_where::IWhere;
use crate::parameter::Parameter;
use crate::where_text::boxed;
use std::collections::HashMap;

pub struct UpdateStatement {
    text: Option<String>,
    table: Option<String>,
    fields: Vec<Parameter>,
    where_clause: Option<Box<dyn IWhere>>,
}

impl UpdateStatement {
    pub fn text(text: impl Into<String>, where_clause: Option<Box<dyn IWhere>>) -> Self {
        Self {
            text: Some(text.into()),
            table: None,
            fields: Vec::new(),
            where_clause,
        }
    }

    pub fn parameter_list(
        fields: &[Parameter],
        where_clause: Option<&dyn IWhere>,
    ) -> Vec<Parameter> {
        compose("", where_clause, fields).1
    }

    pub fn new(
        table: impl AsRef<str>,
        fields: &[Parameter],
        where_clause: Option<&dyn IWhere>,
    ) -> Self {
        Self {
            text: None,
            table: Some(table.as_ref().to_string()),
            fields: fields.to_vec(),
            where_clause: boxed(where_clause),
        }
    }

    pub fn table(&self) -> Option<&str> {
        self.table.as_deref()
    }

    pub fn fields(&self) -> &[Parameter] {
        &self.fields
    }

    pub fn where_clause(&self) -> Option<&dyn IWhere> {
        self.where_clause.as_deref()
    }

    pub fn parameters(&self) -> Vec<Parameter> {
        match self.table {
            Some(_) => compose("", self.where_clause.as_deref(), &self.fields).1,
            None => Vec::new(),
        }
    }
}

fn compose(
    sql: &str,
    where_clause: Option<&dyn IWhere>,
    fields: &[Parameter],
) -> (String, Vec<Parameter>) {
    let Some(predicate) = where_clause else {
        return (sql.to_string(), fields.to_vec());
    };
    let text = format!("{sql} WHERE {}", predicate.get_text());
    let (head, mut tail) = match text.rfind(" WHERE ") {
        Some(split) => (text[..split].to_string(), text[split..].to_string()),
        None => (text, String::new()),
    };
    let mut bound = fields.to_vec();
    let values = fields.iter().fold(HashMap::new(), |mut values, field| {
        values.insert(field.name.clone(), field.value.clone());
        values
    });
    for parameter in predicate.parameters() {
        match values.get(&parameter.name) {
            Some(current) if current != &parameter.value => {
                let alias = format!("{}__where", parameter.name);
                tail = rename(&tail, &parameter.name, &alias);
                bound.push(Parameter::new(alias, parameter.value));
            }
            _ => bound.push(parameter),
        }
    }
    (format!("{head}{tail}"), bound)
}

fn rename(text: &str, name: &str, alias: &str) -> String {
    let token = format!("${name}");
    let next = format!("${alias}");
    let mut result = String::new();
    let mut index = 0;
    while let Some(at) = text[index..].find(&token) {
        let at = index + at;
        let end = at + token.len();
        let boundary = match text[end..].chars().next() {
            None => true,
            Some(ch) => !ch.is_ascii_alphanumeric() && ch != '_',
        };
        result.push_str(&text[index..at]);
        result.push_str(if boundary { &next } else { &token });
        index = end;
    }
    result.push_str(&text[index..]);
    result
}

impl Text for UpdateStatement {
    fn get_text(&self) -> String {
        let Some(table) = &self.table else {
            return self.text.clone().unwrap_or_default();
        };
        let sql = format!(
            "UPDATE {table} SET {}",
            self.fields
                .iter()
                .map(|field| format!("{} = ${}", field.name, field.name))
                .collect::<Vec<_>>()
                .join(", ")
        );
        compose(&sql, self.where_clause.as_deref(), &self.fields).0
    }
}

impl crate::ReadInput for UpdateStatement {
    fn as_statement(&self) -> Option<&dyn crate::Statement> {
        Some(self)
    }
}

impl Statement for UpdateStatement {
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
