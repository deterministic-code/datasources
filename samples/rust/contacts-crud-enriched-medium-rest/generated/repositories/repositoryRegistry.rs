use datasources::{Datasource, ICRUDRepository, IReadonlyRepository, RepositoryFactory};
use super::super::Address;
use super::super::AddressType;
use super::super::Contact;
use super::super::ContactCard;
use super::super::ContactChangeLog;
use super::super::ContactGroup;
use super::super::ContactGroupMember;
use super::super::ContactPreference;
use super::super::ContactSetting;
use super::super::ContactSource;
use super::super::LegacyContact;
use super::super::LocalePreference;
use super::super::Phone;
use super::super::PhoneType;
use std::sync::Arc;

pub fn register(factory: &RepositoryFactory) {
    factory.register_named(
        "Address",
        "addresses",
        |connection| Arc::new(super::addressesRepository::AddressesRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<Address>>,
        |connection| Arc::new(super::addressesRepository::AddressesRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<Address>>,
    );
    factory.register_named(
        "AddressType",
        "address_types",
        |connection| Arc::new(super::addressTypesRepository::AddressTypesRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<AddressType>>,
        |connection| Arc::new(super::addressTypesRepository::AddressTypesRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<AddressType>>,
    );
    factory.register_named(
        "Contact",
        "contacts",
        |connection| Arc::new(super::contactsRepository::ContactsRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<Contact>>,
        |connection| Arc::new(super::contactsRepository::ContactsRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<Contact>>,
    );
    factory.register_named(
        "ContactCard",
        "contact_cards",
        |connection| Arc::new(super::contactCardsRepository::ContactCardsRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<ContactCard>>,
        |connection| Arc::new(super::contactCardsRepository::ContactCardsRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<ContactCard>>,
    );
    factory.register_named(
        "ContactChangeLog",
        "contact_change_logs",
        |connection| Arc::new(super::contactChangeLogsRepository::ContactChangeLogsRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<ContactChangeLog>>,
        |connection| Arc::new(super::contactChangeLogsRepository::ContactChangeLogsRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<ContactChangeLog>>,
    );
    factory.register_named(
        "ContactGroup",
        "contact_groups",
        |connection| Arc::new(super::contactGroupsRepository::ContactGroupsRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<ContactGroup>>,
        |connection| Arc::new(super::contactGroupsRepository::ContactGroupsRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<ContactGroup>>,
    );
    factory.register_named(
        "ContactGroupMember",
        "contact_group_members",
        |connection| Arc::new(super::contactGroupMembersRepository::ContactGroupMembersRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<ContactGroupMember>>,
        |connection| Arc::new(super::contactGroupMembersRepository::ContactGroupMembersRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<ContactGroupMember>>,
    );
    factory.register_named(
        "ContactPreference",
        "contact_preferences",
        |connection| Arc::new(super::contactPreferencesRepository::ContactPreferencesRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<ContactPreference>>,
        |connection| Arc::new(super::contactPreferencesRepository::ContactPreferencesRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<ContactPreference>>,
    );
    factory.register_named(
        "ContactSetting",
        "contact_settings",
        |connection| Arc::new(super::contactSettingsRepository::ContactSettingsRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<ContactSetting>>,
        |connection| Arc::new(super::contactSettingsRepository::ContactSettingsRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<ContactSetting>>,
    );
    factory.register_named(
        "ContactSource",
        "contact_sources",
        |connection| Arc::new(super::contactSourcesRepository::ContactSourcesRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<ContactSource>>,
        |connection| Arc::new(super::contactSourcesRepository::ContactSourcesRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<ContactSource>>,
    );
    factory.register_named(
        "LegacyContact",
        "legacy_contacts",
        |connection| Arc::new(super::legacyContactsRepository::LegacyContactsRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<LegacyContact>>,
        |connection| Arc::new(super::legacyContactsRepository::LegacyContactsRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<LegacyContact>>,
    );
    factory.register_named(
        "LocalePreference",
        "locale_preferences",
        |connection| Arc::new(super::localePreferencesRepository::LocalePreferencesRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<LocalePreference>>,
        |connection| Arc::new(super::localePreferencesRepository::LocalePreferencesRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<LocalePreference>>,
    );
    factory.register_named(
        "Phone",
        "phones",
        |connection| Arc::new(super::phonesRepository::PhonesRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<Phone>>,
        |connection| Arc::new(super::phonesRepository::PhonesRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<Phone>>,
    );
    factory.register_named(
        "PhoneType",
        "phone_types",
        |connection| Arc::new(super::phoneTypesRepository::PhoneTypesRepository::new(connection).expect("repository")) as Arc<dyn ICRUDRepository<PhoneType>>,
        |connection| Arc::new(super::phoneTypesRepository::PhoneTypesRepository::create_readonly(connection).expect("repository")) as Arc<dyn IReadonlyRepository<PhoneType>>,
    );
}

datasources::bind_repositories!(register);

pub trait Repositories {
    fn addresses(&self) -> Arc<super::addressesRepository::AddressesRepository>;
    fn address_types(&self) -> Arc<super::addressTypesRepository::AddressTypesRepository>;
    fn contacts(&self) -> Arc<super::contactsRepository::ContactsRepository>;
    fn contact_cards(&self) -> Arc<super::contactCardsRepository::ContactCardsRepository>;
    fn contact_change_logs(&self) -> Arc<super::contactChangeLogsRepository::ContactChangeLogsRepository>;
    fn contact_groups(&self) -> Arc<super::contactGroupsRepository::ContactGroupsRepository>;
    fn contact_group_members(&self) -> Arc<super::contactGroupMembersRepository::ContactGroupMembersRepository>;
    fn contact_preferences(&self) -> Arc<super::contactPreferencesRepository::ContactPreferencesRepository>;
    fn contact_settings(&self) -> Arc<super::contactSettingsRepository::ContactSettingsRepository>;
    fn contact_sources(&self) -> Arc<super::contactSourcesRepository::ContactSourcesRepository>;
    fn legacy_contacts(&self) -> Arc<super::legacyContactsRepository::LegacyContactsRepository>;
    fn locale_preferences(&self) -> Arc<super::localePreferencesRepository::LocalePreferencesRepository>;
    fn phones(&self) -> Arc<super::phonesRepository::PhonesRepository>;
    fn phone_types(&self) -> Arc<super::phoneTypesRepository::PhoneTypesRepository>;
}

impl Repositories for Arc<dyn Datasource> {
    fn addresses(&self) -> Arc<super::addressesRepository::AddressesRepository> {
        Arc::new(super::addressesRepository::AddressesRepository::new(self.bind_connection(None)).expect("addresses"))
    }
    fn address_types(&self) -> Arc<super::addressTypesRepository::AddressTypesRepository> {
        Arc::new(super::addressTypesRepository::AddressTypesRepository::new(self.bind_connection(None)).expect("address_types"))
    }
    fn contacts(&self) -> Arc<super::contactsRepository::ContactsRepository> {
        Arc::new(super::contactsRepository::ContactsRepository::new(self.bind_connection(None)).expect("contacts"))
    }
    fn contact_cards(&self) -> Arc<super::contactCardsRepository::ContactCardsRepository> {
        Arc::new(super::contactCardsRepository::ContactCardsRepository::new(self.bind_connection(None)).expect("contact_cards"))
    }
    fn contact_change_logs(&self) -> Arc<super::contactChangeLogsRepository::ContactChangeLogsRepository> {
        Arc::new(super::contactChangeLogsRepository::ContactChangeLogsRepository::new(self.bind_connection(None)).expect("contact_change_logs"))
    }
    fn contact_groups(&self) -> Arc<super::contactGroupsRepository::ContactGroupsRepository> {
        Arc::new(super::contactGroupsRepository::ContactGroupsRepository::new(self.bind_connection(None)).expect("contact_groups"))
    }
    fn contact_group_members(&self) -> Arc<super::contactGroupMembersRepository::ContactGroupMembersRepository> {
        Arc::new(super::contactGroupMembersRepository::ContactGroupMembersRepository::new(self.bind_connection(None)).expect("contact_group_members"))
    }
    fn contact_preferences(&self) -> Arc<super::contactPreferencesRepository::ContactPreferencesRepository> {
        Arc::new(super::contactPreferencesRepository::ContactPreferencesRepository::new(self.bind_connection(None)).expect("contact_preferences"))
    }
    fn contact_settings(&self) -> Arc<super::contactSettingsRepository::ContactSettingsRepository> {
        Arc::new(super::contactSettingsRepository::ContactSettingsRepository::new(self.bind_connection(None)).expect("contact_settings"))
    }
    fn contact_sources(&self) -> Arc<super::contactSourcesRepository::ContactSourcesRepository> {
        Arc::new(super::contactSourcesRepository::ContactSourcesRepository::new(self.bind_connection(None)).expect("contact_sources"))
    }
    fn legacy_contacts(&self) -> Arc<super::legacyContactsRepository::LegacyContactsRepository> {
        Arc::new(super::legacyContactsRepository::LegacyContactsRepository::new(self.bind_connection(None)).expect("legacy_contacts"))
    }
    fn locale_preferences(&self) -> Arc<super::localePreferencesRepository::LocalePreferencesRepository> {
        Arc::new(super::localePreferencesRepository::LocalePreferencesRepository::new(self.bind_connection(None)).expect("locale_preferences"))
    }
    fn phones(&self) -> Arc<super::phonesRepository::PhonesRepository> {
        Arc::new(super::phonesRepository::PhonesRepository::new(self.bind_connection(None)).expect("phones"))
    }
    fn phone_types(&self) -> Arc<super::phoneTypesRepository::PhoneTypesRepository> {
        Arc::new(super::phoneTypesRepository::PhoneTypesRepository::new(self.bind_connection(None)).expect("phone_types"))
    }
}
