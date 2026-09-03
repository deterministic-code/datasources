#![allow(dead_code, unused_imports)]

mod address;
mod base;
mod contact;
mod contact_base;
mod contact_change_log;
mod contact_group;
mod contact_group_base;
mod contact_group_member;
mod contact_source;
mod contact_type;
mod contacts_service;
mod import_contact_request;
mod import_contact_result;
mod legacy_contact;
mod legacy_migration_result;
mod phone;

pub use address::Address;
pub use base::Base;
pub use contact::Contact;
pub use contact_base::ContactBase;
pub use contact_change_log::ContactChangeLog;
pub use contact_group::ContactGroup;
pub use contact_group_base::ContactGroupBase;
pub use contact_group_member::ContactGroupMember;
pub use contact_source::ContactSource;
pub use contact_type::ContactType;
pub use contacts_service::ContactsService;
pub use import_contact_request::ImportContactRequest;
pub use import_contact_result::ImportContactResult;
pub use legacy_contact::LegacyContact;
pub use legacy_migration_result::LegacyMigrationResult;
pub use phone::Phone;
