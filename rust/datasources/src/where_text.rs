use crate::i_where::IWhere;
use crate::statements::text::Text;
use crate::Where;

pub(crate) fn boxed(where_clause: Option<&dyn IWhere>) -> Option<Box<dyn IWhere>> {
    match where_clause {
        None => None,
        Some(predicate) => match predicate.as_any().downcast_ref::<Where>() {
            Some(clause) => Some(Box::new(clause.clone())),
            None => Some(Box::new(WhereText::new(predicate.get_text()))),
        },
    }
}

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
