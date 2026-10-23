use futures_util::TryStreamExt;
use super::contact_columns::join_entity;
use super::contact_view::ContactView;
use super::contacts_join_query::contacts_join_query;
use super::contacts_service_of::contacts_service_of;
use super::contacts_view::contacts_view;
use super::from_enriched_contacts::from_enriched_contacts;
use super::from_mapped_contacts::from_mapped_contacts;
use crate::repo_root;
use datasources::converters::RowTypeConverter;
use datasources::{
    CRUDRepository, CRUDRepositoryOptions, Command, Connection, ConsoleCommandMiddleware,
    FlatToNestedMapper, IReadonlyRepository, Mapper, Native, TypeConverter, Value,
};
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;

pub fn contact_seed(dialect: &str) -> String {
    fs::read_to_string(repo_root().join(format!("specs/contacts/{dialect}.sql")))
        .unwrap_or_else(|err| panic!("contact seed {dialect}: {err}"))
}

pub fn contact_schema(dialect: &str) -> String {
    let dir = repo_root().join("specs/contacts");
    let files = HashMap::from([
        (
            "types.yaml".into(),
            fs::read_to_string(dir.join("types.yaml")).expect("types.yaml"),
        ),
        (
            "datasource.yaml".into(),
            fs::read_to_string(dir.join("datasource.yaml")).expect("datasource.yaml"),
        ),
    ]);
    let settings = HashMap::from([(
        "datasource.pluralize_datatable_names".into(),
        "false".into(),
    )]);
    let ddl = datasources_code_generators::schema_sql(dialect, &files, &settings)
        .unwrap_or_else(|err| panic!("contact schema {dialect}: {err}"));
    format!("{ddl}{}", contact_seed(dialect))
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
    cells.row_converter = Some(RowTypeConverter::new(Vec::new()));
    let join_repo = CRUDRepository::new(connection.clone(), cells).expect("join repo");
    join_repo.add_middleware(middleware.clone());
    let query = contacts_join_query(None);
    let eager = from_mapped_contacts(&Mapper::map(
        &FlatToNestedMapper::new(contacts_view()),
        join_repo.read(Some(&query), None, None).try_collect::<Vec<_>>().await?,
    ));
    let enriched = from_enriched_contacts(
        &contacts_service_of(connection, verbose.then_some(middleware), field)
            .read(None, None, None)
            .try_collect::<Vec<_>>().await?,
    );
    Ok((eager, enriched))
}
