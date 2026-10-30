use futures_util::TryStreamExt;
use super::contact_columns::join_entity;
use super::contact_view::ContactView;
use super::contacts_join_query::contacts_join_query;
use super::contacts_service_of::contacts_service_of;
use super::contacts_view::contacts_view;
use super::from_enriched_contacts::from_enriched_contacts;
use super::from_mapped_contacts::from_mapped_contacts;
use crate::specs_dir;
use datasources::converters::RowTypeConverter;
use datasources::{
    CRUDRepository, CRUDRepositoryOptions, Command, Connection, ConsoleCommandMiddleware,
    FlatToNestedMapper, Mapper, Native, Row, TypeConverter, Value,
};
use datasources_code_generators::SqlGenerateDriver;
use serde_yaml::Value as Yaml;
use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, Once};

pub fn contact_seed(dialect: &str) -> String {
    fs::read_to_string(specs_dir().join(format!("contacts/{dialect}.sql")))
        .unwrap_or_else(|err| panic!("contact seed {dialect}: {err}"))
}

pub fn register_contact_dialect(dialect: &str, driver: SqlGenerateDriver) {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        datasources_code_generators::register_dialect(dialect, driver).unwrap();
    });
}

pub fn contact_schema(dialect: &str) -> String {
    let dir = specs_dir().join("contacts");
    let datasource = fs::read_to_string(dir.join("datasource.yaml")).expect("datasource.yaml");
    let files = HashMap::from([
        (
            "types.yaml".into(),
            fs::read_to_string(dir.join("types.yaml")).expect("types.yaml"),
        ),
        ("datasource.yaml".into(), datasource.clone()),
    ]);
    let ddl = datasources_code_generators::schema_sql(dialect, &files, &settings_of(&datasource))
        .unwrap_or_else(|err| panic!("contact schema {dialect}: {err}"));
    format!("{ddl}{}", contact_seed(dialect))
}

fn settings_of(yaml: &str) -> HashMap<String, String> {
    let parsed: Yaml = serde_yaml::from_str(yaml).expect("datasource.yaml");
    let mut settings = HashMap::new();
    if let Some(root) = parsed.get("settings") {
        flatten(root, "", &mut settings);
    }
    settings
}

fn flatten(value: &Yaml, prefix: &str, settings: &mut HashMap<String, String>) {
    match value {
        Yaml::Mapping(map) => {
            for (key, child) in map {
                let Some(name) = key.as_str() else { continue };
                let path = if prefix.is_empty() {
                    name.to_string()
                } else {
                    format!("{prefix}.{name}")
                };
                flatten(child, &path, settings);
            }
        }
        Yaml::Sequence(items) if !prefix.is_empty() => {
            settings.insert(
                prefix.into(),
                items
                    .iter()
                    .map(|item| match item {
                        Yaml::Bool(flag) => flag.to_string(),
                        Yaml::Number(number) => number.to_string(),
                        Yaml::String(text) => text.clone(),
                        Yaml::Null => "null".into(),
                        _ => String::new(),
                    })
                    .collect::<Vec<_>>()
                    .join(","),
            );
        }
        Yaml::Bool(flag) if !prefix.is_empty() => {
            settings.insert(prefix.into(), flag.to_string());
        }
        Yaml::Number(number) if !prefix.is_empty() => {
            settings.insert(prefix.into(), number.to_string());
        }
        Yaml::String(text) if !prefix.is_empty() => {
            settings.insert(prefix.into(), text.clone());
        }
        Yaml::Null if !prefix.is_empty() => {
            settings.insert(prefix.into(), "null".into());
        }
        _ => {}
    }
}

pub fn statements_of(sql: &str) -> Vec<String> {
    sql.split(';')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(str::to_string)
        .collect()
}

pub async fn apply_sql(command: &dyn Command, statements: &[String]) -> datasources::Result<()> {
    for sql in statements {
        command.execute_native(&Native::new(sql.as_str()), None).await?;
    }
    Ok(())
}

pub async fn contacts_parity(
    connection: Arc<dyn Connection>,
    verbose: bool,
    field: fn(&str) -> Option<Box<dyn TypeConverter<Value, Value> + Send + Sync>>,
) -> datasources::Result<(Vec<ContactView>, Vec<ContactView>)> {
    let mut options = HashMap::new();
    options.insert(
        "isVerbose".into(),
        if verbose { "true" } else { "false" }.into(),
    );
    let middleware = Arc::new(ConsoleCommandMiddleware::new().options(options));
    let mut cells = CRUDRepositoryOptions::default();
    cells.entity_mapper = Some(Arc::new(join_entity()));
        cells.property_mappings = Some(Arc::new(super::contact_columns::columns()));
    cells.row_converter = Some(RowTypeConverter::new(std::collections::HashMap::new()));
    let join_repo = CRUDRepository::<Row>::new(connection.clone(), cells).expect("join repo");
    join_repo.add_middleware(middleware.clone());
    let query = contacts_join_query(None);
    let eager = from_mapped_contacts(&Mapper::map(
        &FlatToNestedMapper::new(contacts_view()),
        join_repo.read(&query).try_collect::<Vec<_>>().await?,
    ));
    let enriched = from_enriched_contacts(
        &contacts_service_of(connection, verbose.then_some(middleware), field)
            .read(&())
            .try_collect::<Vec<_>>().await?,
    );
    Ok((eager, enriched))
}
