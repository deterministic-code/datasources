using Datasources;
using Datasources.Sqlite;
using static Datasources.Cmp;

namespace Datasources.Sqlite.Tests;

public class UserViewCRUDRepositoryTests
{
    static readonly IMapper<string, string> Snake = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);

    sealed class UserView
    {
        public int Id { get; set; }
        public string Username { get; set; } = "";
        public bool IsEnabled { get; set; }
        public int UserTypeId { get; set; }
        public UserType? UserType { get; set; }
        public List<Role>? Roles { get; set; }
        public List<Group>? Groups { get; set; }
    }

    sealed class UserType
    {
        public int Id { get; set; }
        public string Name { get; set; } = "";
    }

    sealed class Role
    {
        public int Id { get; set; }
        public string Name { get; set; } = "";
    }

    sealed class Group
    {
        public int Id { get; set; }
        public string Name { get; set; } = "";
    }

    sealed class UserRole
    {
        public int UserId { get; set; }
        public int RoleId { get; set; }
    }

    sealed class UserGroup
    {
        public int UserId { get; set; }
        public int GroupId { get; set; }
    }

    sealed record Graph(
        CRUDRepository<UserView> UsersRepo,
        CRUDRepository<UserType> UserTypesRepo,
        CRUDRepository<Role> RolesRepo,
        CRUDRepository<Group> GroupsRepo,
        CRUDRepository<UserRole> UserRolesRepo,
        CRUDRepository<UserGroup> UserGroupsRepo,
        UserType AdminType,
        Role AdminRole,
        Role EditorRole,
        Group Engineering,
        Group Ops);

    static async Task<Graph> GraphAsync()
    {
        var ds = new Datasource("sqlite", new SqliteDriver(), new Dictionary<string, string?> { ["filename"] = ":memory:" });
        var connection = await Open(ds, """
            CREATE TABLE users (
                id INTEGER PRIMARY KEY,
                username TEXT NOT NULL,
                is_enabled INTEGER NOT NULL,
                user_type_id INTEGER NOT NULL
            );
            CREATE TABLE user_types (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL
            );
            CREATE TABLE roles (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL
            );
            CREATE TABLE groups (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL
            );
            CREATE TABLE user_roles (
                user_id INTEGER NOT NULL,
                role_id INTEGER NOT NULL,
                PRIMARY KEY (user_id, role_id)
            );
            CREATE TABLE user_groups (
                user_id INTEGER NOT NULL,
                group_id INTEGER NOT NULL,
                PRIMARY KEY (user_id, group_id)
            );
            """);
        var usersRepo = BuildRepo<UserView>(connection, "Users");
        var userTypesRepo = BuildRepo<UserType>(connection, "UserTypes");
        var rolesRepo = BuildRepo<Role>(connection, "Roles");
        var groupsRepo = BuildRepo<Group>(connection, "Groups");
        var userRolesRepo = BuildRepo<UserRole>(connection, "UserRoles");
        var userGroupsRepo = BuildRepo<UserGroup>(connection, "UserGroups");
        return new Graph(
            usersRepo,
            userTypesRepo,
            rolesRepo,
            groupsRepo,
            userRolesRepo,
            userGroupsRepo,
            await userTypesRepo.Create(new UserType { Name = "admin" }),
            await rolesRepo.Create(new Role { Name = "admin" }),
            await rolesRepo.Create(new Role { Name = "editor" }),
            await groupsRepo.Create(new Group { Name = "engineering" }),
            await groupsRepo.Create(new Group { Name = "ops" }));
    }

    static CRUDRepository<T> BuildRepo<T>(IConnection connection, string entity) where T : new() =>
        new(connection, new CRUDRepositoryOptions<T>
        {
            Entity = entity,
            EntityMapper = Snake,
            ColumnMappings = Snake,
            Ids = CreateRepositoryOptions.FromType<T>().Ids,
        });

    static CRUDRepository<UserView> Repo(Graph graph, IWriteEnrichmentStrategy<UserView>? write = null)
    {
        var userViewRepo = new CRUDRepository<UserView>(graph.UsersRepo, write: write);
        userViewRepo.AddEnriched("UserType", "UserTypeId", graph.UserTypesRepo, "Id");
        userViewRepo.AddLinked("Roles", "Id", graph.RolesRepo, "Id", graph.UserRolesRepo, "UserId", "RoleId");
        userViewRepo.AddLinked("Groups", "Id", graph.GroupsRepo, "Id", graph.UserGroupsRepo, "UserId", "GroupId");
        return userViewRepo;
    }

    [Fact]
    public async Task CreateUpdateAndDeleteSyncsLinkRows()
    {
        var ds = new Datasource("sqlite", new SqliteDriver(), new Dictionary<string, string?> { ["filename"] = ":memory:" });
        var connection = await Open(ds, """
            CREATE TABLE users (
                id INTEGER PRIMARY KEY,
                username TEXT NOT NULL,
                is_enabled INTEGER NOT NULL,
                user_type_id INTEGER NOT NULL
            );
            CREATE TABLE user_types (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL
            );
            CREATE TABLE roles (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL
            );
            CREATE TABLE groups (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL
            );
            CREATE TABLE user_roles (
                user_id INTEGER NOT NULL,
                role_id INTEGER NOT NULL,
                PRIMARY KEY (user_id, role_id)
            );
            CREATE TABLE user_groups (
                user_id INTEGER NOT NULL,
                group_id INTEGER NOT NULL,
                PRIMARY KEY (user_id, group_id)
            );
            """);
        var usersRepo = BuildRepo<UserView>(connection, "Users");
        var userTypesRepo = BuildRepo<UserType>(connection, "UserTypes");
        var rolesRepo = BuildRepo<Role>(connection, "Roles");
        var groupsRepo = BuildRepo<Group>(connection, "Groups");
        var userRolesRepo = BuildRepo<UserRole>(connection, "UserRoles");
        var userGroupsRepo = BuildRepo<UserGroup>(connection, "UserGroups");
        var adminType = await userTypesRepo.Create(new UserType { Name = "admin" });
        var adminRole = await rolesRepo.Create(new Role { Name = "admin" });
        var editorRole = await rolesRepo.Create(new Role { Name = "editor" });
        var engineering = await groupsRepo.Create(new Group { Name = "engineering" });
        var ops = await groupsRepo.Create(new Group { Name = "ops" });
        var userViewRepo = new CRUDRepository<UserView>(usersRepo);
        userViewRepo.AddEnriched("UserType", "UserTypeId", userTypesRepo, "Id");
        userViewRepo.AddLinked("Roles", "Id", rolesRepo, "Id", userRolesRepo, "UserId", "RoleId");
        userViewRepo.AddLinked("Groups", "Id", groupsRepo, "Id", userGroupsRepo, "UserId", "GroupId");

        var created = await userViewRepo.Create(new UserView
        {
            Username = "ada",
            IsEnabled = true,
            UserTypeId = adminType.Id,
            Roles = [adminRole, editorRole],
            Groups = [engineering]
        });
        Assert.True(created.Id > 0);
        Assert.Equal("ada", created.Username);
        Assert.Equal(adminType.Id, created.UserTypeId);
        Assert.Equal("admin", created.UserType!.Name);
        Assert.Equal(["admin", "editor"], created.Roles!.Select(role => role.Name).OrderBy(name => name));
        Assert.Equal(
            new[] { adminRole.Id, editorRole.Id }.OrderBy(id => id),
            created.Roles!.Select(role => role.Id).OrderBy(id => id));
        Assert.All(created.Roles!, role => Assert.True(role.Id > 0));
        Assert.Equal(["engineering"], created.Groups!.Select(group => group.Name));
        Assert.Equal([engineering.Id], created.Groups!.Select(group => group.Id));
        Assert.All(created.Groups!, group => Assert.True(group.Id > 0));
        var userRoles = await userRolesRepo.Read().ToListAsync();
        var userGroups = await userGroupsRepo.Read().ToListAsync();
        Assert.Equal([created.Id, created.Id], userRoles.Select(link => link.UserId));
        Assert.Equal(
            new[] { adminRole.Id, editorRole.Id }.OrderBy(id => id),
            userRoles.Select(link => link.RoleId).OrderBy(id => id));
        Assert.Equal((created.Id, engineering.Id), (userGroups[0].UserId, userGroups[0].GroupId));
        Assert.Equal(["admin", "editor"], (await rolesRepo.Read().ToListAsync()).Select(role => role.Name).OrderBy(name => name));
        Assert.Equal("admin", Assert.Single(await userTypesRepo.Read().ToListAsync()).Name);

        await userViewRepo.Update(new UserView
        {
            Id = created.Id,
            Username = "ada",
            IsEnabled = true,
            UserTypeId = adminType.Id,
            Roles = [editorRole],
            Groups = [engineering, ops]
        });
        var updated = await userViewRepo.ReadSingle(new Where("Username", eq, "ada"));
        Assert.Equal(["editor"], updated!.Roles!.Select(role => role.Name));
        Assert.Equal([editorRole.Id], updated.Roles!.Select(role => role.Id));
        Assert.True(updated.Roles![0].Id > 0);
        Assert.Equal(["engineering", "ops"], updated.Groups!.Select(group => group.Name).OrderBy(name => name));
        Assert.Equal(
            new[] { engineering.Id, ops.Id }.OrderBy(id => id),
            updated.Groups!.Select(group => group.Id).OrderBy(id => id));
        Assert.All(updated.Groups!, group => Assert.True(group.Id > 0));
        userRoles = await userRolesRepo.Read().ToListAsync();
        userGroups = await userGroupsRepo.Read().ToListAsync();
        Assert.Equal([editorRole.Id], userRoles.Select(link => link.RoleId));
        Assert.Equal(
            new[] { engineering.Id, ops.Id }.OrderBy(id => id),
            userGroups.Select(link => link.GroupId).OrderBy(id => id));
        Assert.Equal(["admin", "editor"], (await rolesRepo.Read().ToListAsync()).Select(role => role.Name).OrderBy(name => name));
        Assert.Equal(["engineering", "ops"], (await groupsRepo.Read().ToListAsync()).Select(group => group.Name).OrderBy(name => name));

        await userViewRepo.Delete(new UserView { Id = created.Id });
        Assert.Empty(await usersRepo.Read().ToListAsync());
        Assert.Empty(await userRolesRepo.Read().ToListAsync());
        Assert.Empty(await userGroupsRepo.Read().ToListAsync());
        Assert.Equal(["admin", "editor"], (await rolesRepo.Read().ToListAsync()).Select(role => role.Name).OrderBy(name => name));
        Assert.Equal(["engineering", "ops"], (await groupsRepo.Read().ToListAsync()).Select(group => group.Name).OrderBy(name => name));
        Assert.Equal("admin", Assert.Single(await userTypesRepo.Read().ToListAsync()).Name);
    }

    [Fact]
    public async Task LazyCreateAndUpdateSkipUnloadedCollectionsAndDeleteCascades()
    {
        var graph = await GraphAsync();
        var userViewRepo = Repo(graph, new LazyWriteEnrichmentStrategy<UserView>());

        var created = await userViewRepo.Create(new UserView
        {
            Username = "ada",
            IsEnabled = true,
            UserTypeId = graph.AdminType.Id,
            Roles = null,
            Groups = [graph.Engineering]
        });
        Assert.Empty(await graph.UserRolesRepo.Read().ToListAsync());
        var userGroups = await graph.UserGroupsRepo.Read().ToListAsync();
        Assert.Equal((created!.Id, graph.Engineering.Id), (userGroups[0].UserId, userGroups[0].GroupId));

        await userViewRepo.Update(new UserView
        {
            Id = created.Id,
            Username = "ada",
            IsEnabled = true,
            UserTypeId = graph.AdminType.Id,
            Roles = null,
            Groups = null
        });
        Assert.Empty(await graph.UserRolesRepo.Read().ToListAsync());
        userGroups = await graph.UserGroupsRepo.Read().ToListAsync();
        Assert.Equal((created.Id, graph.Engineering.Id), (userGroups[0].UserId, userGroups[0].GroupId));

        await userViewRepo.Update(new UserView
        {
            Id = created.Id,
            Username = "ada",
            IsEnabled = true,
            UserTypeId = graph.AdminType.Id,
            Roles = [graph.EditorRole],
            Groups = null
        });
        Assert.Equal([graph.EditorRole.Id], (await graph.UserRolesRepo.Read().ToListAsync()).Select(link => link.RoleId));
        userGroups = await graph.UserGroupsRepo.Read().ToListAsync();
        Assert.Equal((created.Id, graph.Engineering.Id), (userGroups[0].UserId, userGroups[0].GroupId));
        Assert.Equal(["admin", "editor"], (await graph.RolesRepo.Read().ToListAsync()).Select(role => role.Name).OrderBy(name => name));
        Assert.Equal(["engineering", "ops"], (await graph.GroupsRepo.Read().ToListAsync()).Select(group => group.Name).OrderBy(name => name));

        await userViewRepo.Delete(new UserView { Id = created.Id, Roles = null, Groups = null });
        Assert.Empty(await graph.UsersRepo.Read().ToListAsync());
        Assert.Empty(await graph.UserRolesRepo.Read().ToListAsync());
        Assert.Empty(await graph.UserGroupsRepo.Read().ToListAsync());
        Assert.Equal(["admin", "editor"], (await graph.RolesRepo.Read().ToListAsync()).Select(role => role.Name).OrderBy(name => name));
        Assert.Equal(["engineering", "ops"], (await graph.GroupsRepo.Read().ToListAsync()).Select(group => group.Name).OrderBy(name => name));
        Assert.Equal("admin", Assert.Single(await graph.UserTypesRepo.Read().ToListAsync()).Name);
    }

    static async Task<IConnection> Open(IDatasource ds, string sql)
    {
        var connection = await ds.CreateConnection();
        await connection.Open();
        var command = connection.CreateCommand();
        foreach (var statement in sql.Split(';', StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries))
            await command.Execute(statement);
        return connection;
    }
}
