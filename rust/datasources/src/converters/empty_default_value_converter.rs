use super::DefaultValueConverter;
use crate::datasource_error::DatasourceError;
use crate::result::Result;

pub struct EmptyDefaultValueConverter;

pub static EMPTY_DEFAULT_VALUE_CONVERTER: EmptyDefaultValueConverter = EmptyDefaultValueConverter;

impl DefaultValueConverter for EmptyDefaultValueConverter {
    fn convert(&self, _: &str, _: &str) -> Result<Option<String>> {
        Ok(None)
    }

    fn reverse_convert(&self, spec_type: &str, native: &str) -> Result<String> {
        Err(DatasourceError::message(format!(
            "Native default '{native}' is not a valid {spec_type} default"
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_returns_none() {
        assert_eq!(
            EMPTY_DEFAULT_VALUE_CONVERTER.convert("string", "hi").unwrap(),
            None
        );
    }

    #[test]
    fn reverse_convert_throws() {
        assert_eq!(
            EMPTY_DEFAULT_VALUE_CONVERTER
                .reverse_convert("string", "hi")
                .unwrap_err()
                .to_string(),
            "Native default 'hi' is not a valid string default"
        );
    }
}
