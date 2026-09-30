use super::Mapper;
use crate::converters::{Casing, ChangeCasingConverter};

pub struct ChangeCasingMapper {
    forward: ChangeCasingConverter,
    reverse: ChangeCasingConverter,
}

impl ChangeCasingMapper {
    pub fn new(from: Casing, to: Casing) -> Self {
        Self {
            forward: ChangeCasingConverter::new(to),
            reverse: ChangeCasingConverter::new(from),
        }
    }
}

impl Mapper<String, String> for ChangeCasingMapper {
    fn map(&self, value: String) -> String {
        self.forward.convert(&value)
    }

    fn reverse_map(&self, value: String) -> String {
        self.reverse.convert(&value)
    }
}

#[cfg(test)]
mod tests {
    use super::ChangeCasingMapper;
    use crate::converters::Casing;
    use crate::mappers::Mapper;

    #[test]
    fn maps_pascal_to_snake_and_back() {
        let mapper = ChangeCasingMapper::new(Casing::Pascal, Casing::Snake);
        assert_eq!(mapper.map("UserType".into()), "user_type");
        assert_eq!(mapper.map("Id".into()), "id");
        assert_eq!(mapper.reverse_map("user_type".into()), "UserType");
        assert_eq!(mapper.reverse_map("id".into()), "Id");
    }

    #[test]
    fn maps_contact_names() {
        let mapper = ChangeCasingMapper::new(Casing::Pascal, Casing::Snake);
        let contact = mapper.map("Contact".into());
        let first_name = mapper.map("FirstName".into());
        let email_address = mapper.map("EmailAddress".into());
        assert_eq!(contact, "contact");
        assert_eq!(first_name, "first_name");
        assert_eq!(email_address, "email_address");
    }

    #[test]
    fn reverse_maps_contact_names() {
        let mapper = ChangeCasingMapper::new(Casing::Pascal, Casing::Snake);
        let contact = mapper.reverse_map("contact".into());
        let first_name = mapper.reverse_map("first_name".into());
        let email_address = mapper.reverse_map("email_address".into());
        assert_eq!(contact, "Contact");
        assert_eq!(first_name, "FirstName");
        assert_eq!(email_address, "EmailAddress");
    }
}
