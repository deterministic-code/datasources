mod datasources_providers;
pub use api::Api;
pub use datasources_providers::register;
pub use crate::generated::{
    AddressType, Category, Contact, ContactSource, Game, GameState, Group, PhoneType, Review, Role, User,
};
pub use crate::generated::Repositories;

#[path = "generated.rs"]
mod generated;

pub mod api;
pub mod clock;
mod json_object;

#[cfg(test)]
mod api_tests;
