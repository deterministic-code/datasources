use super::text::Text;
use crate::parameter::Parameter;

pub trait Statement: Text {
    fn parameters(&self) -> Vec<Parameter> {
        Vec::new()
    }
}
