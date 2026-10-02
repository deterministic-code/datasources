use super::expression::Expression;
use super::operator::Operator;
use super::statement_text_renderer::StatementTextRenderer;
use crate::value::Value;

pub struct ExpressionRenderer;

impl ExpressionRenderer {
    pub fn render(&self, expression: &Expression, text: &StatementTextRenderer) -> String {
        match expression {
            Expression::Identifier(id) => id.name.clone(),
            Expression::Literal(lit) => inline(&lit.value),
            Expression::Subquery(query) => format!("({query})"),
            Expression::Array(arr) => arr
                .elements
                .iter()
                .map(|element| self.render(element, text))
                .collect::<Vec<_>>()
                .join(", "),
            Expression::Operator(Operator::Not { argument }) => {
                format!("NOT ({})", self.render(argument, text))
            }
            Expression::Operator(Operator::IsNull { argument }) => {
                format!("{} IS NULL", self.render(argument, text))
            }
            Expression::Operator(Operator::IsNotNull { argument }) => {
                format!("{} IS NOT NULL", self.render(argument, text))
            }
            Expression::Operator(Operator::Exists { argument }) => {
                format!("EXISTS {}", self.render(argument, text))
            }
            Expression::Operator(op @ Operator::In { left, right }) => match (&**left, &**right) {
                (Expression::Identifier(_), Expression::Array(arr)) if arr.elements.is_empty() => {
                    "1 = 0".into()
                }
                (Expression::Identifier(id), Expression::Array(arr)) => {
                    let list = (0..arr.elements.len())
                        .map(|i| format!("${}_{i}", parameter_name(&id.name)))
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("{} IN ({list})", id.name)
                }
                (Expression::Identifier(id), Expression::Subquery(_)) => {
                    format!("{} IN {}", id.name, self.render(right, text))
                }
                _ => self.compare_text(op, left, right, text),
            },
            Expression::Operator(Operator::NotIn { left, right }) => match (&**left, &**right) {
                (Expression::Identifier(_), Expression::Array(arr)) if arr.elements.is_empty() => {
                    "1 = 1".into()
                }
                (Expression::Identifier(id), Expression::Array(arr)) => {
                    let list = (0..arr.elements.len())
                        .map(|i| format!("${}_{i}", parameter_name(&id.name)))
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("{} NOT IN ({list})", id.name)
                }
                (Expression::Identifier(id), Expression::Subquery(_)) => {
                    format!("{} NOT IN {}", id.name, self.render(right, text))
                }
                _ => format!(
                    "{} NOT IN {}",
                    self.render(left, text),
                    self.render(right, text)
                ),
            },
            Expression::Operator(Operator::Between { value, low, high }) => {
                self.bounds("BETWEEN", value, low, high, text)
            }
            Expression::Operator(Operator::NotBetween { value, low, high }) => {
                self.bounds("NOT BETWEEN", value, low, high, text)
            }
            Expression::Operator(Operator::And { left, right }) => {
                format!(
                    "{} AND {}",
                    self.wrap(left, true, text),
                    self.wrap(right, true, text)
                )
            }
            Expression::Operator(Operator::Or { left, right }) => {
                format!(
                    "{} OR {}",
                    self.wrap(left, false, text),
                    self.wrap(right, false, text)
                )
            }
            Expression::Operator(
                op @ (Operator::Equals { left, right }
                | Operator::NotEquals { left, right }
                | Operator::GreaterThan { left, right }
                | Operator::LessThan { left, right }
                | Operator::GreaterThanOrEqual { left, right }
                | Operator::LessThanOrEqual { left, right }
                | Operator::Like { left, right }
                | Operator::NotLike { left, right }),
            ) => self.compare_text(op, left, right, text),
        }
    }

    fn wrap(&self, child: &Expression, and: bool, text: &StatementTextRenderer) -> String {
        if needs_parens(child, and) {
            format!("({})", self.render(child, text))
        } else {
            self.render(child, text)
        }
    }

    fn compare_text(
        &self,
        op: &Operator,
        left: &Expression,
        right: &Expression,
        text: &StatementTextRenderer,
    ) -> String {
        match (left, right) {
            (Expression::Identifier(id), Expression::Literal(_)) => {
                format!("{} {} ${}", id.name, symbol(op), parameter_name(&id.name))
            }
            (Expression::Literal(left), Expression::Literal(right)) => {
                format!(
                    "{} {} {}",
                    inline(&left.value),
                    symbol(op),
                    inline(&right.value)
                )
            }
            (left, right) => format!(
                "{} {} {}",
                self.render(left, text),
                symbol(op),
                self.render(right, text)
            ),
        }
    }

    fn bounds(
        &self,
        word: &str,
        value: &Expression,
        low: &Expression,
        high: &Expression,
        text: &StatementTextRenderer,
    ) -> String {
        match (value, low, high) {
            (Expression::Identifier(id), Expression::Literal(_), Expression::Literal(_)) => {
                let name = parameter_name(&id.name);
                format!("{} {word} ${name}_0 AND ${name}_1", id.name)
            }
            _ => format!(
                "{} {word} {} AND {}",
                self.render(value, text),
                self.render(low, text),
                self.render(high, text)
            ),
        }
    }
}

fn symbol(op: &Operator) -> &'static str {
    match op {
        Operator::Equals { .. } => "=",
        Operator::NotEquals { .. } => "<>",
        Operator::GreaterThan { .. } => ">",
        Operator::LessThan { .. } => "<",
        Operator::GreaterThanOrEqual { .. } => ">=",
        Operator::LessThanOrEqual { .. } => "<=",
        Operator::Like { .. } => "LIKE",
        Operator::NotLike { .. } => "NOT LIKE",
        _ => "IN",
    }
}

fn inline(value: &Value) -> String {
    match value {
        Value::Int(number) => number.to_string(),
        Value::Float(number) => number.to_string(),
        Value::Decimal(number) => number.to_string(),
        Value::Bool(flag) => flag.to_string(),
        Value::Text(text) => format!("'{}'", text.replace('\'', "''")),
        Value::Null => "null".into(),
        other => other.wire_text(),
    }
}

fn needs_parens(child: &Expression, and: bool) -> bool {
    match child {
        Expression::Operator(
            Operator::Not { .. } | Operator::In { .. } | Operator::NotIn { .. },
        ) => true,
        Expression::Operator(Operator::And { .. }) if !and => true,
        Expression::Operator(Operator::Or { .. }) if and => true,
        _ => false,
    }
}

pub(crate) fn parameter_name(name: &str) -> String {
    parts(name)
        .into_iter()
        .map(|part| {
            if part.len() >= 2 && part.starts_with('"') && part.ends_with('"') {
                part[1..part.len() - 1].replace("\"\"", "\"")
            } else {
                part
            }
        })
        .collect::<Vec<_>>()
        .join(".")
}

fn parts(name: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut start = 0;
    let mut quoted = false;
    let chars: Vec<char> = name.chars().collect();
    for (i, ch) in chars.iter().enumerate() {
        if *ch == '"' {
            quoted = !quoted;
        }
        if *ch == '.' && !quoted {
            found.push(chars[start..i].iter().collect());
            start = i + 1;
        }
    }
    found.push(chars[start..].iter().collect());
    found
}
