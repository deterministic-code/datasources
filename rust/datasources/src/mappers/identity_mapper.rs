use super::Mapper;
use std::marker::PhantomData;

pub struct IdentityMapper<T> {
    _value: PhantomData<fn() -> T>,
}

impl<T> IdentityMapper<T> {
    pub fn new() -> Self {
        Self {
            _value: PhantomData,
        }
    }
}

impl<T> Default for IdentityMapper<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Mapper<T, T> for IdentityMapper<T> {
    fn map(&self, value: T) -> T {
        value
    }

    fn reverse_map(&self, value: T) -> T {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::IdentityMapper;
    use crate::mappers::Mapper;

    #[test]
    fn maps_and_reverses_the_same_value() {
        let mapper = IdentityMapper::<String>::new();
        let mapped = mapper.map("contact".into());
        let reversed = mapper.reverse_map("contact".into());
        assert_eq!(mapped, "contact");
        assert_eq!(reversed, "contact");
    }
}
