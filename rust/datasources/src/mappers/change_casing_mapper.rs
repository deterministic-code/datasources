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
}
