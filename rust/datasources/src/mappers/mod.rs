use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DatasourceMapper {
    pub name: String,
    pub source: String,
    pub destination: String,
    to: HashMap<String, String>,
    from: HashMap<String, String>,
}

impl DatasourceMapper {
    pub fn mapping(source: &str, destination: &str, names: &[(&str, &str)]) -> Self {
        Self::mapping_named("", source, destination, names)
    }

    pub fn mapping_named(
        name: impl Into<String>,
        source: &str,
        destination: &str,
        names: &[(&str, &str)],
    ) -> Self {
        let to = names
            .iter()
            .map(|(src, dest)| (src.to_string(), dest.to_string()))
            .collect::<HashMap<_, _>>();
        let from = names
            .iter()
            .map(|(src, dest)| (dest.to_string(), src.to_string()))
            .collect();
        Self {
            name: name.into(),
            source: source.to_string(),
            destination: destination.to_string(),
            to,
            from,
        }
    }

    pub fn to(&self, name: &str) -> String {
        self.to
            .get(name)
            .cloned()
            .unwrap_or_else(|| name.to_string())
    }

    pub fn from(&self, name: &str) -> String {
        self.from
            .get(name)
            .cloned()
            .unwrap_or_else(|| name.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_names_the_caller_supplies() {
        let mapping = DatasourceMapper::mapping_named(
            "users",
            "sql",
            "rust",
            &[("user_accounts", "accounts"), ("created_at", "created_at")],
        );
        assert_eq!(mapping.name, "users");
        assert_eq!(mapping.source, "sql");
        assert_eq!(mapping.destination, "rust");
        assert_eq!(mapping.to("user_accounts"), "accounts");
        assert_eq!(mapping.to("id"), "id");
        assert_eq!(mapping.from("created_at"), "created_at");
        assert_eq!(mapping.from("id"), "id");
        assert_eq!(mapping, mapping);
        assert_eq!(format!("{:?}", mapping).contains("sql"), true);
    }

    #[test]
    fn is_identity_when_no_names_are_given() {
        let mapping = DatasourceMapper::mapping("sql", "go", &[]);
        assert_eq!(mapping.to("user_name"), "user_name");
        assert_eq!(mapping.from("UserName"), "UserName");
    }
}
