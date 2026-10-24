use crate::i_where::IWhere;

#[derive(Default)]
pub struct NestedSelectBuilderOptions<'a> {
    pub where_clause: Option<&'a dyn IWhere>,
    pub stop_at: Vec<String>,
}
