#[allow(unused_imports, dead_code)]
mod datasources_register;
#[allow(unused_imports)]
use datasources_register::Repositories;
pub(crate) use datasources_register::repositories;

pub mod api;
pub mod clock;
pub mod json_row;
pub mod migrate;
pub mod repos;
pub mod schema;

#[cfg(test)]
mod api_tests;
#[cfg(test)]
mod migrate_tests;
#[cfg(test)]
mod schema_tests;
