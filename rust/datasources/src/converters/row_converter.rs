use crate::row::Row;
use crate::result::Result;

#[derive(Clone, Debug, Default)]
pub struct RowConverter;

impl RowConverter {
    pub fn convert(&self, row: &Row) -> Result<Row> {
        Ok(row.clone())
    }

    pub fn reverse_convert(&self, obj: &Row) -> Result<Row> {
        Ok(obj.clone())
    }
}
