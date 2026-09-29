use datasources::{bind_install, defaults, register, DatasourceError, DatasourceUrlParser, Result};
use std::collections::HashMap;
use std::sync::Arc;

fn get<'a>(options: &'a HashMap<String, String>, key: &str) -> Option<&'a str> {
    options
        .get(key)
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
}

pub fn provider_names(options: &HashMap<String, String>) -> Result<Vec<String>> {
    let mut names = Vec::new();
    if let Some(value) = get(options, "INSTALLED_PROVIDERS") {
        for name in value.split(',') {
            let name = name.trim();
            if !name.is_empty() && !names.iter().any(|existing| existing == name) {
                names.push(name.to_string());
            }
        }
    }
    let Some(url) = get(options, "DATASOURCE_URL") else {
        return Ok(names);
    };
    let provider = DatasourceUrlParser::parse(url)
        .map(|parsed| parsed.provider)
        .ok_or_else(|| DatasourceError::message("DATASOURCE_URL has no provider"))?;
    if !names.iter().any(|existing| existing == &provider) {
        names.push(provider);
    }
    Ok(names)
}

macro_rules! register_named {
    ($name:expr, $($feat:literal => $ty:path),+ $(,)?) => {
        match $name {
            $(
                $feat => {
                    #[cfg(feature = $feat)]
                    {
                        register($feat, Arc::new($ty));
                        Ok(())
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

fn register_named(name: &str) -> Result<()> {
    register_named!(
        name,
        "json" => datasources_json::JsonDriver,
        "memory" => datasources_memory::MemoryDriver,
        "sqlite" => datasources_sqlite::SqliteDriver,
        "xml" => datasources_xml::XmlDriver,
        "yaml" => datasources_yaml::YamlDriver,
        "mysql" => datasources_mysql::MysqlDriver,
        "sqlserver" => datasources_sqlserver::SqlServerDriver,
        "postgres" => datasources_postgres::PostgresDriver,
        "oracle" => datasources_oracle::OracleDriver,
        "mongodb" => datasources_mongodb::MongoDbDriver,
        "dynamodb" => datasources_dynamodb::DynamoDbDriver,
    )
}

fn install_listed(options: &HashMap<String, String>) -> Result<()> {
    for name in provider_names(options)? {
        register_named(&name)?;
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
    use datasources::{create, create_default, default, registered, reset};
    use std::sync::Mutex;

    static ENV: Mutex<()> = Mutex::new(());

    fn clear_env() {
        for key in [
            "INSTALLED_PROVIDERS",
            "DATASOURCE_URL",
            "PROVIDER",
        ] {
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
                ("DATASOURCE_URL", "postgres://localhost"),
            ]))
            .unwrap(),
            vec!["sqlite", "json", "postgres"]
        );
        assert_eq!(
            provider_names(&opts(&[
                ("INSTALLED_PROVIDERS", "json,memory"),
                ("DATASOURCE_URL", "json://"),
            ]))
            .unwrap(),
            vec!["json", "memory"]
        );
        assert_eq!(
            provider_names(&opts(&[("INSTALLED_PROVIDERS", " sqlite ")])).unwrap(),
            vec!["sqlite"]
        );
        assert_eq!(
            provider_names(&opts(&[("DATASOURCE_URL", "postgresql://localhost/db")])).unwrap(),
            vec!["postgresql"]
        );
        assert!(provider_names(&HashMap::new()).unwrap().is_empty());
        assert_eq!(
            provider_names(&opts(&[("DATASOURCE_URL", "not-a-url")]))
                .unwrap_err()
                .to_string(),
            "DATASOURCE_URL has no provider"
        );
    }

    #[test]
    fn registers_listed_factories_and_defaults() {
        isolated(|| {
            register_from(opts(&[
                ("INSTALLED_PROVIDERS", "json,memory"),
                ("DATASOURCE_URL", "memory://"),
            ]))
            .unwrap();
            let mut names = registered();
            names.sort();
            assert_eq!(names, vec!["json", "memory"]);
            assert_eq!(
                create(Some(&opts(&[
                    ("PROVIDER", "memory"),
                    ("DATASOURCE_URL", "memory://"),
                ])))
                .unwrap()
                .provider(),
                "memory"
            );
            assert_eq!(
                create(Some(&opts(&[("PROVIDER", "json")])))
                    .unwrap()
                    .provider(),
                "json"
            );
        });
    }

    #[test]
    fn registers_installed_providers() {
        isolated(|| {
            register_from(opts(&[
                ("INSTALLED_PROVIDERS", "json,memory"),
                ("DATASOURCE_URL", "memory://"),
            ]))
            .unwrap();
            let mut names = registered();
            names.sort();
            assert_eq!(names, vec!["json", "memory"]);
            assert_eq!(
                create(Some(&opts(&[
                    ("PROVIDER", "memory"),
                    ("DATASOURCE_URL", "memory://"),
                ])))
                .unwrap()
                .provider(),
                "memory"
            );
        });
    }

    #[test]
    fn create_default_reads_env() {
        isolated(|| {
            init();
            std::env::set_var("INSTALLED_PROVIDERS", "json,memory");
            std::env::set_var("DATASOURCE_URL", "memory://");
            std::env::set_var("PROVIDER", "memory");
            assert_eq!(create_default().unwrap().provider(), "memory");
            assert_eq!(default().unwrap().provider(), "memory");
            std::env::set_var("PROVIDER", "json");
            std::env::set_var("DATASOURCE_URL", "json://");
            assert_eq!(default().unwrap().provider(), "json");
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

    #[test]
    fn providers_export_only_their_provider() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../providers");
        let mut found = 0;
        for entry in std::fs::read_dir(&root).unwrap() {
            let path = entry.unwrap().path().join("src/lib.rs");
            if !path.is_file() {
                continue;
            }
            found += 1;
            let source = std::fs::read_to_string(&path).unwrap();
            let public: Vec<_> = source
                .lines()
                .map(str::trim)
                .filter(|line| {
                    line.starts_with("pub ")
                        || (line.starts_with("pub(")
                            && !line.starts_with("pub(crate)")
                            && !line.starts_with("pub(super)"))
                })
                .collect();
            assert_eq!(public.len(), 1, "{}: {public:?}", path.display());
            let line = public[0].trim_end_matches(';');
            assert!(
                line.starts_with("pub use ") && line.ends_with("Driver"),
                "{}: {line}",
                path.display()
            );
        }
        assert!(found >= 11, "expected every provider crate, found {found}");
    }
}
