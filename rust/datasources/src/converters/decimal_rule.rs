#[derive(Clone, Debug)]
pub enum DecimalRule {
    Optional { type_name: String, fallback: String },
    Required { type_name: String },
}
