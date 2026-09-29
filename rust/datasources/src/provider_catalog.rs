use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProviderCatalog {
    pub name: String,
    pub location: String,
    pub module: String,
}

impl ProviderCatalog {
    pub fn new(
        name: impl Into<String>,
        location: impl Into<String>,
        module: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            location: location.into(),
            module: module.into(),
        }
    }

    pub fn default() -> &'static HashMap<String, Self> {
        static DEFAULT: OnceLock<HashMap<String, ProviderCatalog>> = OnceLock::new();
        DEFAULT.get_or_init(|| {
            [
                ("json", "datasources_json", "datasources_json_driver"),
                ("memory", "datasources_memory", "datasources_memory_driver"),
                ("sqlite", "datasources_sqlite", "datasources_sqlite_driver"),
                ("xml", "datasources_xml", "datasources_xml_driver"),
                ("yaml", "datasources_yaml", "datasources_yaml_driver"),
                ("mysql", "datasources_mysql", "datasources_mysql_driver"),
                ("sqlserver", "datasources_sqlserver", "datasources_sqlserver_driver"),
                ("postgres", "datasources_postgres", "datasources_postgres_driver"),
                ("oracle", "datasources_oracle", "datasources_oracle_driver"),
                ("mongodb", "datasources_mongodb", "datasources_mongodb_driver"),
                ("dynamodb", "datasources_dynamodb", "datasources_dynamodb_driver"),
            ]
            .into_iter()
            .map(|(name, location, module)| (name.to_string(), Self::new(name, location, module)))
            .collect()
        })
    }
}
