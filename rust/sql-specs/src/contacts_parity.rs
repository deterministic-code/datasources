use crate::contact_view::ContactView;
use crate::contacts_join_query::contacts_join_query;
use crate::contacts_service_of::contacts_service_of;
use crate::contacts_view::contacts_view;
use crate::from_enriched_contacts::from_enriched_contacts;
use crate::from_mapped_contacts::from_mapped_contacts;
use crate::repo_root;
use datasources::converters::RowTypeConverter;
use datasources::{
    CRUDRepository, CRUDRepositoryOptions, Command, Connection, ConsoleCommandMiddleware,
    CreateTableStatement, FlatToNestedMapper, IReadonlyRepository, Mapper, Native, TypeConverter, Value,
};
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;

fn quote_ident(id: &str) -> String {
    format!("\"{}\"", id.replace('"', "\"\""))
}

pub fn contact_seed(dialect: &str) -> String {
    fs::read_to_string(repo_root().join(format!("specs/contacts/{dialect}.sql")))
        .unwrap_or_else(|err| panic!("contact seed {dialect}: {err}"))
}

pub fn contact_schema(dialect: &str, render: impl Fn(&CreateTableStatement) -> String) -> String {
    let ddl = crate::schema_tables("contacts", quote_ident, None)
        .iter()
        .map(render)
        .collect::<Vec<_>>()
        .join(";\n");
    format!("{ddl};\n{}", contact_seed(dialect))
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
    cells.entity = "contacts_base".into();
    cells.row_converter = Some(RowTypeConverter::new(Vec::new()));
    let mut join_repo = CRUDRepository::new(connection.clone(), cells).expect("join repo");
    join_repo.add_middleware(middleware.clone());
    let query = contacts_join_query();
    let eager = from_mapped_contacts(&Mapper::map(
        &FlatToNestedMapper::new(contacts_view()),
        join_repo.read(Some(&query), None).await?,
    ));
    let enriched = from_enriched_contacts(
        &contacts_service_of(connection, verbose.then_some(middleware), field)
            .read(None, None)
            .await?,
    );
    Ok((eager, enriched))
}
