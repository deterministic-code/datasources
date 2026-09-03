use crate::value::Value;
use std::collections::HashMap;

pub type MappedRow = HashMap<String, MappedValue>;

#[derive(Clone, Debug, PartialEq)]
pub enum MappedValue {
    Cell(Value),
    Many(Vec<MappedRow>),
}
