pub struct Native(String);

impl Native {
    pub fn new(text: impl Into<String>) -> Self {
        Self(text.into())
    }

    pub fn text(&self) -> &str {
        &self.0
    }
}

pub fn n(text: impl Into<String>) -> Native {
    Native::new(text)
}
