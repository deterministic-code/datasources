using System.Runtime.CompilerServices;
using Datasources;

public static class RepositoryRegistry
{
    [ModuleInitializer]
    internal static void Bind() => RepositoryFactory.Bind(Register);

    public static void Register(RepositoryFactory factory)
    {
        factory.Register("Group", "Groups", connection => new GroupRepository(connection), connection => GroupRepository.CreateReadonly(connection));
        factory.Register("Role", "Roles", connection => new RoleRepository(connection), connection => RoleRepository.CreateReadonly(connection));
        factory.Register("User", "Users", connection => new UserRepository(connection), connection => UserRepository.CreateReadonly(connection));
        factory.Register("UserGroup", "UserGroups", connection => new UserGroupRepository(connection), connection => UserGroupRepository.CreateReadonly(connection));
        factory.Register("UserRole", "UserRoles", connection => new UserRoleRepository(connection), connection => UserRoleRepository.CreateReadonly(connection));
    }
}
