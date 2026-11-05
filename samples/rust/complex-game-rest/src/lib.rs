mod datasources_providers;
pub use api::Api;
pub use datasources_providers::register;
pub use crate::generated::{
    Category, Contact, ContactSource, Game, GameState, Group, Review, Role, User,
};
pub use crate::generated::Repositories;
pub use crate::generated::repositories::categoriesRepository::categories_repository;
pub use crate::generated::repositories::contactSourcesRepository::contact_sources_repository;
pub use crate::generated::repositories::contactsRepository::ContactsRepository;
pub use crate::generated::repositories::gameStatesRepository::game_states_repository;
pub use crate::generated::repositories::gamesRepository::GamesRepository;
pub use crate::generated::repositories::groupsRepository::groups_repository;
pub use crate::generated::repositories::reviewsRepository::reviews_repository;
pub use crate::generated::repositories::rolesRepository::roles_repository;
pub use crate::generated::repositories::usersRepository::UsersRepository;

#[path = "generated.rs"]
mod generated;

pub mod api;
mod json_object;

#[cfg(test)]
mod api_tests;
