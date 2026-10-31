use super::expression_renderer::ExpressionRenderer;
use super::i_condition_visitor::ConditionVisitor;
use super::r#where::Where;
use super::statement_text_renderer::StatementTextRenderer;
use super::i_condition::ICondition;
use super::where_text::WhereText;

pub struct ConditionRenderer;

impl ConditionVisitor for ConditionRenderer {
    fn visit_where(&self, where_clause: &Where, text: &StatementTextRenderer) -> String {
        ExpressionRenderer.render(&where_clause.condition, text)
    }

    fn visit_where_text(&self, where_clause: &WhereText, _text: &StatementTextRenderer) -> String {
        where_clause.raw_text().to_string()
    }
}

impl ConditionRenderer {
    pub fn render(&self, condition: &dyn ICondition, text: &StatementTextRenderer) -> String {
        condition.accept(text)
    }
}
