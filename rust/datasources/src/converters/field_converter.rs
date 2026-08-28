use crate::result::Result;
use crate::value::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct FieldConverter {
    to: Arc<dyn Fn(&Value) -> Result<Value> + Send + Sync>,
    from: Arc<dyn Fn(&Value) -> Result<Value> + Send + Sync>,
}

impl FieldConverter {
    pub fn new(
        to: impl Fn(&Value) -> Result<Value> + Send + Sync + 'static,
        from: impl Fn(&Value) -> Result<Value> + Send + Sync + 'static,
    ) -> Self {
        Self {
            to: Arc::new(to),
            from: Arc::new(from),
        }
    }

    pub fn to(&self, value: &Value) -> Result<Value> {
        (self.to)(value)
    }

    pub fn from(&self, value: &Value) -> Result<Value> {
        (self.from)(value)
    }
}
