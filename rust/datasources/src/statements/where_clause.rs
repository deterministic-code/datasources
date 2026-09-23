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
        _ => "IN",
    }
}

fn inline(value: &Value) -> String {
    match value {
        Value::Int(number) => number.to_string(),
        Value::Float(number) => number.to_string(),
        Value::Decimal(number) => number.to_string(),
        Value::Bool(flag) => flag.to_string(),
        Value::Text(text) => text.clone(),
        Value::Null => "null".into(),
        other => other.wire_text(),
    }
}

fn needs_parens(child: &Expression, and: bool) -> bool {
    match child {
        Expression::Operator(Operator::Not { .. } | Operator::In { .. }) => true,
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
            format!("{} {} ${}", id.name, symbol(op), id.name)
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
            vec![Parameter::new(id.name.clone(), lit.value.clone())]
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
        Expression::Array(arr) => arr
            .elements
            .iter()
            .map(render)
            .collect::<Vec<_>>()
            .join(", "),
        Expression::Operator(Operator::Not { argument }) => {
            format!("NOT ({})", render(argument))
        }
        Expression::Operator(op @ Operator::In { left, right }) => match (&**left, &**right) {
            (Expression::Identifier(_), Expression::Array(arr)) if arr.elements.is_empty() => {
                "1 = 0".into()
            }
            (Expression::Identifier(id), Expression::Array(arr)) => {
                let list = (0..arr.elements.len())
                    .map(|i| format!("${}_{i}", id.name))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{} IN ({list})", id.name)
            }
            _ => compare_text(op, left, right),
        },
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
            | Operator::LessThanOrEqual { left, right }),
        ) => compare_text(op, left, right),
    }
}

fn collect(expression: &Expression) -> Vec<Parameter> {
    match expression {
        Expression::Literal(_) | Expression::Identifier(_) => vec![],
        Expression::Operator(Operator::Not { argument }) => collect(argument),
        Expression::Operator(Operator::In { left, right }) => match (left.as_ref(), right.as_ref())
        {
            (Expression::Identifier(id), Expression::Array(arr)) => arr
                .elements
                .iter()
                .enumerate()
                .map(|(i, element)| match element {
                    Expression::Literal(lit) => {
                        Parameter::new(format!("{}_{i}", id.name), lit.value.clone())
                    }
                    _ => panic!("empty in"),
                })
                .collect(),
            (left, right) => compare_parameters(left, right),
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
            | Operator::LessThanOrEqual { left, right },
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
