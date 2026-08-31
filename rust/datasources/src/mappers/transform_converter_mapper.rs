use crate::converters::TypeConverter;
use crate::mappers::IMapper;

pub struct TransformConverterMapper<T> {
    converter: Box<dyn TypeConverter<T, T> + Send + Sync>,
    filter: Option<Box<dyn Fn(&T) -> bool + Send + Sync>>,
}

impl<T> TransformConverterMapper<T> {
    pub fn new(converter: impl TypeConverter<T, T> + Send + Sync + 'static) -> Self {
        Self {
            converter: Box::new(converter),
            filter: None,
        }
    }

    pub fn with_filter(
        converter: impl TypeConverter<T, T> + Send + Sync + 'static,
        filter: impl Fn(&T) -> bool + Send + Sync + 'static,
    ) -> Self {
        Self {
            converter: Box::new(converter),
            filter: Some(Box::new(filter)),
        }
    }

    fn apply(&self, value: T, reverse: bool) -> T {
        match (
            self.filter.as_ref().is_none_or(|filter| filter(&value)),
            reverse,
        ) {
            (true, false) => self.converter.convert(&value).expect("convert"),
            (true, true) => self.converter.reverse_convert(&value).expect("convert"),
            (false, _) => value,
        }
    }
}

impl<T> IMapper<T, T> for TransformConverterMapper<T> {
    fn map(&self, value: T) -> T {
        self.apply(value, false)
    }

    fn reverse_map(&self, value: T) -> T {
        self.apply(value, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::converters::TypeConverter;
    use crate::result::Result;

    struct Wrap;

    impl TypeConverter<String, String> for Wrap {
        fn convert(&self, from: &String) -> Result<String> {
            Ok(format!("[{from}]"))
        }

        fn reverse_convert(&self, to: &String) -> Result<String> {
            Ok(to.trim_matches(['[', ']']).to_string())
        }
    }

    #[test]
    fn converts_both_ways_without_filter() {
        let mapper = TransformConverterMapper::new(Wrap);
        assert_eq!(IMapper::map(&mapper, "ab".into()), "[ab]");
        assert_eq!(IMapper::reverse_map(&mapper, "[ab]".into()), "ab");
    }

    #[test]
    fn converts_when_filter_is_true() {
        let mapper = TransformConverterMapper::with_filter(Wrap, |s| s.contains("ab"));
        assert_eq!(IMapper::map(&mapper, "ab".into()), "[ab]");
        assert_eq!(IMapper::reverse_map(&mapper, "[ab]".into()), "ab");
    }

    #[test]
    fn is_identity_when_filter_is_false() {
        let mapper = TransformConverterMapper::with_filter(Wrap, |s| s.contains("ab"));
        assert_eq!(IMapper::map(&mapper, "a".into()), "a");
        assert_eq!(IMapper::reverse_map(&mapper, "[a]".into()), "[a]");
    }
}
