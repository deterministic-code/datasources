use crate::StatementTextRenderer;
use std::any::Any;

pub trait ICondition: Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn accept(&self, text: &StatementTextRenderer) -> String;
}
