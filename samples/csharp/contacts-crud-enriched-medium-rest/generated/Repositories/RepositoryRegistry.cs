using System.Runtime.CompilerServices;
using Datasources;

public static class RepositoryRegistry
{
    [ModuleInitializer]
    internal static void Bind() => RepositoryFactory.Bind(Register);

    public static void Register(RepositoryFactory factory)
    {
        factory.Register("Address", "Addresses", connection => new AddressesRepository(connection), connection => AddressesRepository.CreateReadonly(connection));
        factory.Register("AddressType", "AddressTypes", connection => new AddressTypesRepository(connection), connection => AddressTypesRepository.CreateReadonly(connection));
        factory.Register("Contact", "Contacts", connection => new ContactsRepository(connection), connection => ContactsRepository.CreateReadonly(connection));
        factory.Register("ContactCard", "ContactCards", connection => new ContactCardsRepository(connection), connection => ContactCardsRepository.CreateReadonly(connection));
        factory.Register("ContactChangeLog", "ContactChangeLogs", connection => new ContactChangeLogsRepository(connection), connection => ContactChangeLogsRepository.CreateReadonly(connection));
        factory.Register("ContactGroup", "ContactGroups", connection => new ContactGroupsRepository(connection), connection => ContactGroupsRepository.CreateReadonly(connection));
        factory.Register("ContactGroupMember", "ContactGroupMembers", connection => new ContactGroupMembersRepository(connection), connection => ContactGroupMembersRepository.CreateReadonly(connection));
        factory.Register("ContactPreference", "ContactPreferences", connection => new ContactPreferencesRepository(connection), connection => ContactPreferencesRepository.CreateReadonly(connection));
        factory.Register("ContactSetting", "ContactSettings", connection => new ContactSettingsRepository(connection), connection => ContactSettingsRepository.CreateReadonly(connection));
        factory.Register("ContactSource", "ContactSources", connection => new ContactSourcesRepository(connection), connection => ContactSourcesRepository.CreateReadonly(connection));
        factory.Register("LegacyContact", "LegacyContacts", connection => new LegacyContactsRepository(connection), connection => LegacyContactsRepository.CreateReadonly(connection));
        factory.Register("LocalePreference", "LocalePreferences", connection => new LocalePreferencesRepository(connection), connection => LocalePreferencesRepository.CreateReadonly(connection));
        factory.Register("Phone", "Phones", connection => new PhonesRepository(connection), connection => PhonesRepository.CreateReadonly(connection));
        factory.Register("PhoneType", "PhoneTypes", connection => new PhoneTypesRepository(connection), connection => PhoneTypesRepository.CreateReadonly(connection));
    }
}
