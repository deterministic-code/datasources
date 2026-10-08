use datasources::{Datasource, ICRUDRepository, IReadonlyRepository, RepositoryAccess, RepositoryFactory};
use std::sync::Arc;

pub fn register(factory: &RepositoryFactory) {
    factory.register_named(
        "Address",
        "addresses",
        |connection| Arc::new(super::address_repository::address_repository(connection).expect("repository")) as Arc<dyn ICRUDRepository>,
        |connection| Arc::new(super::address_repository::address_readonly_repository(connection).expect("repository")) as Arc<dyn IReadonlyRepository>,
    );
    factory.register_named(
        "AddressType",
        "address_types",
        |connection| Arc::new(super::address_type_repository::address_type_repository(connection).expect("repository")) as Arc<dyn ICRUDRepository>,
        |connection| Arc::new(super::address_type_repository::address_type_readonly_repository(connection).expect("repository")) as Arc<dyn IReadonlyRepository>,
    );
    factory.register_named(
        "Contact",
        "contacts",
        |connection| Arc::new(super::contact_repository::contact_repository(connection).expect("repository")) as Arc<dyn ICRUDRepository>,
        |connection| Arc::new(super::contact_repository::contact_readonly_repository(connection).expect("repository")) as Arc<dyn IReadonlyRepository>,
    );
    factory.register_named(
        "ContactChangeLog",
        "contact_change_logs",
        |connection| Arc::new(super::contact_change_log_repository::contact_change_log_repository(connection).expect("repository")) as Arc<dyn ICRUDRepository>,
        |connection| Arc::new(super::contact_change_log_repository::contact_change_log_readonly_repository(connection).expect("repository")) as Arc<dyn IReadonlyRepository>,
    );
    factory.register_named(
        "ContactGroup",
        "contact_groups",
        |connection| Arc::new(super::contact_group_repository::contact_group_repository(connection).expect("repository")) as Arc<dyn ICRUDRepository>,
        |connection| Arc::new(super::contact_group_repository::contact_group_readonly_repository(connection).expect("repository")) as Arc<dyn IReadonlyRepository>,
    );
    factory.register_named(
        "ContactGroupMember",
        "contact_group_members",
        |connection| Arc::new(super::contact_group_member_repository::contact_group_member_repository(connection).expect("repository")) as Arc<dyn ICRUDRepository>,
        |connection| Arc::new(super::contact_group_member_repository::contact_group_member_readonly_repository(connection).expect("repository")) as Arc<dyn IReadonlyRepository>,
    );
    factory.register_named(
        "ContactPreferences",
        "contact_preferences",
        |connection| Arc::new(super::contact_preferences_repository::contact_preferences_repository(connection).expect("repository")) as Arc<dyn ICRUDRepository>,
        |connection| Arc::new(super::contact_preferences_repository::contact_preferences_readonly_repository(connection).expect("repository")) as Arc<dyn IReadonlyRepository>,
    );
    factory.register_named(
        "ContactSettings",
        "contact_settings",
        |connection| Arc::new(super::contact_settings_repository::contact_settings_repository(connection).expect("repository")) as Arc<dyn ICRUDRepository>,
        |connection| Arc::new(super::contact_settings_repository::contact_settings_readonly_repository(connection).expect("repository")) as Arc<dyn IReadonlyRepository>,
    );
    factory.register_named(
        "ContactSource",
        "contact_sources",
        |connection| Arc::new(super::contact_source_repository::contact_source_repository(connection).expect("repository")) as Arc<dyn ICRUDRepository>,
        |connection| Arc::new(super::contact_source_repository::contact_source_readonly_repository(connection).expect("repository")) as Arc<dyn IReadonlyRepository>,
    );
    factory.register_named(
        "LegacyContact",
        "legacy_contacts",
        |connection| Arc::new(super::legacy_contact_repository::legacy_contact_repository(connection).expect("repository")) as Arc<dyn ICRUDRepository>,
        |connection| Arc::new(super::legacy_contact_repository::legacy_contact_readonly_repository(connection).expect("repository")) as Arc<dyn IReadonlyRepository>,
    );
    factory.register_named(
        "Phone",
        "phones",
        |connection| Arc::new(super::phone_repository::phone_repository(connection).expect("repository")) as Arc<dyn ICRUDRepository>,
        |connection| Arc::new(super::phone_repository::phone_readonly_repository(connection).expect("repository")) as Arc<dyn IReadonlyRepository>,
    );
    factory.register_named(
        "PhoneType",
        "phone_types",
        |connection| Arc::new(super::phone_type_repository::phone_type_repository(connection).expect("repository")) as Arc<dyn ICRUDRepository>,
        |connection| Arc::new(super::phone_type_repository::phone_type_readonly_repository(connection).expect("repository")) as Arc<dyn IReadonlyRepository>,
    );
}

pub trait Repositories {
    fn addresses(&self) -> RepositoryAccess;
    fn address_types(&self) -> RepositoryAccess;
    fn contacts(&self) -> RepositoryAccess;
    fn contact_change_logs(&self) -> RepositoryAccess;
    fn contact_groups(&self) -> RepositoryAccess;
    fn contact_group_members(&self) -> RepositoryAccess;
    fn contact_preferences(&self) -> RepositoryAccess;
    fn contact_settings(&self) -> RepositoryAccess;
    fn contact_sources(&self) -> RepositoryAccess;
    fn legacy_contacts(&self) -> RepositoryAccess;
    fn phones(&self) -> RepositoryAccess;
    fn phone_types(&self) -> RepositoryAccess;
}

impl Repositories for dyn Datasource {
    fn addresses(&self) -> RepositoryAccess {
        self.access("addresses", None)
    }
    fn address_types(&self) -> RepositoryAccess {
        self.access("address_types", None)
    }
    fn contacts(&self) -> RepositoryAccess {
        self.access("contacts", None)
    }
    fn contact_change_logs(&self) -> RepositoryAccess {
        self.access("contact_change_logs", None)
    }
    fn contact_groups(&self) -> RepositoryAccess {
        self.access("contact_groups", None)
    }
    fn contact_group_members(&self) -> RepositoryAccess {
        self.access("contact_group_members", None)
    }
    fn contact_preferences(&self) -> RepositoryAccess {
        self.access("contact_preferences", None)
    }
    fn contact_settings(&self) -> RepositoryAccess {
        self.access("contact_settings", None)
    }
    fn contact_sources(&self) -> RepositoryAccess {
        self.access("contact_sources", None)
    }
    fn legacy_contacts(&self) -> RepositoryAccess {
        self.access("legacy_contacts", None)
    }
    fn phones(&self) -> RepositoryAccess {
        self.access("phones", None)
    }
    fn phone_types(&self) -> RepositoryAccess {
        self.access("phone_types", None)
    }
}
