use crate::i_condition::ICondition;
use crate::i_where::IWhere;
use crate::StatementTextRenderer;
use std::sync::Mutex;

pub struct MockWhere {
    text: String,
    accept_calls: Mutex<usize>,
}

impl MockWhere {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            accept_calls: Mutex::new(0),
        }
    }

    pub fn accept_calls(&self) -> usize {
        *self.accept_calls.lock().expect("mock")
    }
}

impl crate::ReadInput for MockWhere {
    fn as_where(&self) -> Option<&dyn crate::i_where::IWhere> {
        Some(self)
    }
}

impl ICondition for MockWhere {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn accept(&self, _text: &StatementTextRenderer) -> String {
        *self.accept_calls.lock().expect("mock") += 1;
        self.text.clone()
    }
}

impl IWhere for MockWhere {}
