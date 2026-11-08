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
        factory.Register("Category", "Categories", connection => new CategoriesRepository(connection), connection => CategoriesRepository.CreateReadonly(connection));
        factory.Register("Contact", "Contacts", connection => new ContactsRepository(connection), connection => ContactsRepository.CreateReadonly(connection));
        factory.Register("ContactChangeLog", "ContactChangeLogs", connection => new ContactChangeLogsRepository(connection), connection => ContactChangeLogsRepository.CreateReadonly(connection));
        factory.Register("ContactGroup", "ContactGroups", connection => new ContactGroupsRepository(connection), connection => ContactGroupsRepository.CreateReadonly(connection));
        factory.Register("ContactGroupMember", "ContactGroupMembers", connection => new ContactGroupMembersRepository(connection), connection => ContactGroupMembersRepository.CreateReadonly(connection));
        factory.Register("ContactPreference", "ContactPreferences", connection => new ContactPreferencesRepository(connection), connection => ContactPreferencesRepository.CreateReadonly(connection));
        factory.Register("ContactSetting", "ContactSettings", connection => new ContactSettingsRepository(connection), connection => ContactSettingsRepository.CreateReadonly(connection));
        factory.Register("ContactSource", "ContactSources", connection => new ContactSourcesRepository(connection), connection => ContactSourcesRepository.CreateReadonly(connection));
        factory.Register("GameSetting", "GameSettings", connection => new GameSettingsRepository(connection), connection => GameSettingsRepository.CreateReadonly(connection));
        factory.Register("GameState", "GameStates", connection => new GameStatesRepository(connection), connection => GameStatesRepository.CreateReadonly(connection));
        factory.Register("Game", "Games", connection => new GamesRepository(connection), connection => GamesRepository.CreateReadonly(connection));
        factory.Register("GamesCategory", "GamesCategories", connection => new GamesCategoriesRepository(connection), connection => GamesCategoriesRepository.CreateReadonly(connection));
        factory.Register("Group", "Groups", connection => new GroupsRepository(connection), connection => GroupsRepository.CreateReadonly(connection));
        factory.Register("LegacyContact", "LegacyContacts", connection => new LegacyContactsRepository(connection), connection => LegacyContactsRepository.CreateReadonly(connection));
        factory.Register("Phone", "Phones", connection => new PhonesRepository(connection), connection => PhonesRepository.CreateReadonly(connection));
        factory.Register("PhoneType", "PhoneTypes", connection => new PhoneTypesRepository(connection), connection => PhoneTypesRepository.CreateReadonly(connection));
        factory.Register("Review", "Reviews", connection => new ReviewsRepository(connection), connection => ReviewsRepository.CreateReadonly(connection));
        factory.Register("Role", "Roles", connection => new RolesRepository(connection), connection => RolesRepository.CreateReadonly(connection));
        factory.Register("User", "Users", connection => new UsersRepository(connection), connection => UsersRepository.CreateReadonly(connection));
        factory.Register("UserGroup", "UserGroups", connection => new UserGroupsRepository(connection), connection => UserGroupsRepository.CreateReadonly(connection));
        factory.Register("UserRole", "UserRoles", connection => new UserRolesRepository(connection), connection => UserRolesRepository.CreateReadonly(connection));
        factory.Register("UserSetting", "UserSettings", connection => new UserSettingsRepository(connection), connection => UserSettingsRepository.CreateReadonly(connection));
    }
}
