use super::expression::Expression;

#[derive(Clone)]
pub struct ArrayExpression {
    pub elements: Vec<Expression>,
}

impl ArrayExpression {
    pub fn new(elements: Vec<Expression>) -> Self {
        Self { elements }
    }
}
