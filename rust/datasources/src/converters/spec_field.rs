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
}
