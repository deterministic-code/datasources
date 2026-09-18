use datasources::converters::RowTypeConverter;
use datasources::{
    as_readonly, CommandMiddleware, Connection, DeepCRUDRepository, DefaultTypeConverters,
    CRUDRepository, Repository, RepositoryBuilderOptions, TypeConverter, Value,
};
use std::collections::HashMap;
use std::sync::Arc;

fn columns(names: &[&str]) -> Vec<String> {
    names.iter().map(|name| (*name).to_string()).collect()
}

fn take(provider: &str, key: &str) -> Option<Box<dyn TypeConverter<Value, Value> + Send + Sync>> {
    DefaultTypeConverters::for_provider(provider).remove(key)
}

fn stamp_fields(
    provider: &str,
) -> HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>> {
    [
        ("uuid", "uuid"),
        ("created", "datetime"),
        ("updated", "datetime"),
        ("version", "binary"),
    ]
    .into_iter()
    .filter_map(|(column, key)| {
        take(provider, key).map(|converter| (column.to_string(), converter))
    })
    .collect()
}

fn repo(
    connection: Arc<dyn Connection>,
    entity: &str,
    names: &[&str],
    middleware: Option<Arc<dyn CommandMiddleware>>,
) -> Arc<dyn CRUDRepository> {
    let mut options = RepositoryBuilderOptions::with_factory(connection.clone());
    options.entity = entity.into();
    options.mapping = Some(
        RowTypeConverter::new(columns(names)).with_fields(stamp_fields(&connection.provider())),
    );
    let mut repo = Repository::new(options).expect("contacts repo");
    if let Some(middleware) = middleware {
        repo.add_middleware(middleware);
    }
    Arc::new(repo)
}

pub fn contacts_service_of(
    connection: Arc<dyn Connection>,
    middleware: Option<Arc<dyn CommandMiddleware>>,
) -> DeepCRUDRepository {
    let base = [
        "id",
        "uuid",
        "created",
        "updated",
        "version",
        "contact_source_id",
        "contact_type_id",
        "first_name",
        "last_name",
        "email",
        "notes",
    ];
    let named = [
        "id",
        "uuid",
        "created",
        "updated",
        "version",
        "name",
        "description",
    ];
    let address = [
        "id",
        "uuid",
        "created",
        "updated",
        "version",
        "contact_id",
        "address_type_id",
        "line1",
        "line2",
        "city",
        "region",
        "postal_code",
        "country",
    ];
    let phone = [
        "id",
        "uuid",
        "created",
        "updated",
        "version",
        "contact_id",
        "phone_type_id",
        "number",
    ];
    let address_types = as_readonly(repo(
        connection.clone(),
        "address_type",
        &named,
        middleware.clone(),
    ));
    let phone_types = as_readonly(repo(
        connection.clone(),
        "phone_type",
        &named,
        middleware.clone(),
    ));
    let mut inner = DeepCRUDRepository::new(repo(
        connection.clone(),
        "contacts_base",
        &base,
        middleware.clone(),
    ));
    inner
        .add_enriched(
            "contact_type_name",
            "contact_type_id",
            as_readonly(repo(
                connection.clone(),
                "contact_type",
                &named,
                middleware.clone(),
            )),
            "id",
            Some("name".into()),
            false,
        )
        .add_enriched(
            "contact_source_name",
            "contact_source_id",
            as_readonly(repo(
                connection.clone(),
                "contact_source",
                &named,
                middleware.clone(),
            )),
            "id",
            Some("name".into()),
            false,
        )
        .add_enriched_repo_with(
            "addresses",
            "id",
            repo(connection.clone(), "address", &address, middleware.clone()),
            "contact_id",
            None,
            true,
            |node| {
                node.include(
                    address_types,
                    "address_type_name",
                    "address_type_id",
                    "id",
                    Some("name".into()),
                    false,
                );
            },
        )
        .add_enriched_repo_with(
            "phones",
            "id",
            repo(connection, "phone", &phone, middleware),
            "contact_id",
            None,
            true,
            |node| {
                node.include(
                    phone_types,
                    "phone_type_name",
                    "phone_type_id",
                    "id",
                    Some("name".into()),
                    false,
                );
            },
        );
    inner
}
