use super::routine_parameter::RoutineParameter;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NestedWhereArgument {
    pub path: String,
    pub parameter: RoutineParameter,
}

impl NestedWhereArgument {
    pub fn new(path: impl Into<String>, parameter: RoutineParameter) -> Self {
        Self {
            path: path.into(),
            parameter,
        }
    }
}
