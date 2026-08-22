use crate::query::Query;
use std::sync::Mutex;

pub struct MockQuery {
    text: String,
    calls: Mutex<usize>,
}

impl MockQuery {
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

impl Query for MockQuery {
    fn get_text(&self) -> String {
        *self.calls.lock().expect("mock") += 1;
        self.text.clone()
    }
}
