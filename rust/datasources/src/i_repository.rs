use crate::row::Row;
use crate::schema::Schema;
use std::any::TypeId;

pub trait IRepository: Send + Sync {
    fn entity_type(&self) -> TypeId {
        TypeId::of::<Row>()
    }

    fn schema(&self) -> &Schema {
        Schema::empty()
    }
}
