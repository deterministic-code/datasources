use super::Mapper;
use crate::converters::PluralizerConverter;

pub struct PluralizerMapper {
    pluralizer: PluralizerConverter,
}

impl PluralizerMapper {
    pub fn new() -> Self {
        Self {
            pluralizer: PluralizerConverter,
        }
    }
}

impl Default for PluralizerMapper {
    fn default() -> Self {
        Self::new()
    }
}

impl Mapper<String, String> for PluralizerMapper {
    fn map(&self, value: String) -> String {
        self.pluralizer.convert(&value).to_lowercase()
    }

    fn reverse_map(&self, value: String) -> String {
        self.pluralizer.reverse_convert(&value)
    }
}

#[cfg(test)]
mod tests {
    use super::PluralizerMapper;
    use crate::mappers::Mapper;

    #[test]
    fn pluralizes_and_lowercases_the_table_name() {
        let mapper = PluralizerMapper::new();
        assert_eq!(mapper.map("User".into()), "users");
        assert_eq!(mapper.reverse_map("users".into()), "user");
    }
}
