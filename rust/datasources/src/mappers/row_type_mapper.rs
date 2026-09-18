use crate::converters::RowTypeConverter;
use crate::mappers::TransformMapper;
use crate::row::Row;
use std::sync::Arc;

pub struct RowTypeMapper;

impl RowTypeMapper {
    pub fn transform(converter: RowTypeConverter) -> TransformMapper<Row, Row> {
        let converter = Arc::new(converter);
        let map = converter.clone();
        let reverse = converter;
        TransformMapper::new(
            move |row| map.convert(&row).expect("convert"),
            move |row| reverse.reverse_convert(&row).expect("convert"),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mappers::Mapper;
    use crate::value::Value;
    use std::collections::HashMap;

    #[test]
    fn maps_a_flat_row_and_back() {
        let mapper = RowTypeMapper::transform(RowTypeConverter::new(vec!["id".into()]));
        let row = HashMap::from([("id".into(), Value::Int(1))]);
        assert_eq!(Mapper::map(&mapper, row.clone()), row);
        assert_eq!(Mapper::reverse_map(&mapper, row.clone()), row);
    }
}
