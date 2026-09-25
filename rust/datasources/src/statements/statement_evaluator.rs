use super::expression::Expression;
use super::operator::Operator;
use super::r#where::Where;
use crate::i_where::IWhere;
use crate::row::Row;
use crate::value::Value;

pub struct StatementEvaluator;

impl StatementEvaluator {
    pub fn evaluate(&self, expression: &Expression, parameters: &Row) -> Value {
        match expression {
            Expression::Identifier(id) => parameters.get(&id.name).cloned().unwrap_or(Value::Null),
            Expression::Literal(lit) => lit.value.clone(),
            Expression::Operator(Operator::Not { argument }) => {
                Value::Bool(!is_true(&self.evaluate(argument, parameters)))
            }
            Expression::Operator(Operator::And { left, right }) => Value::Bool(
                is_true(&self.evaluate(left, parameters))
                    && is_true(&self.evaluate(right, parameters)),
            ),
            Expression::Operator(Operator::Or { left, right }) => Value::Bool(
                is_true(&self.evaluate(left, parameters))
                    || is_true(&self.evaluate(right, parameters)),
            ),
            Expression::Operator(Operator::IsNull { argument }) => {
                Value::Bool(matches!(self.evaluate(argument, parameters), Value::Null))
            }
            Expression::Operator(Operator::IsNotNull { argument }) => {
                Value::Bool(!matches!(self.evaluate(argument, parameters), Value::Null))
            }
            Expression::Operator(Operator::Like { left, right }) => like(
                self.evaluate(left, parameters),
                self.evaluate(right, parameters),
            ),
            Expression::Operator(Operator::NotLike { left, right }) => {
                Value::Bool(!is_true(&like(
                    self.evaluate(left, parameters),
                    self.evaluate(right, parameters),
                )))
            }
            Expression::Operator(Operator::Between { value, low, high }) => between(
                self.evaluate(value, parameters),
                self.evaluate(low, parameters),
                self.evaluate(high, parameters),
            ),
            Expression::Operator(Operator::NotBetween { value, low, high }) => {
                Value::Bool(!is_true(&between(
                    self.evaluate(value, parameters),
                    self.evaluate(low, parameters),
                    self.evaluate(high, parameters),
                )))
            }
            Expression::Operator(Operator::NotIn { left, right }) => {
                Value::Bool(!self.contains(left, right, parameters))
            }
            Expression::Operator(Operator::Exists { .. }) | Expression::Subquery(_) => {
                panic!("subquery is not evaluable")
            }
            Expression::Operator(Operator::In { left, right }) => {
                Value::Bool(self.contains(left, right, parameters))
            }
            Expression::Operator(Operator::Equals { left, right }) => {
                Value::Bool(self.evaluate(left, parameters) == self.evaluate(right, parameters))
            }
            Expression::Operator(Operator::NotEquals { left, right }) => {
                Value::Bool(self.evaluate(left, parameters) != self.evaluate(right, parameters))
            }
            Expression::Operator(Operator::GreaterThan { left, right }) => Value::Bool(
                compare(
                    &self.evaluate(left, parameters),
                    &self.evaluate(right, parameters),
                ) > 0,
            ),
            Expression::Operator(Operator::LessThan { left, right }) => Value::Bool(
                compare(
                    &self.evaluate(left, parameters),
                    &self.evaluate(right, parameters),
                ) < 0,
            ),
            Expression::Operator(Operator::GreaterThanOrEqual { left, right }) => Value::Bool(
                compare(
                    &self.evaluate(left, parameters),
                    &self.evaluate(right, parameters),
                ) >= 0,
            ),
            Expression::Operator(Operator::LessThanOrEqual { left, right }) => Value::Bool(
                compare(
                    &self.evaluate(left, parameters),
                    &self.evaluate(right, parameters),
                ) <= 0,
            ),
            Expression::Array(_) => panic!("array is not a scalar"),
        }
    }

    pub fn evaluate_where(&self, statement: &dyn IWhere, parameters: &Row) -> Value {
        statement
            .as_any()
            .downcast_ref::<Where>()
            .map(|where_clause| self.evaluate(&where_clause.condition, parameters))
            .unwrap_or_else(|| panic!("text where is not evaluable"))
    }

    fn contains(&self, left: &Expression, right: &Expression, parameters: &Row) -> bool {
        match right {
            Expression::Array(arr) => {
                let want = self.evaluate(left, parameters);
                arr.elements
                    .iter()
                    .any(|element| self.evaluate(element, parameters) == want)
            }
            _ => panic!("empty in"),
        }
    }
}

fn is_true(value: &Value) -> bool {
    matches!(value, Value::Bool(true))
}

fn like(value: Value, pattern: Value) -> Value {
    match (value, pattern) {
        (Value::Null, _) | (_, Value::Null) => Value::Bool(false),
        (Value::Text(text), Value::Text(pattern)) => Value::Bool(wild(&text, &pattern)),
        _ => panic!("empty like"),
    }
}

fn wild(text: &str, pattern: &str) -> bool {
    fn walk(text: &[char], pattern: &[char]) -> bool {
        match pattern.split_first() {
            None => text.is_empty(),
            Some(('%', rest)) => (0..=text.len()).any(|i| walk(&text[i..], rest)),
            Some(('_', rest)) => text
                .split_first()
                .is_some_and(|(_, rest_text)| walk(rest_text, rest)),
            Some((ch, rest)) => text
                .split_first()
                .is_some_and(|(head, rest_text)| head == ch && walk(rest_text, rest)),
        }
    }
    let text: Vec<char> = text.chars().collect();
    let pattern: Vec<char> = pattern.chars().collect();
    walk(&text, &pattern)
}

fn between(value: Value, low: Value, high: Value) -> Value {
    if matches!(value, Value::Null) || matches!(low, Value::Null) || matches!(high, Value::Null) {
        return Value::Bool(false);
    }
    Value::Bool(compare(&value, &low) >= 0 && compare(&value, &high) <= 0)
}

fn compare(left: &Value, right: &Value) -> i32 {
    match (left, right) {
        (Value::Int(a), Value::Int(b)) => a.cmp(b) as i32,
        (Value::Float(a), Value::Float(b)) => a.partial_cmp(b).expect("float") as i32,
        (Value::Decimal(a), Value::Decimal(b)) => a.cmp(b) as i32,
        (Value::Int(a), Value::Float(b)) => (*a as f64).partial_cmp(b).expect("float") as i32,
        (Value::Float(a), Value::Int(b)) => a.partial_cmp(&(*b as f64)).expect("float") as i32,
        (Value::Text(a), Value::Text(b)) => a.cmp(b) as i32,
        _ => panic!("empty compare"),
    }
}
