use crate::api::Api;
use crate::clock::Clock;
use crate::generated::repositories::categoriesRepository::categories_repository;
use crate::generated::repositories::contactRepository::contact_repository;
use crate::generated::repositories::gameStatesRepository::game_states_repository;
use crate::generated::repositories::gamesRepository::games_repository;
use crate::generated::repositories::reviewsRepository::reviews_repository;
use crate::generated::repositories::usersRepository::users_repository;
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
        Arc::new(users_repository(Arc::clone(&connection)).unwrap()),
        Arc::new(categories_repository(Arc::clone(&connection)).unwrap()),
        Arc::new(games_repository(Arc::clone(&connection)).unwrap()),
        Arc::new(reviews_repository(Arc::clone(&connection)).unwrap()),
        Arc::new(game_states_repository(Arc::clone(&connection)).unwrap()),
        Arc::new(contact_repository(connection).unwrap()),
        Clock::new(),
    );
    assert_eq!(api.handle("GET", "/users", None).await.unwrap().status, 400);
}

#[tokio::test]
async fn clock_defaults_cover_now() {
    let _ = Clock::new().now();
    let _ = Clock::default().uuid();
}
