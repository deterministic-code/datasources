use super::statement_text_renderer::StatementTextRenderer;
use std::sync::LazyLock;

pub struct DefaultStatementTextRenderer;

impl DefaultStatementTextRenderer {
    pub fn instance() -> &'static StatementTextRenderer {
        static INSTANCE: LazyLock<StatementTextRenderer> =
            LazyLock::new(|| StatementTextRenderer::new(None, None));
        &INSTANCE
    }
}
