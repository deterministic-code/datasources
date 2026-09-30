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

    #[test]
    fn maps_contacts() {
        let mapper = PluralizerMapper::new();
        let contact = mapper.map("Contact".into());
        let address = mapper.map("Address".into());
        assert_eq!(contact, "contacts");
        assert_eq!(address, "addresses");
    }

    #[test]
    fn reverse_maps_contacts() {
        let mapper = PluralizerMapper::new();
        let contact = mapper.reverse_map("contacts".into());
        let address = mapper.reverse_map("addresses".into());
        assert_eq!(contact, "contact");
        assert_eq!(address, "address");
    }
}
