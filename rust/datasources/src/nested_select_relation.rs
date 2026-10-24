use crate::nested_select_relation_kind::NestedSelectRelationKind;
use crate::nested_select_type::NestedSelectType;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NestedSelectRelation {
    pub name: String,
    pub kind: NestedSelectRelationKind,
    pub from: String,
    pub to: String,
    pub target: NestedSelectType,
}

impl NestedSelectRelation {
    pub fn new(
        name: impl Into<String>,
        kind: NestedSelectRelationKind,
        from: impl Into<String>,
        to: impl Into<String>,
        target: NestedSelectType,
    ) -> Self {
        Self {
            name: name.into(),
            kind,
            from: from.into(),
            to: to.into(),
            target,
        }
    }
}
