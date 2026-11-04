use super::i_where::IWhere;
use crate::parameter::Parameter;
use super::statement_text_renderer::StatementTextRenderer;
use super::update_statement::UpdateStatement;
use std::collections::HashMap;

pub struct UpdateStatementTextRenderer<'a> {
    text: &'a StatementTextRenderer,
    quote: fn(&str) -> String,
    parameter: fn(&str) -> String,
}

impl<'a> UpdateStatementTextRenderer<'a> {
    pub fn new(text: &'a StatementTextRenderer) -> Self {
        Self {
            text,
            quote: |name| name.to_string(),
            parameter: |name| format!("${name}"),
        }
    }

    pub fn with_quote(mut self, quote: fn(&str) -> String, parameter: fn(&str) -> String) -> Self {
        self.quote = quote;
        self.parameter = parameter;
        self
    }

    pub fn render(&self, statement: &UpdateStatement) -> String {
        self.compose(
            &format!(
                "UPDATE {} SET {}",
                self.quote(statement.table()),
                self.set(statement.fields())
            ),
            statement.where_clause(),
            statement.fields(),
        )
        .0
    }

    pub fn compose(
        &self,
        sql: &str,
        where_clause: Option<&dyn IWhere>,
        fields: &[Parameter],
    ) -> (String, Vec<Parameter>) {
        let Some(predicate) = where_clause else {
            return (sql.to_string(), fields.to_vec());
        };
        let rendered = format!("{sql} WHERE {}", self.text.render_where(predicate));
        let (head, mut tail) = match rendered.rfind(" WHERE ") {
            Some(split) => (rendered[..split].to_string(), rendered[split..].to_string()),
            None => (rendered, String::new()),
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
                    tail = rename(&tail, &self.parameter(&parameter.name), &self.parameter(&alias));
                    bound.push(Parameter::new(alias, parameter.value));
                }
                _ => bound.push(parameter),
            }
        }
        (format!("{head}{tail}"), bound)
    }

    pub fn quote(&self, name: &str) -> String {
        (self.quote)(name)
    }

    pub fn parameter(&self, name: &str) -> String {
        (self.parameter)(name)
    }

    fn set(&self, fields: &[Parameter]) -> String {
        fields
            .iter()
            .map(|field| format!("{} = {}", self.quote(&field.name), self.parameter(&field.name)))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

fn rename(text: &str, token: &str, next: &str) -> String {
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
