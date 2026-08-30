pub trait IMapper<T> {
    fn to(&self, value: T) -> T;
    fn from(&self, value: T) -> T;
}

pub trait ISourceDestinationMapper {
    fn source(&self) -> &str;
    fn destination(&self) -> &str;
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StringMapper;

impl StringMapper {
    pub fn to(&self, name: &str) -> String {
        name.to_string()
    }

    pub fn from(&self, name: &str) -> String {
        name.to_string()
    }
}

impl IMapper<String> for StringMapper {
    fn to(&self, name: String) -> String {
        name
    }

    fn from(&self, name: String) -> String {
        name
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StringSourceDestinationMapper {
    pub source: String,
    pub destination: String,
}

impl StringSourceDestinationMapper {
    pub fn new(source: impl Into<String>, destination: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            destination: destination.into(),
        }
    }

    pub fn to(&self, name: &str) -> String {
        name.to_string()
    }

    pub fn from(&self, name: &str) -> String {
        name.to_string()
    }
}

impl IMapper<String> for StringSourceDestinationMapper {
    fn to(&self, name: String) -> String {
        name
    }

    fn from(&self, name: String) -> String {
        name
    }
}

impl ISourceDestinationMapper for StringSourceDestinationMapper {
    fn source(&self) -> &str {
        &self.source
    }

    fn destination(&self) -> &str {
        &self.destination
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_and_from_are_the_name() {
        let mapper = StringMapper;
        assert_eq!(mapper.to("user_accounts"), "user_accounts");
        assert_eq!(mapper.from("UserAccounts"), "UserAccounts");
        assert_eq!(IMapper::to(&mapper, "user_accounts".into()), "user_accounts");
        assert_eq!(IMapper::from(&mapper, "UserAccounts".into()), "UserAccounts");
    }

    #[test]
    fn source_and_destination() {
        let mapper = StringSourceDestinationMapper::new("sql", "rust");
        assert_eq!(mapper.source(), "sql");
        assert_eq!(mapper.destination(), "rust");
        assert_eq!(mapper.to("user_accounts"), "user_accounts");
        assert_eq!(mapper.from("UserAccounts"), "UserAccounts");
        assert_eq!(mapper, mapper);
        assert!(format!("{:?}", mapper).contains("sql"));
    }
}
