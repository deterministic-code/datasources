use crate::i_where::IWhere;
use crate::statements::text::Text;

pub struct WhereText {
    text: String,
}

impl WhereText {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl Text for WhereText {
    fn get_text(&self) -> String {
        self.text.clone()
    }
}

impl crate::ReadInput for WhereText {
    fn as_where(&self) -> Option<&dyn IWhere> {
        Some(self)
    }
}

impl IWhere for WhereText {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
