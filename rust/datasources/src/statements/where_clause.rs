use super::array_expression::ArrayExpression;
use super::expression::Expression;
use super::identifier::Identifier;
use super::literal::Literal;
use super::operator::Operator;
use super::text::Text;
use crate::parameter::Parameter;
use crate::result::Result;
use crate::value::Value;
use crate::where_trait::Where;

#[derive(Clone)]
pub struct WhereClause {
    pub condition: Expression,
}

impl WhereClause {
    pub fn new(condition: impl Into<Expression>) -> Self {
        Self {
            condition: condition.into(),
        }
    }

    pub fn binary(column: &str, op: fn(&str, Value) -> Operator, value: impl Into<Value>) -> Self {
        Self::new(op(column, value.into()))
    }

    pub fn and(
        self,
        column: &str,
        op: fn(&str, Value) -> Operator,
        value: impl Into<Value>,
    ) -> Self {
        Self::new(Operator::and(self.condition, op(column, value.into())))
    }

    pub fn or(
        self,
        column: &str,
        op: fn(&str, Value) -> Operator,
        value: impl Into<Value>,
    ) -> Self {
        Self::new(Operator::or(self.condition, op(column, value.into())))
    }

    pub fn map<C, V>(&self, column: C, value: V) -> Result<Self>
    where
        C: Fn(&str) -> String,
        V: Fn(&str, &Value) -> Result<Value>,
    {
        Ok(Self::new(rewrite(&self.condition, &column, &value)?))
    }
}

impl Text for WhereClause {
    fn get_text(&self) -> String {
        render(&self.condition)
    }
}

impl crate::ReadInput for WhereClause {
    fn as_where(&self) -> Option<&dyn Where> {
        Some(self)
    }
}

impl Where for WhereClause {
    fn parameters(&self) -> Vec<Parameter> {
        collect(&self.condition)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl From<WhereClause> for Expression {
    fn from(where_clause: WhereClause) -> Self {
        where_clause.condition
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

fn wrap(child: &Expression, and: bool) -> String {
    if needs_parens(child, and) {
        format!("({})", render(child))
    } else {
        render(child)
    }
}

fn compare_text(op: &Operator, left: &Expression, right: &Expression) -> String {
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
        (left, right) => format!("{} {} {}", render(left), symbol(op), render(right)),
    }
}

fn compare_parameters(left: &Expression, right: &Expression) -> Vec<Parameter> {
    match (left, right) {
        (Expression::Identifier(id), Expression::Literal(lit)) => {
            vec![Parameter::new(parameter_name(&id.name), lit.value.clone())]
        }
        (Expression::Literal(_), Expression::Literal(_)) => vec![],
        (left, right) => {
            let mut parameters = collect(left);
            parameters.extend(collect(right));
            parameters
        }
    }
}

fn render(expression: &Expression) -> String {
    match expression {
        Expression::Identifier(id) => id.name.clone(),
        Expression::Literal(lit) => inline(&lit.value),
        Expression::Subquery(query) => format!("({query})"),
        Expression::Array(arr) => arr
            .elements
            .iter()
            .map(render)
            .collect::<Vec<_>>()
            .join(", "),
        Expression::Operator(Operator::Not { argument }) => {
            format!("NOT ({})", render(argument))
        }
        Expression::Operator(Operator::IsNull { argument }) => {
            format!("{} IS NULL", render(argument))
        }
        Expression::Operator(Operator::IsNotNull { argument }) => {
            format!("{} IS NOT NULL", render(argument))
        }
        Expression::Operator(Operator::Exists { argument }) => {
            format!("EXISTS {}", render(argument))
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
                format!("{} IN {}", id.name, render(right))
            }
            _ => compare_text(op, left, right),
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
                format!("{} NOT IN {}", id.name, render(right))
            }
            _ => format!("{} NOT IN {}", render(left), render(right)),
        },
        Expression::Operator(Operator::Between { value, low, high }) => {
            bounds("BETWEEN", value, low, high)
        }
        Expression::Operator(Operator::NotBetween { value, low, high }) => {
            bounds("NOT BETWEEN", value, low, high)
        }
        Expression::Operator(Operator::And { left, right }) => {
            format!("{} AND {}", wrap(left, true), wrap(right, true))
        }
        Expression::Operator(Operator::Or { left, right }) => {
            format!("{} OR {}", wrap(left, false), wrap(right, false))
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
        ) => compare_text(op, left, right),
    }
}

fn bounds(word: &str, value: &Expression, low: &Expression, high: &Expression) -> String {
    match (value, low, high) {
        (Expression::Identifier(id), Expression::Literal(_), Expression::Literal(_)) => {
            let name = parameter_name(&id.name);
            format!("{} {word} ${name}_0 AND ${name}_1", id.name)
        }
        _ => format!(
            "{} {word} {} AND {}",
            render(value),
            render(low),
            render(high)
        ),
    }
}

fn parameter_name(name: &str) -> String {
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

fn collect(expression: &Expression) -> Vec<Parameter> {
    match expression {
        Expression::Literal(_) | Expression::Identifier(_) | Expression::Subquery(_) => vec![],
        Expression::Operator(
            Operator::Not { argument }
            | Operator::IsNull { argument }
            | Operator::IsNotNull { argument }
            | Operator::Exists { argument },
        ) => collect(argument),
        Expression::Operator(Operator::In { left, right } | Operator::NotIn { left, right }) => {
            match (left.as_ref(), right.as_ref()) {
                (Expression::Identifier(id), Expression::Array(arr)) => arr
                    .elements
                    .iter()
                    .enumerate()
                    .map(|(i, element)| match element {
                        Expression::Literal(lit) => Parameter::new(
                            format!("{}_{i}", parameter_name(&id.name)),
                            lit.value.clone(),
                        ),
                        _ => panic!("empty in"),
                    })
                    .collect(),
                (left, right) => compare_parameters(left, right),
            }
        }
        Expression::Operator(
            Operator::Between { value, low, high } | Operator::NotBetween { value, low, high },
        ) => match (value.as_ref(), low.as_ref(), high.as_ref()) {
            (Expression::Identifier(id), Expression::Literal(low), Expression::Literal(high)) => {
                let name = parameter_name(&id.name);
                vec![
                    Parameter::new(format!("{name}_0"), low.value.clone()),
                    Parameter::new(format!("{name}_1"), high.value.clone()),
                ]
            }
            _ => {
                let mut parameters = collect(value);
                parameters.extend(collect(low));
                parameters.extend(collect(high));
                parameters
            }
        },
        Expression::Operator(Operator::And { left, right } | Operator::Or { left, right }) => {
            compare_parameters(left, right)
        }
        Expression::Operator(
            Operator::Equals { left, right }
            | Operator::NotEquals { left, right }
            | Operator::GreaterThan { left, right }
            | Operator::LessThan { left, right }
            | Operator::GreaterThanOrEqual { left, right }
            | Operator::LessThanOrEqual { left, right }
            | Operator::Like { left, right }
            | Operator::NotLike { left, right },
        ) => compare_parameters(left, right),
        Expression::Array(arr) => arr.elements.iter().flat_map(collect).collect(),
    }
}

fn rewrite<C, V>(expression: &Expression, column: &C, value: &V) -> Result<Expression>
where
    C: Fn(&str) -> String,
    V: Fn(&str, &Value) -> Result<Value>,
{
    Ok(match expression {
        Expression::Identifier(id) => Expression::Identifier(Identifier::new(column(&id.name))),
        Expression::Literal(lit) => Expression::Literal(lit.clone()),
        Expression::Subquery(query) => Expression::Subquery(query.clone()),
        Expression::Array(arr) => Expression::Array(ArrayExpression::new(
            arr.elements
                .iter()
                .map(|element| rewrite(element, column, value))
                .collect::<Result<_>>()?,
        )),
        Expression::Operator(op) => Expression::Operator(rewrite_op(op, column, value)?),
    })
}

fn rewrite_op<C, V>(op: &Operator, column: &C, value: &V) -> Result<Operator>
where
    C: Fn(&str) -> String,
    V: Fn(&str, &Value) -> Result<Value>,
{
    Ok(match op {
        Operator::And { left, right } => Operator::and(
            rewrite(left, column, value)?,
            rewrite(right, column, value)?,
        ),
        Operator::Or { left, right } => Operator::or(
            rewrite(left, column, value)?,
            rewrite(right, column, value)?,
        ),
        Operator::Not { argument } => Operator::not(rewrite(argument, column, value)?),
        Operator::IsNull { argument } => Operator::is_null(rewrite(argument, column, value)?),
        Operator::IsNotNull { argument } => {
            Operator::is_not_null(rewrite(argument, column, value)?)
        }
        Operator::Exists { argument } => Operator::exists(rewrite(argument, column, value)?),
        Operator::NotIn { left, right } => Operator::NotIn {
            left: Box::new(rewrite(left, column, value)?),
            right: Box::new(in_right(left, right, column, value)?),
        },
        Operator::Like { left, right } => binary(left, right, column, value, |left, right| {
            Operator::Like { left, right }
        })?,
        Operator::NotLike { left, right } => binary(left, right, column, value, |left, right| {
            Operator::NotLike { left, right }
        })?,
        Operator::Between {
            value: subject,
            low,
            high,
        } => bounds_op(subject, low, high, column, value, |value, low, high| {
            Operator::Between { value, low, high }
        })?,
        Operator::NotBetween {
            value: subject,
            low,
            high,
        } => bounds_op(subject, low, high, column, value, |value, low, high| {
            Operator::NotBetween { value, low, high }
        })?,
        Operator::In { left, right } => Operator::In {
            left: Box::new(rewrite(left, column, value)?),
            right: Box::new(in_right(left, right, column, value)?),
        },
        Operator::Equals { left, right } => binary(left, right, column, value, |left, right| {
            Operator::Equals { left, right }
        })?,
        Operator::NotEquals { left, right } => {
            binary(left, right, column, value, |left, right| {
                Operator::NotEquals { left, right }
            })?
        }
        Operator::GreaterThan { left, right } => {
            binary(left, right, column, value, |left, right| {
                Operator::GreaterThan { left, right }
            })?
        }
        Operator::LessThan { left, right } => binary(left, right, column, value, |left, right| {
            Operator::LessThan { left, right }
        })?,
        Operator::GreaterThanOrEqual { left, right } => {
            binary(left, right, column, value, |left, right| {
                Operator::GreaterThanOrEqual { left, right }
            })?
        }
        Operator::LessThanOrEqual { left, right } => {
            binary(left, right, column, value, |left, right| {
                Operator::LessThanOrEqual { left, right }
            })?
        }
    })
}

fn bounds_op<C, V>(
    subject: &Expression,
    low: &Expression,
    high: &Expression,
    column: &C,
    value: &V,
    build: impl FnOnce(Box<Expression>, Box<Expression>, Box<Expression>) -> Operator,
) -> Result<Operator>
where
    C: Fn(&str) -> String,
    V: Fn(&str, &Value) -> Result<Value>,
{
    match (subject, low, high) {
        (Expression::Identifier(id), Expression::Literal(low), Expression::Literal(high)) => {
            Ok(build(
                Box::new(Expression::Identifier(Identifier::new(column(&id.name)))),
                Box::new(Expression::Literal(Literal::new(value(
                    &id.name, &low.value,
                )?))),
                Box::new(Expression::Literal(Literal::new(value(
                    &id.name,
                    &high.value,
                )?))),
            ))
        }
        _ => Ok(build(
            Box::new(rewrite(subject, column, value)?),
            Box::new(rewrite(low, column, value)?),
            Box::new(rewrite(high, column, value)?),
        )),
    }
}

fn binary<C, V>(
    left: &Expression,
    right: &Expression,
    column: &C,
    value: &V,
    build: impl FnOnce(Box<Expression>, Box<Expression>) -> Operator,
) -> Result<Operator>
where
    C: Fn(&str) -> String,
    V: Fn(&str, &Value) -> Result<Value>,
{
    let (left, right) = pair(left, right, column, value)?;
    Ok(build(Box::new(left), Box::new(right)))
}

fn pair<C, V>(
    left: &Expression,
    right: &Expression,
    column: &C,
    value: &V,
) -> Result<(Expression, Expression)>
where
    C: Fn(&str) -> String,
    V: Fn(&str, &Value) -> Result<Value>,
{
    match (left, right) {
        (Expression::Identifier(id), Expression::Literal(lit)) => Ok((
            Expression::Identifier(Identifier::new(column(&id.name))),
            Expression::Literal(Literal::new(value(&id.name, &lit.value)?)),
        )),
        (Expression::Literal(lit), Expression::Identifier(id)) => Ok((
            Expression::Literal(Literal::new(value(&id.name, &lit.value)?)),
            Expression::Identifier(Identifier::new(column(&id.name))),
        )),
        _ => Ok((
            rewrite(left, column, value)?,
            rewrite(right, column, value)?,
        )),
    }
}

fn in_right<C, V>(
    left: &Expression,
    right: &Expression,
    column: &C,
    value: &V,
) -> Result<Expression>
where
    C: Fn(&str) -> String,
    V: Fn(&str, &Value) -> Result<Value>,
{
    match (left, right) {
        (Expression::Identifier(id), Expression::Array(arr)) => {
            let elements = arr
                .elements
                .iter()
                .map(|element| match element {
                    Expression::Literal(lit) => Ok(Expression::Literal(Literal::new(value(
                        &id.name, &lit.value,
                    )?))),
                    other => rewrite(other, column, value),
                })
                .collect::<Result<Vec<_>>>()?;
            Ok(Expression::Array(ArrayExpression::new(elements)))
        }
        _ => rewrite(right, column, value),
    }
}
