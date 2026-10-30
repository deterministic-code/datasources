use super::routine_parameter::RoutineParameter;

#[derive(Clone)]
pub struct ProcedureByGroup {
    pub logical: Vec<String>,
    pub columns: Vec<String>,
    pub parameters: Vec<RoutineParameter>,
    pub update_parameters: Vec<RoutineParameter>,
}

pub struct ProcedureSource {
    pub plural: String,
    pub table: String,
    pub columns: Vec<String>,
    pub create_parameters: Vec<RoutineParameter>,
    pub writable: Vec<String>,
    pub create_columns: Vec<RoutineParameter>,
    pub update_columns: Vec<RoutineParameter>,
    pub typed_columns: Vec<RoutineParameter>,
    pub by_groups: Vec<ProcedureByGroup>,
    pub key: String,
    pub key_type: String,
    pub audit_updated: Option<String>,
    pub generated_key: bool,
    pub routine_name: Box<dyn Fn(&str) -> String + Send + Sync>,
}
