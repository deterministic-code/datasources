use super::statement::Statement;
use super::text::Text;
use crate::parameter::Parameter;
use crate::where_trait::Where;
use std::collections::HashMap;

pub struct UpdateStatement {
    text: String,
    where_clause: Option<Box<dyn Where>>,
    parameters: Vec<Parameter>,
}

impl UpdateStatement {
    pub fn text(text: impl Into<String>, where_clause: Option<Box<dyn Where>>) -> Self {
        Self::bound(text, where_clause, Vec::new())
    }

    pub fn bound(
        text: impl Into<String>,
        where_clause: Option<Box<dyn Where>>,
        parameters: Vec<Parameter>,
    ) -> Self {
        Self {
            text: text.into(),
            where_clause,
            parameters,
        }
    }

    pub fn new(
        table: impl AsRef<str>,
        fields: &[Parameter],
        where_clause: Option<&dyn Where>,
    ) -> Self {
        let sql = format!(
            "UPDATE {} SET {}",
            table.as_ref(),
            fields
                .iter()
                .map(|field| format!("{} = ${}", field.name, field.name))
                .collect::<Vec<_>>()
                .join(", ")
        );
        let (text, parameters) = bind(&sql, where_clause, fields);
        Self {
            text,
            parameters,
            where_clause: where_clause.map(|predicate| {
                Box::new(crate::text_where::TextWhere::new(predicate.get_text())) as Box<dyn Where>
            }),
        }
    }

    pub fn where_clause(&self) -> Option<&dyn Where> {
        self.where_clause.as_deref()
    }
}

fn bind(
    sql: &str,
    where_clause: Option<&dyn Where>,
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
        self.text.clone()
    }
}

impl crate::ReadInput for UpdateStatement {
    fn as_statement(&self) -> Option<&dyn crate::Statement> {
        Some(self)
    }
}

impl Statement for UpdateStatement {
    fn parameters(&self) -> Vec<Parameter> {
        self.parameters.clone()
    }
    fn kind(&self) -> super::statement::StatementKind {
        super::statement::StatementKind::Update
    }
    fn predicate(&self) -> Option<&dyn Where> {
        self.where_clause.as_deref()
    }
}
