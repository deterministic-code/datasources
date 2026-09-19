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
    if let Some(value) = get(options, "INSTALLED_PROVIDERS") {
        for name in value.split(',') {
            let name = name.trim();
            if !name.is_empty() && !names.iter().any(|existing| existing == name) {
                names.push(name.to_string());
            }
        }
    }
    if let Some(provider) = get(options, "DATABASE_URL").and_then(datasources::provider_from_url) {
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
    use std::sync::Mutex;

    static ENV: Mutex<()> = Mutex::new(());

    fn clear_env() {
        for key in ["INSTALLED_PROVIDERS", "DATABASE_URL", "DATABASE_SCHEMA"] {
            std::env::remove_var(key);
        }
    }

    fn isolated(test: impl FnOnce()) {
        let _lock = ENV.lock().expect("env");
        reset();
        clear_env();
        test();
        clear_env();
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
                ("INSTALLED_PROVIDERS", "sqlite, json, sqlite"),
                ("DATABASE_URL", "postgres://localhost"),
            ])),
            vec!["sqlite", "json", "postgres"]
        );
        assert_eq!(
            provider_names(&opts(&[
                ("INSTALLED_PROVIDERS", "json,memory"),
                ("DATABASE_URL", "json://"),
            ])),
            vec!["json", "memory"]
        );
        assert_eq!(
            provider_names(&opts(&[("INSTALLED_PROVIDERS", " sqlite ")])),
            vec!["sqlite"]
        );
        assert!(provider_names(&HashMap::new()).is_empty());
    }

    #[test]
    fn registers_listed_factories_and_defaults() {
        isolated(|| {
            register_from(opts(&[
                ("INSTALLED_PROVIDERS", "json,memory"),
                ("DATABASE_URL", "memory://"),
            ]))
            .unwrap();
            let mut names = registered();
            names.sort();
            assert_eq!(names, vec!["json", "memory"]);
            assert_eq!(create_default().unwrap().provider(), "memory");
            assert_eq!(create("json", None).unwrap().provider(), "json");
        });
    }

    #[test]
    fn registers_installed_providers() {
        isolated(|| {
            register_from(opts(&[
                ("INSTALLED_PROVIDERS", "json,memory"),
                ("DATABASE_URL", "memory://"),
            ]))
            .unwrap();
            let mut names = registered();
            names.sort();
            assert_eq!(names, vec!["json", "memory"]);
            assert_eq!(create_default().unwrap().provider(), "memory");
        });
    }

    #[test]
    fn create_default_reads_env() {
        isolated(|| {
            init();
            std::env::set_var("INSTALLED_PROVIDERS", "json,memory");
            std::env::set_var("DATABASE_URL", "memory://");
            assert_eq!(create_default().unwrap().provider(), "memory");
        });
    }

    #[test]
    fn missing_feature_or_unknown_name() {
        isolated(|| {
            assert!(matches!(
                register_from(opts(&[("INSTALLED_PROVIDERS", "nope")])),
                Err(DatasourceError::ProviderNotInstalled(_))
            ));
            reset();
            assert!(matches!(
                register_from(opts(&[("INSTALLED_PROVIDERS", "sqlite")])),
                Err(DatasourceError::ProviderNotInstalled(_))
            ));
        });
    }
}
