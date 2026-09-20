use crate::mappers::mapped_value::MappedRow;
use crate::mappers::type_map::TypeMap;
use crate::mappers::Mapper;
use crate::row::Row;

pub trait FlatToNestedMapper: Mapper<Vec<Row>, Vec<MappedRow>> {
    fn spec(&self) -> &TypeMap;
}
