use super::array_expression::ArrayExpression;
use super::identifier::Identifier;
use super::literal::Literal;
use super::operator::Operator;

#[derive(Clone)]
pub enum Expression {
    Operator(Operator),
    Identifier(Identifier),
    Literal(Literal),
    Array(ArrayExpression),
}
