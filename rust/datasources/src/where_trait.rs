use crate::parameter::Parameter;
use crate::statements::text::Text;
use std::any::Any;

pub trait Where: Text {
    fn parameters(&self) -> Vec<Parameter> {
        Vec::new()
    }
    fn as_any(&self) -> &dyn Any;
}
