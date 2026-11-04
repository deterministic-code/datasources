mod datasources_providers;
mod datasources_register;
pub use datasources_providers::register;
pub use datasources_register::{
    Address, Contact, ContactGroup, LegacyContact, LocalePreferences, Phone, Repositories,
};
pub(crate) use datasources_register::repositories;

pub mod api;
pub mod clock;
mod json_object;
pub mod json_row;
pub mod repos;

#[cfg(test)]
mod api_tests;
