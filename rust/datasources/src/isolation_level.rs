#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum IsolationLevel {
    ReadUncommitted,
    ReadCommitted,
    RepeatableRead,
    #[default]
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
