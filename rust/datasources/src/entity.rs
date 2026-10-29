use crate::default_write_enrichment_strategy::{decode_rows, encode_rows};
use crate::result::Result;
use crate::row::Row;
use crate::schema::Schema;
use std::collections::HashMap;

pub trait Entity: Clone + Send + Sync + 'static {
    fn to_row(&self) -> Row;
    fn from_row(row: Row) -> Result<Self>
    where
        Self: Sized;
    fn children(&self, property: &str) -> Vec<Row> {
        let _ = property;
        Vec::new()
    }
    fn set_children(&mut self, property: &str, rows: Vec<Row>) {
        let _ = (property, rows);
    }
    fn child_properties() -> &'static [&'static str] {
        &[]
    }
    fn schema() -> Schema {
        Schema::new(HashMap::new())
    }
}

impl Entity for Row {
    fn to_row(&self) -> Row {
        self.clone()
    }

    fn from_row(row: Row) -> Result<Self> {
        Ok(row)
    }

    fn children(&self, property: &str) -> Vec<Row> {
        decode_rows(self.get(property))
    }

    fn set_children(&mut self, property: &str, rows: Vec<Row>) {
        self.insert(property.to_string(), encode_rows(&rows));
    }
}

pub fn stored_row<T: Entity>(item: &T) -> Row {
    let mut row = item.to_row();
    for name in T::child_properties() {
        row.insert((*name).to_string(), encode_rows(&item.children(name)));
    }
    row
}

pub fn entity_from_row<T: Entity>(row: Row) -> Result<T> {
    let mut item = T::from_row(row.clone())?;
    for name in T::child_properties() {
        item.set_children(name, decode_rows(row.get(*name)));
    }
    Ok(item)
}
