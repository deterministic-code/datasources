use crate::api::Api;
use crate::generated::repositories::categoriesRepository::categories_repository;
use crate::generated::repositories::contactsRepository::ContactsRepository;
use crate::generated::repositories::gameStatesRepository::game_states_repository;
use crate::generated::repositories::gamesRepository::GamesRepository;
use crate::generated::repositories::reviewsRepository::reviews_repository;
use crate::generated::repositories::usersRepository::UsersRepository;
use datasources::Driver;
use std::collections::HashMap;
use std::sync::Arc;

#[tokio::test]
async fn maps_datasource_errors() {
    let mut options = HashMap::new();
    options.insert("DATASOURCE_URL".into(), "sqlite:///:memory:".into());
    let connection = datasources_sqlite::SqliteDriver.connection(Some(&options));
    connection.open().await.unwrap();
    let api = Api::new(
        UsersRepository::new(Arc::clone(&connection)).unwrap(),
        categories_repository(Arc::clone(&connection)).unwrap(),
        GamesRepository::new(Arc::clone(&connection)).unwrap(),
        reviews_repository(Arc::clone(&connection)).unwrap(),
        game_states_repository(Arc::clone(&connection)).unwrap(),
        ContactsRepository::new(connection).unwrap(),
    );
    assert_eq!(api.handle("GET", "/users", None).await.unwrap().status, 400);
}
