use crate::row::Row;
use std::any::TypeId;

pub trait IRepository: Send + Sync {
    fn entity_type(&self) -> TypeId {
        TypeId::of::<Row>()
    }
}
