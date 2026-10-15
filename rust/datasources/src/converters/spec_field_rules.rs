use super::decimal_rule::DecimalRule;

#[derive(Clone)]
pub struct SpecFieldRules {
    pub dialect_name: Option<String>,
    pub string_unlimited: String,
    pub string_sized: fn(i32) -> String,
    pub character: Option<String>,
    pub decimal: Option<DecimalRule>,
    pub binary_unlimited: Option<String>,
    pub binary_sized: Option<fn(i32) -> String>,
    pub occ_binary: Option<String>,
    pub row_version: Option<String>,
}
