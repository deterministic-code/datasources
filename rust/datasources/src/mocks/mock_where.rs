use crate::i_where::IWhere;
use crate::statements::text::Text;
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

impl Text for MockWhere {
    fn get_text(&self) -> String {
        *self.calls.lock().expect("mock") += 1;
        self.text.clone()
    }
}

impl crate::ReadInput for MockWhere {
    fn as_where(&self) -> Option<&dyn crate::i_where::IWhere> {
        Some(self)
    }
}

impl IWhere for MockWhere {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
