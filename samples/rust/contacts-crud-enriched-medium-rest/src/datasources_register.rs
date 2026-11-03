#![allow(unused_imports)]
#[path = "../generated/rust/mod.rs"]
mod generated;
pub use generated::Repositories;
pub use generated::{Address, Contact, ContactGroup, LegacyContact, LocalePreferences, Phone};

pub mod repositories {
    pub use super::generated::repositories::*;
    pub use super::generated::repositories::{
        contactGroupRepository as contact_group_repository,
        contactRepository as contact_repository,
        legacyContactRepository as legacy_contact_repository,
    };
}
