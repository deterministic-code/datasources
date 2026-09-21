use super::array_expression::ArrayExpression;
use super::expression::Expression;
use super::identifier::Identifier;
use super::literal::Literal;
use crate::value::Value;

#[derive(Clone)]
pub enum Operator {
    Equals {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    NotEquals {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    GreaterThan {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    LessThan {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    GreaterThanOrEqual {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    LessThanOrEqual {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    And {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Or {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    In {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Not {
        argument: Box<Expression>,
    },
}

impl Operator {
    pub fn equals(field: impl Into<String>, value: impl Into<Value>) -> Self {
        Self::Equals {
            left: Box::new(Expression::Identifier(Identifier::new(field))),
            right: Box::new(Expression::Literal(Literal::new(value))),
        }
    }

    pub fn not_equals(field: impl Into<String>, value: impl Into<Value>) -> Self {
        Self::NotEquals {
            left: Box::new(Expression::Identifier(Identifier::new(field))),
            right: Box::new(Expression::Literal(Literal::new(value))),
        }
    }

    pub fn greater_than(field: impl Into<String>, value: impl Into<Value>) -> Self {
        Self::GreaterThan {
            left: Box::new(Expression::Identifier(Identifier::new(field))),
            right: Box::new(Expression::Literal(Literal::new(value))),
        }
    }

    pub fn less_than(field: impl Into<String>, value: impl Into<Value>) -> Self {
        Self::LessThan {
            left: Box::new(Expression::Identifier(Identifier::new(field))),
            right: Box::new(Expression::Literal(Literal::new(value))),
        }
    }

    pub fn greater_than_or_equal(field: impl Into<String>, value: impl Into<Value>) -> Self {
        Self::GreaterThanOrEqual {
            left: Box::new(Expression::Identifier(Identifier::new(field))),
            right: Box::new(Expression::Literal(Literal::new(value))),
        }
    }

    pub fn less_than_or_equal(field: impl Into<String>, value: impl Into<Value>) -> Self {
        Self::LessThanOrEqual {
            left: Box::new(Expression::Identifier(Identifier::new(field))),
            right: Box::new(Expression::Literal(Literal::new(value))),
        }
    }

    pub fn and(left: impl Into<Expression>, right: impl Into<Expression>) -> Self {
        Self::And {
            left: Box::new(left.into()),
            right: Box::new(right.into()),
        }
    }

    pub fn or(left: impl Into<Expression>, right: impl Into<Expression>) -> Self {
        Self::Or {
            left: Box::new(left.into()),
            right: Box::new(right.into()),
        }
    }

    pub fn not(argument: impl Into<Expression>) -> Self {
        Self::Not {
            argument: Box::new(argument.into()),
        }
    }

    pub fn in_list(field: impl Into<String>, values: Vec<Value>) -> Self {
        Self::In {
            left: Box::new(Expression::Identifier(Identifier::new(field))),
            right: Box::new(Expression::Array(ArrayExpression::new(
                values
                    .into_iter()
                    .map(|value| Expression::Literal(Literal::new(value)))
                    .collect(),
            ))),
        }
    }
}

impl From<Operator> for Expression {
    fn from(op: Operator) -> Self {
        Expression::Operator(op)
    }
}
