use datasources::{bind_install, defaults, register, DatasourceError, DatasourceFactory, Result};
use std::collections::HashMap;
use std::sync::Arc;

fn get<'a>(options: &'a HashMap<String, String>, key: &str) -> Option<&'a str> {
    options
        .get(key)
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
}

pub fn provider_names(options: &HashMap<String, String>) -> Vec<String> {
    let mut names = Vec::new();
    let listed = get(options, "providers").or_else(|| get(options, "INSTALLED_PROVIDERS"));
    if let Some(value) = listed {
        for name in value.split(',') {
            let name = name.trim();
            if !name.is_empty() && !names.iter().any(|existing| existing == name) {
                names.push(name.to_string());
            }
        }
    }
    if let Some(provider) = get(options, "provider").map(str::to_string).or_else(|| {
        get(options, "url")
            .or_else(|| get(options, "DATABASE_URL"))
            .and_then(datasources::provider_from_url)
    }) {
        if !names.iter().any(|existing| existing == &provider) {
            names.push(provider);
        }
    }
    names
}

macro_rules! resolve {
    ($name:expr, $($feat:literal => $ctor:expr),+ $(,)?) => {
        match $name {
            $(
                $feat => {
                    #[cfg(feature = $feat)]
                    {
                        Ok($ctor as Arc<dyn DatasourceFactory>)
                    }
                    #[cfg(not(feature = $feat))]
                    {
                        Err(DatasourceError::not_installed($name))
                    }
                }
            )+
            _ => Err(DatasourceError::not_installed($name)),
        }
    };
}

fn factory_for(name: &str) -> Result<Arc<dyn DatasourceFactory>> {
    resolve!(
        name,
        "json" => datasources_json::JsonDatasource::instance(),
        "memory" => datasources_memory::MemoryDatasource::instance(),
        "sqlite" => datasources_sqlite::SqliteDatasource::instance(),
        "text" => datasources_text::TextDatasource::instance(),
        "xml" => datasources_xml::XmlDatasource::instance(),
        "yaml" => datasources_yaml::YamlDatasource::instance(),
        "mysql" => datasources_mysql::MysqlDatasource::instance(),
        "sqlserver" => datasources_sqlserver::SqlServerDatasource::instance(),
        "postgres" => datasources_postgres::PostgresDatasource::instance(),
        "oracle" => datasources_oracle::OracleDatasource::instance(),
        "mongodb" => datasources_mongodb::MongoDbDatasource::instance(),
        "dynamodb" => datasources_dynamodb::DynamoDbDatasource::instance(),
    )
}

fn install_listed(options: &HashMap<String, String>) -> Result<()> {
    for name in provider_names(options) {
        register(factory_for(&name)?);
    }
    Ok(())
}

pub fn init() {
    bind_install(install_listed);
}

pub fn register_from(options: HashMap<String, String>) -> Result<()> {
    init();
    defaults(options)
}

#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
#[cfg_attr(windows, link_section = ".CRT$XCU")]
static INIT: unsafe extern "C" fn() = {
    unsafe extern "C" fn init_ctor() {
        init();
    }
    init_ctor
};

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::{create, create_default, registered, reset};

    fn clear_env() {
        for key in [
            "INSTALLED_PROVIDERS",
            "providers",
            "DATABASE_URL",
            "url",
            "DATABASE_SCHEMA",
            "provider",
        ] {
            std::env::remove_var(key);
        }
    }

    fn opts(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(key, value)| ((*key).to_string(), (*value).to_string()))
            .collect()
    }

    #[test]
    fn names_from_list_and_default() {
        assert_eq!(
            provider_names(&opts(&[
                ("providers", "sqlite, json, sqlite"),
                ("provider", "postgres"),
            ])),
            vec!["sqlite", "json", "postgres"]
        );
        assert_eq!(
            provider_names(&opts(&[
                ("INSTALLED_PROVIDERS", "json,memory"),
                ("url", "json://"),
            ])),
            vec!["json", "memory"]
        );
        assert_eq!(
            provider_names(&opts(&[("provider", " sqlite ")])),
            vec!["sqlite"]
        );
        assert!(provider_names(&HashMap::new()).is_empty());
    }

    #[test]
    fn registers_listed_factories_and_defaults() {
        reset();
        clear_env();
        register_from(opts(&[
            ("providers", "json,memory"),
            ("provider", "memory"),
        ]))
        .unwrap();
        let mut names = registered();
        names.sort();
        assert_eq!(names, vec!["json", "memory"]);
        assert_eq!(create_default().unwrap().provider(), "memory");
        assert_eq!(create("json", None).unwrap().provider(), "json");
    }

    #[test]
    fn registers_installed_providers() {
        reset();
        clear_env();
        register_from(opts(&[
            ("INSTALLED_PROVIDERS", "json,memory"),
            ("provider", "memory"),
        ]))
        .unwrap();
        let mut names = registered();
        names.sort();
        assert_eq!(names, vec!["json", "memory"]);
        assert_eq!(create_default().unwrap().provider(), "memory");
    }

    #[test]
    fn create_default_reads_env() {
        reset();
        clear_env();
        init();
        std::env::set_var("INSTALLED_PROVIDERS", "json,memory");
        std::env::set_var("provider", "memory");
        let result = create_default();
        clear_env();
        assert_eq!(result.unwrap().provider(), "memory");
    }

    #[test]
    fn missing_feature_or_unknown_name() {
        reset();
        clear_env();
        assert!(matches!(
            register_from(opts(&[("provider", "nope")])),
            Err(DatasourceError::ProviderNotInstalled(_))
        ));
        reset();
        assert!(matches!(
            register_from(opts(&[("provider", "sqlite")])),
            Err(DatasourceError::ProviderNotInstalled(_))
        ));
    }
}
