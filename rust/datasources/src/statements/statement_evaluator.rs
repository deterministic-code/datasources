use super::expression::Expression;
use super::operator::Operator;
use super::where_clause::WhereClause;
use crate::row::Row;
use crate::value::Value;
use crate::where_trait::Where;

pub struct StatementEvaluator;

impl StatementEvaluator {
    pub fn evaluate(&self, expression: &Expression, parameters: &Row) -> Value {
        match expression {
            Expression::Identifier(id) => parameters
                .get(&id.name)
                .cloned()
                .unwrap_or(Value::Null),
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
                compare(&self.evaluate(left, parameters), &self.evaluate(right, parameters)) > 0,
            ),
            Expression::Operator(Operator::LessThan { left, right }) => Value::Bool(
                compare(&self.evaluate(left, parameters), &self.evaluate(right, parameters)) < 0,
            ),
            Expression::Operator(Operator::GreaterThanOrEqual { left, right }) => Value::Bool(
                compare(&self.evaluate(left, parameters), &self.evaluate(right, parameters)) >= 0,
            ),
            Expression::Operator(Operator::LessThanOrEqual { left, right }) => Value::Bool(
                compare(&self.evaluate(left, parameters), &self.evaluate(right, parameters)) <= 0,
            ),
            Expression::Array(_) => panic!("array is not a scalar"),
        }
    }

    pub fn evaluate_where(&self, statement: &dyn Where, parameters: &Row) -> Value {
        statement
            .as_any()
            .downcast_ref::<WhereClause>()
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
