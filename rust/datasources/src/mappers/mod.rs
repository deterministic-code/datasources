pub trait IMapper<TFrom, TTo> {
    fn map(&self, value: TFrom) -> TTo;
    fn reverse_map(&self, value: TTo) -> TFrom;
}

pub trait ISourceDestinationMapper {
    fn source(&self) -> &str;
    fn destination(&self) -> &str;
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct NameMapper;

impl NameMapper {
    pub fn map(&self, name: &str) -> String {
        name.to_string()
    }

    pub fn reverse_map(&self, name: &str) -> String {
        name.to_string()
    }
}

impl IMapper<String, String> for NameMapper {
    fn map(&self, name: String) -> String {
        name
    }

    fn reverse_map(&self, name: String) -> String {
        name
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NameSourceDestinationMapper {
    pub source: String,
    pub destination: String,
}

impl NameSourceDestinationMapper {
    pub fn new(source: impl Into<String>, destination: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            destination: destination.into(),
        }
    }

    pub fn map(&self, name: &str) -> String {
        name.to_string()
    }

    pub fn reverse_map(&self, name: &str) -> String {
        name.to_string()
    }
}

impl IMapper<String, String> for NameSourceDestinationMapper {
    fn map(&self, name: String) -> String {
        name
    }

    fn reverse_map(&self, name: String) -> String {
        name
    }
}

impl ISourceDestinationMapper for NameSourceDestinationMapper {
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
    fn map_and_reverse_map_are_the_name() {
        let mapper = NameMapper;
        assert_eq!(mapper.map("user_accounts"), "user_accounts");
        assert_eq!(mapper.reverse_map("UserAccounts"), "UserAccounts");
        assert_eq!(IMapper::map(&mapper, "user_accounts".into()), "user_accounts");
        assert_eq!(
            IMapper::reverse_map(&mapper, "UserAccounts".into()),
            "UserAccounts"
        );
    }

    #[test]
    fn source_and_destination() {
        let mapper = NameSourceDestinationMapper::new("sql", "rust");
        assert_eq!(mapper.source(), "sql");
        assert_eq!(mapper.destination(), "rust");
        assert_eq!(mapper.map("user_accounts"), "user_accounts");
        assert_eq!(mapper.reverse_map("UserAccounts"), "UserAccounts");
        assert_eq!(mapper, mapper);
        assert!(format!("{:?}", mapper).contains("sql"));
    }
}
