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
    IsNull {
        argument: Box<Expression>,
    },
    IsNotNull {
        argument: Box<Expression>,
    },
    Like {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    NotLike {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Between {
        value: Box<Expression>,
        low: Box<Expression>,
        high: Box<Expression>,
    },
    NotBetween {
        value: Box<Expression>,
        low: Box<Expression>,
        high: Box<Expression>,
    },
    NotIn {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Exists {
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

    pub fn like(field: impl Into<String>, value: impl Into<Value>) -> Self {
        Self::binary_value(field, value, |left, right| Self::Like { left, right })
    }

    pub fn not_like(field: impl Into<String>, value: impl Into<Value>) -> Self {
        Self::binary_value(field, value, |left, right| Self::NotLike { left, right })
    }

    pub fn is_null(argument: impl Into<Expression>) -> Self {
        Self::IsNull {
            argument: Box::new(argument.into()),
        }
    }

    pub fn is_not_null(argument: impl Into<Expression>) -> Self {
        Self::IsNotNull {
            argument: Box::new(argument.into()),
        }
    }

    pub fn between(
        field: impl Into<String>,
        low: impl Into<Value>,
        high: impl Into<Value>,
    ) -> Self {
        Self::bounds(field, low, high, |value, low, high| Self::Between {
            value,
            low,
            high,
        })
    }

    pub fn not_between(
        field: impl Into<String>,
        low: impl Into<Value>,
        high: impl Into<Value>,
    ) -> Self {
        Self::bounds(field, low, high, |value, low, high| Self::NotBetween {
            value,
            low,
            high,
        })
    }

    pub fn not_in(field: impl Into<String>, values: Vec<Value>) -> Self {
        Self::NotIn {
            left: Box::new(Expression::Identifier(Identifier::new(field))),
            right: Box::new(Expression::Array(ArrayExpression::new(
                values
                    .into_iter()
                    .map(|value| Expression::Literal(Literal::new(value)))
                    .collect(),
            ))),
        }
    }

    pub fn exists(argument: impl Into<Expression>) -> Self {
        Self::Exists {
            argument: Box::new(argument.into()),
        }
    }

    fn binary_value(
        field: impl Into<String>,
        value: impl Into<Value>,
        build: impl FnOnce(Box<Expression>, Box<Expression>) -> Self,
    ) -> Self {
        build(
            Box::new(Expression::Identifier(Identifier::new(field))),
            Box::new(Expression::Literal(Literal::new(value))),
        )
    }

    fn bounds(
        field: impl Into<String>,
        low: impl Into<Value>,
        high: impl Into<Value>,
        build: impl FnOnce(Box<Expression>, Box<Expression>, Box<Expression>) -> Self,
    ) -> Self {
        build(
            Box::new(Expression::Identifier(Identifier::new(field))),
            Box::new(Expression::Literal(Literal::new(low))),
            Box::new(Expression::Literal(Literal::new(high))),
        )
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
