use datasources::{CRUDRepository, Datasource, ICRUDRepository, IReadonlyRepository, RepositoryFactory};
use super::super::{Address, AddressType, Contact, ContactCard, ContactChangeLog, ContactGroup, ContactGroupMember, ContactPreferences, ContactSettings, ContactSource, LegacyContact, LocalePreferences, Phone, PhoneType};
use std::sync::Arc;

pub fn register(factory: &RepositoryFactory) {
    factory.register_named(
        "Address",
        "addresses",
        |connection| Arc::new(super::addressRepository::address_repository(connection).expect("repository")) as Arc<dyn ICRUDRepository<Address>>,
        |connection| Arc::new(super::addressRepository::address_readonly_repository(connection).expect("repository")) as Arc<dyn IReadonlyRepository<Address>>,
    );
    factory.register_named(
        "AddressType",
        "address_types",
        |connection| Arc::new(super::addressTypeRepository::address_type_repository(connection).expect("repository")) as Arc<dyn ICRUDRepository<AddressType>>,
        |connection| Arc::new(super::addressTypeRepository::address_type_readonly_repository(connection).expect("repository")) as Arc<dyn IReadonlyRepository<AddressType>>,
    );
    factory.register_named(
        "Contact",
        "contacts",
        |connection| Arc::new(super::contactRepository::contact_repository(connection).expect("repository")) as Arc<dyn ICRUDRepository<Contact>>,
        |connection| Arc::new(super::contactRepository::contact_readonly_repository(connection).expect("repository")) as Arc<dyn IReadonlyRepository<Contact>>,
    );
    factory.register_named(
        "ContactCard",
        "contact_cards",
        |connection| Arc::new(super::contactCardRepository::contact_card_repository(connection).expect("repository")) as Arc<dyn ICRUDRepository<ContactCard>>,
        |connection| Arc::new(super::contactCardRepository::contact_card_readonly_repository(connection).expect("repository")) as Arc<dyn IReadonlyRepository<ContactCard>>,
    );
    factory.register_named(
        "ContactChangeLog",
        "contact_change_logs",
        |connection| Arc::new(super::contactChangeLogRepository::contact_change_log_repository(connection).expect("repository")) as Arc<dyn ICRUDRepository<ContactChangeLog>>,
        |connection| Arc::new(super::contactChangeLogRepository::contact_change_log_readonly_repository(connection).expect("repository")) as Arc<dyn IReadonlyRepository<ContactChangeLog>>,
    );
    factory.register_named(
        "ContactGroup",
        "contact_groups",
        |connection| Arc::new(super::contactGroupRepository::contact_group_repository(connection).expect("repository")) as Arc<dyn ICRUDRepository<ContactGroup>>,
        |connection| Arc::new(super::contactGroupRepository::contact_group_readonly_repository(connection).expect("repository")) as Arc<dyn IReadonlyRepository<ContactGroup>>,
    );
    factory.register_named(
        "ContactGroupMember",
        "contact_group_members",
        |connection| Arc::new(super::contactGroupMemberRepository::contact_group_member_repository(connection).expect("repository")) as Arc<dyn ICRUDRepository<ContactGroupMember>>,
        |connection| Arc::new(super::contactGroupMemberRepository::contact_group_member_readonly_repository(connection).expect("repository")) as Arc<dyn IReadonlyRepository<ContactGroupMember>>,
    );
    factory.register_named(
        "ContactPreferences",
        "contact_preferences",
        |connection| Arc::new(super::contactPreferencesRepository::contact_preferences_repository(connection).expect("repository")) as Arc<dyn ICRUDRepository<ContactPreferences>>,
        |connection| Arc::new(super::contactPreferencesRepository::contact_preferences_readonly_repository(connection).expect("repository")) as Arc<dyn IReadonlyRepository<ContactPreferences>>,
    );
    factory.register_named(
        "ContactSettings",
        "contact_settings",
        |connection| Arc::new(super::contactSettingsRepository::contact_settings_repository(connection).expect("repository")) as Arc<dyn ICRUDRepository<ContactSettings>>,
        |connection| Arc::new(super::contactSettingsRepository::contact_settings_readonly_repository(connection).expect("repository")) as Arc<dyn IReadonlyRepository<ContactSettings>>,
    );
    factory.register_named(
        "ContactSource",
        "contact_sources",
        |connection| Arc::new(super::contactSourceRepository::contact_source_repository(connection).expect("repository")) as Arc<dyn ICRUDRepository<ContactSource>>,
        |connection| Arc::new(super::contactSourceRepository::contact_source_readonly_repository(connection).expect("repository")) as Arc<dyn IReadonlyRepository<ContactSource>>,
    );
    factory.register_named(
        "LegacyContact",
        "legacy_contacts",
        |connection| Arc::new(super::legacyContactRepository::legacy_contact_repository(connection).expect("repository")) as Arc<dyn ICRUDRepository<LegacyContact>>,
        |connection| Arc::new(super::legacyContactRepository::legacy_contact_readonly_repository(connection).expect("repository")) as Arc<dyn IReadonlyRepository<LegacyContact>>,
    );
    factory.register_named(
        "LocalePreferences",
        "locale_preferences",
        |connection| Arc::new(super::localePreferencesRepository::locale_preferences_repository(connection).expect("repository")) as Arc<dyn ICRUDRepository<LocalePreferences>>,
        |connection| Arc::new(super::localePreferencesRepository::locale_preferences_readonly_repository(connection).expect("repository")) as Arc<dyn IReadonlyRepository<LocalePreferences>>,
    );
    factory.register_named(
        "Phone",
        "phones",
        |connection| Arc::new(super::phoneRepository::phone_repository(connection).expect("repository")) as Arc<dyn ICRUDRepository<Phone>>,
        |connection| Arc::new(super::phoneRepository::phone_readonly_repository(connection).expect("repository")) as Arc<dyn IReadonlyRepository<Phone>>,
    );
    factory.register_named(
        "PhoneType",
        "phone_types",
        |connection| Arc::new(super::phoneTypeRepository::phone_type_repository(connection).expect("repository")) as Arc<dyn ICRUDRepository<PhoneType>>,
        |connection| Arc::new(super::phoneTypeRepository::phone_type_readonly_repository(connection).expect("repository")) as Arc<dyn IReadonlyRepository<PhoneType>>,
    );
}

datasources::bind_repositories!(register);

pub trait Repositories {
    fn addresses(&self) -> Arc<CRUDRepository<Address>>;
    fn address_types(&self) -> Arc<CRUDRepository<AddressType>>;
    fn contacts(&self) -> Arc<CRUDRepository<Contact>>;
    fn contact_cards(&self) -> Arc<CRUDRepository<ContactCard>>;
    fn contact_change_logs(&self) -> Arc<CRUDRepository<ContactChangeLog>>;
    fn contact_groups(&self) -> Arc<CRUDRepository<ContactGroup>>;
    fn contact_group_members(&self) -> Arc<CRUDRepository<ContactGroupMember>>;
    fn contact_preferences(&self) -> Arc<CRUDRepository<ContactPreferences>>;
    fn contact_settings(&self) -> Arc<CRUDRepository<ContactSettings>>;
    fn contact_sources(&self) -> Arc<CRUDRepository<ContactSource>>;
    fn legacy_contacts(&self) -> Arc<CRUDRepository<LegacyContact>>;
    fn locale_preferences(&self) -> Arc<CRUDRepository<LocalePreferences>>;
    fn phones(&self) -> Arc<CRUDRepository<Phone>>;
    fn phone_types(&self) -> Arc<CRUDRepository<PhoneType>>;
}

impl Repositories for dyn Datasource {
    fn addresses(&self) -> Arc<CRUDRepository<Address>> {
        Arc::new(super::addressRepository::address_repository(self.bind_connection(None)).expect("addresses"))
    }
    fn address_types(&self) -> Arc<CRUDRepository<AddressType>> {
        Arc::new(super::addressTypeRepository::address_type_repository(self.bind_connection(None)).expect("address_types"))
    }
    fn contacts(&self) -> Arc<CRUDRepository<Contact>> {
        Arc::new(super::contactRepository::contact_repository(self.bind_connection(None)).expect("contacts"))
    }
    fn contact_cards(&self) -> Arc<CRUDRepository<ContactCard>> {
        Arc::new(super::contactCardRepository::contact_card_repository(self.bind_connection(None)).expect("contact_cards"))
    }
    fn contact_change_logs(&self) -> Arc<CRUDRepository<ContactChangeLog>> {
        Arc::new(super::contactChangeLogRepository::contact_change_log_repository(self.bind_connection(None)).expect("contact_change_logs"))
    }
    fn contact_groups(&self) -> Arc<CRUDRepository<ContactGroup>> {
        Arc::new(super::contactGroupRepository::contact_group_repository(self.bind_connection(None)).expect("contact_groups"))
    }
    fn contact_group_members(&self) -> Arc<CRUDRepository<ContactGroupMember>> {
        Arc::new(super::contactGroupMemberRepository::contact_group_member_repository(self.bind_connection(None)).expect("contact_group_members"))
    }
    fn contact_preferences(&self) -> Arc<CRUDRepository<ContactPreferences>> {
        Arc::new(super::contactPreferencesRepository::contact_preferences_repository(self.bind_connection(None)).expect("contact_preferences"))
    }
    fn contact_settings(&self) -> Arc<CRUDRepository<ContactSettings>> {
        Arc::new(super::contactSettingsRepository::contact_settings_repository(self.bind_connection(None)).expect("contact_settings"))
    }
    fn contact_sources(&self) -> Arc<CRUDRepository<ContactSource>> {
        Arc::new(super::contactSourceRepository::contact_source_repository(self.bind_connection(None)).expect("contact_sources"))
    }
    fn legacy_contacts(&self) -> Arc<CRUDRepository<LegacyContact>> {
        Arc::new(super::legacyContactRepository::legacy_contact_repository(self.bind_connection(None)).expect("legacy_contacts"))
    }
    fn locale_preferences(&self) -> Arc<CRUDRepository<LocalePreferences>> {
        Arc::new(super::localePreferencesRepository::locale_preferences_repository(self.bind_connection(None)).expect("locale_preferences"))
    }
    fn phones(&self) -> Arc<CRUDRepository<Phone>> {
        Arc::new(super::phoneRepository::phone_repository(self.bind_connection(None)).expect("phones"))
    }
    fn phone_types(&self) -> Arc<CRUDRepository<PhoneType>> {
        Arc::new(super::phoneTypeRepository::phone_type_repository(self.bind_connection(None)).expect("phone_types"))
    }
}
