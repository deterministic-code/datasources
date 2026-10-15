#[derive(Clone, Debug)]
pub enum FieldSize {
    Length(i32),
    Decimal(i32, i32),
    Unlimited,
}

#[derive(Clone, Debug, Default)]
pub struct SpecField {
    pub type_name: String,
    pub name: Option<String>,
    pub size: Option<FieldSize>,
    pub default_value: Option<String>,
    pub is_optimistic_concurrency: bool,
    pub use_native_row_version: bool,
}

pub fn uses_native_row_version(field: &SpecField) -> bool {
    field.use_native_row_version && field.is_optimistic_concurrency
}

pub fn emits_native_row_version(field: &SpecField, dialect: &str) -> bool {
    dialect == "sqlserver" && uses_native_row_version(field)
}

pub fn is_occ_binary_column(field: &SpecField) -> bool {
    field.type_name == "binary" && field.is_optimistic_concurrency
}
