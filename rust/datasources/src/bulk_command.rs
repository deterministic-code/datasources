use crate::parameter::Parameter;

pub struct BulkCommand {
    pub text: String,
    pub parameters: Vec<Parameter>,
}
