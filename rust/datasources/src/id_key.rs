use crate::row::Row;
use crate::value::Value;

pub trait IdKey: Send + Sync {
    fn as_value(&self) -> Option<&Value> {
        None
    }

    fn as_row(&self) -> Option<&Row> {
        None
    }
}

impl IdKey for Value {
    fn as_value(&self) -> Option<&Value> {
        Some(self)
    }
}

impl IdKey for Row {
    fn as_row(&self) -> Option<&Row> {
        Some(self)
    }
}
