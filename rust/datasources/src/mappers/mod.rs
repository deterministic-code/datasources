#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Mapper {
    pub name: String,
    pub source: String,
    pub destination: String,
}

impl Mapper {
    pub fn new(
        name: impl Into<String>,
        source: impl Into<String>,
        destination: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_and_from_are_the_name() {
        let mapper = Mapper::new("users", "sql", "rust");
        assert_eq!(mapper.name, "users");
        assert_eq!(mapper.source, "sql");
        assert_eq!(mapper.destination, "rust");
        assert_eq!(mapper.to("user_accounts"), "user_accounts");
        assert_eq!(mapper.from("UserAccounts"), "UserAccounts");
        assert_eq!(mapper, mapper);
        assert!(format!("{:?}", mapper).contains("sql"));
    }
}
