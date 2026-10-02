use super::operator::Operator;

/// Boolean-valued expression. Mirrors C# `Condition`.
pub trait Condition: Send + Sync {}

impl Condition for Operator {}
