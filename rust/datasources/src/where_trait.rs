use crate::statement::Statement;
use std::any::Any;

pub trait Where: Statement {
    fn as_any(&self) -> &dyn Any;
}
