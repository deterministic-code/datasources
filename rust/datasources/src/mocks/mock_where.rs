use crate::statement::Statement;
use crate::where_trait::Where;
use std::sync::Mutex;

pub struct MockWhere {
    text: String,
    calls: Mutex<usize>,
}

impl MockWhere {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            calls: Mutex::new(0),
        }
    }

    pub fn calls(&self) -> usize {
        *self.calls.lock().expect("mock")
    }
}

impl Statement for MockWhere {
    fn get_text(&self) -> String {
        *self.calls.lock().expect("mock") += 1;
        self.text.clone()
    }
}

impl Where for MockWhere {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
