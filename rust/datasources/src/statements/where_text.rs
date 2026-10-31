use super::i_where::IWhere;
use crate::StatementTextRenderer;
use crate::Where;

pub(crate) fn boxed(where_clause: Option<&dyn IWhere>) -> Option<Box<dyn IWhere>> {
    match where_clause {
        None => None,
        Some(predicate) => match predicate.as_any().downcast_ref::<Where>() {
            Some(clause) => Some(Box::new(clause.clone())),
            None => Some(Box::new(WhereText::new(
                crate::DefaultStatementTextRenderer::instance().render_where(predicate),
            ))),
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

    pub fn raw_text(&self) -> &str {
        &self.text
    }
}

impl crate::ReadInput for WhereText {
    fn as_where(&self) -> Option<&dyn IWhere> {
        Some(self)
    }
}

impl crate::ICondition for WhereText {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn accept(&self, text: &StatementTextRenderer) -> String {
        text.conditions().visit_where_text(self, text)
    }
}

impl IWhere for WhereText {}
