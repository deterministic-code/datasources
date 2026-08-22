use crate::parameter::Parameter;

#[derive(Clone)]
pub struct MiddlewareContext {
    pub operation: String,
    pub text: Option<String>,
    pub parameters: Option<Vec<Parameter>>,
    pub error: Option<String>,
}

impl MiddlewareContext {
    pub fn new(operation: impl Into<String>) -> Self {
        Self {
            operation: operation.into(),
            text: None,
            parameters: None,
            error: None,
        }
    }
}
