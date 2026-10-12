use datasources::{Datasource, ICRUDRepository, IReadonlyRepository, RepositoryFactory};
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

#[ctor::ctor]
fn bind() {
    RepositoryFactory::bind(register);
}

pub trait Repositories {
    fn addresses(&self) -> Arc<dyn ICRUDRepository>;
    fn address_types(&self) -> Arc<dyn ICRUDRepository>;
    fn contacts(&self) -> Arc<dyn ICRUDRepository>;
    fn contact_change_logs(&self) -> Arc<dyn ICRUDRepository>;
    fn contact_groups(&self) -> Arc<dyn ICRUDRepository>;
    fn contact_group_members(&self) -> Arc<dyn ICRUDRepository>;
    fn contact_preferences(&self) -> Arc<dyn ICRUDRepository>;
    fn contact_settings(&self) -> Arc<dyn ICRUDRepository>;
    fn contact_sources(&self) -> Arc<dyn ICRUDRepository>;
    fn legacy_contacts(&self) -> Arc<dyn ICRUDRepository>;
    fn phones(&self) -> Arc<dyn ICRUDRepository>;
    fn phone_types(&self) -> Arc<dyn ICRUDRepository>;
}

impl Repositories for dyn Datasource {
    fn addresses(&self) -> Arc<dyn ICRUDRepository> {
        self.create_repository_by_name("addresses", None).expect("addresses")
    }
    fn address_types(&self) -> Arc<dyn ICRUDRepository> {
        self.create_repository_by_name("address_types", None).expect("address_types")
    }
    fn contacts(&self) -> Arc<dyn ICRUDRepository> {
        self.create_repository_by_name("contacts", None).expect("contacts")
    }
    fn contact_change_logs(&self) -> Arc<dyn ICRUDRepository> {
        self.create_repository_by_name("contact_change_logs", None).expect("contact_change_logs")
    }
    fn contact_groups(&self) -> Arc<dyn ICRUDRepository> {
        self.create_repository_by_name("contact_groups", None).expect("contact_groups")
    }
    fn contact_group_members(&self) -> Arc<dyn ICRUDRepository> {
        self.create_repository_by_name("contact_group_members", None).expect("contact_group_members")
    }
    fn contact_preferences(&self) -> Arc<dyn ICRUDRepository> {
        self.create_repository_by_name("contact_preferences", None).expect("contact_preferences")
    }
    fn contact_settings(&self) -> Arc<dyn ICRUDRepository> {
        self.create_repository_by_name("contact_settings", None).expect("contact_settings")
    }
    fn contact_sources(&self) -> Arc<dyn ICRUDRepository> {
        self.create_repository_by_name("contact_sources", None).expect("contact_sources")
    }
    fn legacy_contacts(&self) -> Arc<dyn ICRUDRepository> {
        self.create_repository_by_name("legacy_contacts", None).expect("legacy_contacts")
    }
    fn phones(&self) -> Arc<dyn ICRUDRepository> {
        self.create_repository_by_name("phones", None).expect("phones")
    }
    fn phone_types(&self) -> Arc<dyn ICRUDRepository> {
        self.create_repository_by_name("phone_types", None).expect("phone_types")
    }
}
