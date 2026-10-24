use crate::crud_repository_builder::CRUDRepositoryBuilder;
use crate::crud_repository_options::CRUDRepositoryOptions;
use crate::mocks::MockConnection;
use crate::repository_builder_options_factory::RepositoryBuilderOptionsFactory;

#[test]
fn for_provider_returns_the_dialect_or_the_default() {
    let mut fallback = CRUDRepositoryOptions::default();
    fallback.create_properties = Some(vec!["name".into()]);
    let mut mysql = CRUDRepositoryOptions::default();
    mysql.create_properties = Some(vec!["id".into(), "name".into()]);
    let mut factory = RepositoryBuilderOptionsFactory::new();
    factory.add(fallback.clone()).add_for("mysql", mysql.clone());
    assert_eq!(
        factory.for_provider("mysql").unwrap().create_properties,
        mysql.create_properties
    );
    assert_eq!(
        factory.for_provider("sqlite").unwrap().create_properties,
        fallback.create_properties
    );
}

#[test]
fn builder_selects_by_connection_provider() {
    let mut fallback = CRUDRepositoryOptions::default();
    fallback.create_properties = Some(vec!["name".into()]);
    let mut mysql = CRUDRepositoryOptions::default();
    mysql.create_properties = Some(vec!["id".into()]);
    let builder = CRUDRepositoryBuilder::new()
        .add(fallback.clone())
        .add_for("mysql", mysql.clone());
    assert_eq!(
        builder
            .for_connection(MockConnection::with_command_provider(
                std::sync::Arc::new(crate::mocks::MockCommand::new()),
                "mysql",
            ).as_ref())
            .unwrap()
            .create_properties,
        mysql.create_properties
    );
    assert_eq!(
        builder
            .for_connection(MockConnection::new().as_ref())
            .unwrap()
            .create_properties,
        fallback.create_properties
    );
}

#[test]
fn for_provider_errors_when_no_options_were_added() {
    let error = match RepositoryBuilderOptionsFactory::new().for_provider("mysql") {
        Ok(_) => panic!("expected error"),
        Err(error) => error,
    };
    assert_eq!(error.to_string(), "repository options required");
}
