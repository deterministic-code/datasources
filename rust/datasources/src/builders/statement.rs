use crate::parameter::Parameter;

pub trait Statement: Send + Sync {
    fn get_text(&self) -> String;
    fn parameters(&self) -> Vec<Parameter> {
        Vec::new()
    }
}
