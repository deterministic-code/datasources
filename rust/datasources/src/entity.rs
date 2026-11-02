use crate::converters::{MetaDataRowTypeConverter, SchemaObject, TypeConverter};
use crate::row::Row;

pub trait Entity: Clone + Default + Send + Sync + 'static {
    fn object_converter() -> Box<dyn TypeConverter<Self, Row> + Send + Sync>;
}

impl Entity for Row {
    fn object_converter() -> Box<dyn TypeConverter<Row, Row> + Send + Sync> {
        Box::new(MetaDataRowTypeConverter::<Row>::new())
    }
}

impl<T: SchemaObject> Entity for T {
    fn object_converter() -> Box<dyn TypeConverter<T, Row> + Send + Sync> {
        let mut converter = MetaDataRowTypeConverter::<T>::new();
        converter.should_coerce_id = true;
        Box::new(converter)
    }
}
