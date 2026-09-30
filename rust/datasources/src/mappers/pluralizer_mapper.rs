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
        self.pluralizer.convert(&value)
    }

    fn reverse_map(&self, value: String) -> String {
        self.pluralizer.reverse_convert(&value)
    }
}

#[cfg(test)]
mod tests {
    use super::PluralizerMapper;
    use crate::converters::Casing;
    use crate::mappers::{ChangeCasingMapper, Mapper};

    #[test]
    fn pluralizes_and_lowercases_the_table_name() {
        let mapper = PluralizerMapper::new();
        assert_eq!(mapper.map("User".into()), "Users");
        assert_eq!(mapper.reverse_map("Users".into()), "User");
    }

    #[test]
    fn maps_contacts() {
        let mapper = PluralizerMapper::new();
        let contact = mapper.map("Contact".into());
        let address = mapper.map("Address".into());
        assert_eq!(contact, "Contacts");
        assert_eq!(address, "Addresses");
    }

    #[test]
    fn reverse_maps_contacts() {
        let mapper = PluralizerMapper::new();
        let contact = mapper.reverse_map("Contacts".into());
        let address = mapper.reverse_map("Addresses".into());
        assert_eq!(contact, "Contact");
        assert_eq!(address, "Address");
    }

    #[test]
    fn lowercases_then_pluralizes() {
        let casing = ChangeCasingMapper::new(Casing::Pascal, Casing::Snake);
        let pluralizer = PluralizerMapper::new();
        let contact = pluralizer.map(casing.map("Contact".into()));
        let address = pluralizer.map(casing.map("Address".into()));
        let contact_name = casing.reverse_map(pluralizer.reverse_map(contact.clone()));
        let address_name = casing.reverse_map(pluralizer.reverse_map(address.clone()));
        assert_eq!(contact, "contacts");
        assert_eq!(address, "addresses");
        assert_eq!(contact_name, "Contact");
        assert_eq!(address_name, "Address");
    }
}
