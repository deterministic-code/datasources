use super::operator::Operator;
use crate::value::Value;

pub fn eq(column: &str, value: Value) -> Operator {
    Operator::equals(column, value)
}

pub fn ne(column: &str, value: Value) -> Operator {
    Operator::not_equals(column, value)
}

pub fn gt(column: &str, value: Value) -> Operator {
    Operator::greater_than(column, value)
}

pub fn lt(column: &str, value: Value) -> Operator {
    Operator::less_than(column, value)
}

pub fn ge(column: &str, value: Value) -> Operator {
    Operator::greater_than_or_equal(column, value)
}

pub fn le(column: &str, value: Value) -> Operator {
    Operator::less_than_or_equal(column, value)
}
