using System.Runtime.CompilerServices;
using Datasources;

public static class RepositoryRegistry
{
    [ModuleInitializer]
    internal static void Bind() => RepositoryFactory.Bind(Register);

    public static void Register(RepositoryFactory factory)
    {
        factory.Register("Group", "Groups", connection => new GroupsRepository(connection), connection => GroupsRepository.CreateReadonly(connection));
        factory.Register("Role", "Roles", connection => new RolesRepository(connection), connection => RolesRepository.CreateReadonly(connection));
        factory.Register("UserBase", "UserBases", connection => new UserBasesRepository(connection), connection => UserBasesRepository.CreateReadonly(connection));
        factory.Register("UserGroup", "UserGroups", connection => new UserGroupsRepository(connection), connection => UserGroupsRepository.CreateReadonly(connection));
        factory.Register("UserRole", "UserRoles", connection => new UserRolesRepository(connection), connection => UserRolesRepository.CreateReadonly(connection));
    }
}
