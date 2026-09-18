use crate::mappers::mapped_value::MappedRow;
use crate::mappers::type_map::TypeMap;
use crate::mappers::Mapper;
use crate::row::Row;

pub trait FlatToTypeMapper: Mapper<Row, MappedRow> {
    fn spec(&self) -> &TypeMap;
    fn collapse(&self, rows: Vec<MappedRow>) -> Vec<MappedRow>;
    fn collapse_rows(&self, rows: Vec<Row>) -> Vec<MappedRow> {
        self.collapse(rows.into_iter().map(|row| self.map(row)).collect())
    }
}
