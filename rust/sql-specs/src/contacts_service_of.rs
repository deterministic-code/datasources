use datasources::converters::RowTypeConverter;
use datasources::{
    as_readonly, CommandMiddleware, Connection, DeepCRUDRepository, Repository, TableRepository,
    TableRepositoryBuilderOptions,
};
use std::sync::Arc;

fn columns(names: &[&str]) -> Vec<String> {
    names.iter().map(|name| (*name).to_string()).collect()
}

fn repo(
    connection: Arc<dyn Connection>,
    entity: &str,
    names: &[&str],
    middleware: Option<Arc<dyn CommandMiddleware>>,
) -> Arc<dyn Repository> {
    let mut options = TableRepositoryBuilderOptions::with_factory(connection);
    options.assign_identity_on_add = false;
    options.entity = entity.into();
    options.mapping = Some(RowTypeConverter::new(columns(names)));
    let mut repo = TableRepository::new(options).expect("contacts repo");
    if let Some(middleware) = middleware {
        repo.use_middleware(middleware);
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
        "number",
        "label",
    ];
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
        .add_enriched_repo(
            "addresses",
            "id",
            repo(connection.clone(), "address", &address, middleware.clone()),
            "contact_id",
            None,
            true,
        )
        .add_enriched_repo(
            "phones",
            "id",
            repo(connection, "phone", &phone, middleware),
            "contact_id",
            None,
            true,
        );
    inner
}
