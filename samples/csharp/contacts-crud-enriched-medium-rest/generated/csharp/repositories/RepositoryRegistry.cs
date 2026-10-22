using System.Runtime.CompilerServices;
using Datasources;

public static class RepositoryRegistry
{
    [ModuleInitializer]
    internal static void Bind() => RepositoryFactory.Bind(Register);

    public static void Register(RepositoryFactory factory)
    {
        factory.Register("Address", "Addresses", connection => new AddressRepository(connection), connection => AddressRepository.CreateReadonly(connection));
        factory.Register("AddressType", "AddressTypes", connection => new AddressTypeRepository(connection), connection => AddressTypeRepository.CreateReadonly(connection));
        factory.Register("Contact", "Contacts", connection => new ContactRepository(connection), connection => ContactRepository.CreateReadonly(connection));
        factory.Register("ContactChangeLog", "ContactChangeLogs", connection => new ContactChangeLogRepository(connection), connection => ContactChangeLogRepository.CreateReadonly(connection));
        factory.Register("ContactGroup", "ContactGroups", connection => new ContactGroupRepository(connection), connection => ContactGroupRepository.CreateReadonly(connection));
        factory.Register("ContactGroupMember", "ContactGroupMembers", connection => new ContactGroupMemberRepository(connection), connection => ContactGroupMemberRepository.CreateReadonly(connection));
        factory.Register("ContactPreferences", "ContactPreferences", connection => new ContactPreferencesRepository(connection), connection => ContactPreferencesRepository.CreateReadonly(connection));
        factory.Register("ContactSettings", "ContactSettings", connection => new ContactSettingsRepository(connection), connection => ContactSettingsRepository.CreateReadonly(connection));
        factory.Register("ContactSource", "ContactSources", connection => new ContactSourceRepository(connection), connection => ContactSourceRepository.CreateReadonly(connection));
        factory.Register("LegacyContact", "LegacyContacts", connection => new LegacyContactRepository(connection), connection => LegacyContactRepository.CreateReadonly(connection));
        factory.Register("Phone", "Phones", connection => new PhoneRepository(connection), connection => PhoneRepository.CreateReadonly(connection));
        factory.Register("PhoneType", "PhoneTypes", connection => new PhoneTypeRepository(connection), connection => PhoneTypeRepository.CreateReadonly(connection));
    }
}
