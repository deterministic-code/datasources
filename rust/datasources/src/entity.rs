use crate::converters::{MetaDataRowTypeConverter, SchemaObject, TypeConverter};
use crate::enrichment_payload::{
    cell_from_row, cell_from_struct, payload_from_row, payload_from_struct, set_cell_on_row,
    set_cell_on_struct, set_one_on_struct, set_payload_on_row, set_payload_on_struct,
    set_typed_list_on_struct, typed_list_from_struct,
};
use crate::result::Result;
use crate::row::Row;
use crate::value::Value;
use facet::Peek;

pub trait Entity: Clone + Default + Send + Sync + 'static {
    fn object_converter() -> Box<dyn TypeConverter<Self, Row> + Send + Sync>;

    fn is_dictionary() -> bool {
        false
    }

    fn payload(&self, name: &str) -> Vec<Row> {
        let _ = name;
        Vec::new()
    }

    fn set_payload(&mut self, name: &str, rows: Vec<Row>) -> Result<()> {
        let _ = (name, rows);
        Ok(())
    }

    fn related<R: Entity>(&self, name: &str) -> Vec<R> {
        self.payload(name)
            .into_iter()
            .filter_map(|row| R::object_converter().convert(&row).ok())
            .collect()
    }

    fn set_related<R: Entity>(&mut self, name: &str, items: Vec<R>) -> Result<()> {
        let rows = items
            .iter()
            .map(|item| R::object_converter().reverse_convert(item))
            .collect::<Result<Vec<_>>>()?;
        self.set_payload(name, rows)
    }

    fn set_one<R: Entity>(&mut self, name: &str, item: R) -> Result<()> {
        self.set_related(name, vec![item])
    }

    fn cell(&self, name: &str) -> Option<Value> {
        let _ = name;
        None
    }

    fn set_cell(&mut self, name: &str, value: Value) -> Result<()> {
        let _ = (name, value);
        Ok(())
    }

    fn has(&self, name: &str) -> bool {
        self.cell(name).is_some()
    }

    fn schema_list<R: SchemaObject>(&self, name: &str) -> Vec<R> {
        self.payload(name)
            .into_iter()
            .filter_map(|row| R::object_converter().convert(&row).ok())
            .collect()
    }

    fn set_schema_list<R: SchemaObject>(&mut self, name: &str, items: Vec<R>) -> Result<()> {
        let rows = items
            .iter()
            .map(|item| R::object_converter().reverse_convert(item))
            .collect::<Result<Vec<_>>>()?;
        self.set_payload(name, rows)
    }

    fn set_schema_one<R: SchemaObject>(&mut self, name: &str, item: R) -> Result<()> {
        self.set_schema_list(name, vec![item])
    }
}

impl Entity for Row {
    fn object_converter() -> Box<dyn TypeConverter<Row, Row> + Send + Sync> {
        Box::new(MetaDataRowTypeConverter::<Row>::new())
    }

    fn is_dictionary() -> bool {
        true
    }

    fn payload(&self, name: &str) -> Vec<Row> {
        payload_from_row(self, name)
    }

    fn set_payload(&mut self, name: &str, rows: Vec<Row>) -> Result<()> {
        set_payload_on_row(self, name, rows);
        Ok(())
    }

    fn cell(&self, name: &str) -> Option<Value> {
        cell_from_row(self, name)
    }

    fn set_cell(&mut self, name: &str, value: Value) -> Result<()> {
        set_cell_on_row(self, name, value);
        Ok(())
    }

    fn has(&self, name: &str) -> bool {
        self.contains_key(name)
    }
}

impl<T: SchemaObject> Entity for T {
    fn object_converter() -> Box<dyn TypeConverter<T, Row> + Send + Sync> {
        let mut converter = MetaDataRowTypeConverter::<T>::new();
        converter.should_coerce_id = true;
        Box::new(converter)
    }

    fn payload(&self, name: &str) -> Vec<Row> {
        payload_from_struct(self, name)
    }

    fn set_payload(&mut self, name: &str, rows: Vec<Row>) -> Result<()> {
        *self = set_payload_on_struct(self, name, rows)?;
        Ok(())
    }

    fn cell(&self, name: &str) -> Option<Value> {
        cell_from_struct(self, name)
    }

    fn set_cell(&mut self, name: &str, value: Value) -> Result<()> {
        *self = set_cell_on_struct(self, name, value)?;
        Ok(())
    }

    fn has(&self, name: &str) -> bool {
        Peek::new(self).into_struct().ok().and_then(|st| st.field_by_name(name).ok()).is_some()
    }

    fn schema_list<R: SchemaObject>(&self, name: &str) -> Vec<R> {
        typed_list_from_struct(self, name)
    }

    fn set_schema_list<R: SchemaObject>(&mut self, name: &str, items: Vec<R>) -> Result<()> {
        *self = set_typed_list_on_struct(self, name, items)?;
        Ok(())
    }

    fn set_schema_one<R: SchemaObject>(&mut self, name: &str, item: R) -> Result<()> {
        *self = set_one_on_struct(self, name, item)?;
        Ok(())
    }
}

pub fn assign_related<T: Entity, TRelated: Entity>(
    parent: &mut T,
    name: &str,
    items: Vec<TRelated>,
) -> Result<()> {
    parent.set_related(name, items)
}

pub fn assign_one<T: Entity, TRelated: Entity>(parent: &mut T, name: &str, item: TRelated) -> Result<()> {
    parent.set_one(name, item)
}
