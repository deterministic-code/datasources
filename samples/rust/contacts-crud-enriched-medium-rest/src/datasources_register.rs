#![allow(unused_imports)]
#[path = "../generated/mod.rs"]
mod generated;
pub use generated::Repositories;
pub use generated::{Address, Contact, ContactGroup, LegacyContact, LocalePreferences, Phone};

pub mod repositories {
    pub use super::generated::repositories::*;
    pub use super::generated::repositories::{
        contactGroupsRepository as contact_groups_repository,
        contactsRepository as contacts_repository,
        legacyContactsRepository as legacy_contacts_repository,
    };
}
