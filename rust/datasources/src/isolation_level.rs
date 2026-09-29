#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum IsolationLevel {
    ReadUncommitted,
    #[default]
    ReadCommitted,
    RepeatableRead,
    Serializable,
}

impl IsolationLevel {
    pub fn from_name(name: &str) -> Self {
        match name {
            "readUncommitted" => Self::ReadUncommitted,
            "readCommitted" => Self::ReadCommitted,
            "repeatableRead" => Self::RepeatableRead,
            _ => Self::Serializable,
        }
    }
}
