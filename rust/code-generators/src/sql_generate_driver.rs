use datasources::{DefaultValueConverter, DeterministicTypeConverter, StatementTextRenderer};

pub struct SqlGenerateDriver {
    pub quote_left: &'static str,
    pub quote_right: &'static str,
    pub type_converter: &'static DeterministicTypeConverter,
    pub default_value_converter: &'static DefaultValueConverter,
    pub statement_renderer: &'static StatementTextRenderer,
}
