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
        CRUDRepository<UserView> Users,
        CRUDRepository<UserType> UserTypes,
        CRUDRepository<Role> Roles,
        CRUDRepository<Group> Groups,
        CRUDRepository<UserRole> UserRoles,
        CRUDRepository<UserGroup> UserGroups,
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
        var users = Table<UserView>(connection, "Users");
        var userTypes = Table<UserType>(connection, "UserTypes");
        var roles = Table<Role>(connection, "Roles");
        var groups = Table<Group>(connection, "Groups");
        var userRoles = Table<UserRole>(connection, "UserRoles");
        var userGroups = Table<UserGroup>(connection, "UserGroups");
        return new Graph(
            users,
            userTypes,
            roles,
            groups,
            userRoles,
            userGroups,
            await Inserted(userTypes, new UserType { Name = "admin" }, "admin"),
            await Inserted(roles, new Role { Name = "admin" }, "admin"),
            await Inserted(roles, new Role { Name = "editor" }, "editor"),
            await Inserted(groups, new Group { Name = "engineering" }, "engineering"),
            await Inserted(groups, new Group { Name = "ops" }, "ops"));
    }

    static CRUDRepository<T> Table<T>(IConnection connection, string entity) where T : new() =>
        new(connection, new CRUDRepositoryOptions<T>
        {
            Entity = entity,
            EntityMapper = Snake,
            ColumnMappings = Snake,
            Ids = CreateRepositoryOptions.FromType<T>().Ids,
        });

    static async Task<T> Inserted<T>(CRUDRepository<T> repo, T item, string name) where T : new()
    {
        await repo.Create(item);
        return await repo.ReadSingle(new WhereClause("Name", eq, name))
            ?? throw new InvalidOperationException($"{typeof(T).Name} {name} was not stored");
    }

    static CRUDRepository<UserView> Repo(Graph graph, IWriteEnrichmentStrategy<UserView>? write = null)
    {
        var repo = new CRUDRepository<UserView>(graph.Users, write: write);
        repo.AddEnriched("UserType", "UserTypeId", graph.UserTypes, "Id");
        repo.AddLinked("Roles", "Id", graph.Roles, "Id", graph.UserRoles, "UserId", "RoleId");
        repo.AddLinked("Groups", "Id", graph.Groups, "Id", graph.UserGroups, "UserId", "GroupId");
        return repo;
    }

    static async Task<List<T>> All<T>(CRUDRepository<T> repo) where T : new()
    {
        var items = new List<T>();
        await foreach (var item in repo.Read())
            items.Add(item);
        return items;
    }

    [Fact]
    public async Task CreateUpdateAndDeleteSyncsLinkRows()
    {
        var graph = await GraphAsync();
        var repo = Repo(graph);

        var created = await repo.Create(new UserView
        {
            Username = "ada",
            IsEnabled = true,
            UserTypeId = graph.AdminType.Id,
            Roles = [graph.AdminRole, graph.EditorRole],
            Groups = [graph.Engineering]
        });
        Assert.True(created.Id > 0);
        Assert.Equal("ada", created.Username);
        Assert.Equal(graph.AdminType.Id, created.UserTypeId);
        Assert.Equal("admin", created.UserType!.Name);
        Assert.Equal(["admin", "editor"], created.Roles!.Select(role => role.Name).OrderBy(name => name));
        Assert.Equal(
            new[] { graph.AdminRole.Id, graph.EditorRole.Id }.OrderBy(id => id),
            created.Roles!.Select(role => role.Id).OrderBy(id => id));
        Assert.All(created.Roles!, role => Assert.True(role.Id > 0));
        Assert.Equal(["engineering"], created.Groups!.Select(group => group.Name));
        Assert.Equal([graph.Engineering.Id], created.Groups!.Select(group => group.Id));
        Assert.All(created.Groups!, group => Assert.True(group.Id > 0));
        var userRoles = await All(graph.UserRoles);
        var userGroups = await All(graph.UserGroups);
        Assert.Equal([created.Id, created.Id], userRoles.Select(link => link.UserId));
        Assert.Equal(
            new[] { graph.AdminRole.Id, graph.EditorRole.Id }.OrderBy(id => id),
            userRoles.Select(link => link.RoleId).OrderBy(id => id));
        Assert.Equal((created.Id, graph.Engineering.Id), (userGroups[0].UserId, userGroups[0].GroupId));
        Assert.Equal(["admin", "editor"], (await All(graph.Roles)).Select(role => role.Name).OrderBy(name => name));
        Assert.Equal("admin", Assert.Single(await All(graph.UserTypes)).Name);

        await repo.Update(new UserView
        {
            Id = created.Id,
            Username = "ada",
            IsEnabled = true,
            UserTypeId = graph.AdminType.Id,
            Roles = [graph.EditorRole],
            Groups = [graph.Engineering, graph.Ops]
        });
        var updated = await repo.ReadSingle(new WhereClause("Username", eq, "ada"));
        Assert.Equal(["editor"], updated!.Roles!.Select(role => role.Name));
        Assert.Equal([graph.EditorRole.Id], updated.Roles!.Select(role => role.Id));
        Assert.True(updated.Roles![0].Id > 0);
        Assert.Equal(["engineering", "ops"], updated.Groups!.Select(group => group.Name).OrderBy(name => name));
        Assert.Equal(
            new[] { graph.Engineering.Id, graph.Ops.Id }.OrderBy(id => id),
            updated.Groups!.Select(group => group.Id).OrderBy(id => id));
        Assert.All(updated.Groups!, group => Assert.True(group.Id > 0));
        userRoles = await All(graph.UserRoles);
        userGroups = await All(graph.UserGroups);
        Assert.Equal([graph.EditorRole.Id], userRoles.Select(link => link.RoleId));
        Assert.Equal(
            new[] { graph.Engineering.Id, graph.Ops.Id }.OrderBy(id => id),
            userGroups.Select(link => link.GroupId).OrderBy(id => id));
        Assert.Equal(["admin", "editor"], (await All(graph.Roles)).Select(role => role.Name).OrderBy(name => name));
        Assert.Equal(["engineering", "ops"], (await All(graph.Groups)).Select(group => group.Name).OrderBy(name => name));

        await repo.Delete(new UserView { Id = created.Id });
        Assert.Empty(await All(graph.Users));
        Assert.Empty(await All(graph.UserRoles));
        Assert.Empty(await All(graph.UserGroups));
        Assert.Equal(["admin", "editor"], (await All(graph.Roles)).Select(role => role.Name).OrderBy(name => name));
        Assert.Equal(["engineering", "ops"], (await All(graph.Groups)).Select(group => group.Name).OrderBy(name => name));
        Assert.Equal("admin", Assert.Single(await All(graph.UserTypes)).Name);
    }

    [Fact]
    public async Task LazyCreateAndUpdateSkipUnloadedCollectionsAndDeleteCascades()
    {
        var graph = await GraphAsync();
        var repo = Repo(graph, new LazyWriteEnrichmentStrategy<UserView>());

        var created = await repo.Create(new UserView
        {
            Username = "ada",
            IsEnabled = true,
            UserTypeId = graph.AdminType.Id,
            Roles = null,
            Groups = [graph.Engineering]
        });
        Assert.Empty(await All(graph.UserRoles));
        var userGroups = await All(graph.UserGroups);
        Assert.Equal((created!.Id, graph.Engineering.Id), (userGroups[0].UserId, userGroups[0].GroupId));

        await repo.Update(new UserView
        {
            Id = created.Id,
            Username = "ada",
            IsEnabled = true,
            UserTypeId = graph.AdminType.Id,
            Roles = null,
            Groups = null
        });
        Assert.Empty(await All(graph.UserRoles));
        userGroups = await All(graph.UserGroups);
        Assert.Equal((created.Id, graph.Engineering.Id), (userGroups[0].UserId, userGroups[0].GroupId));

        await repo.Update(new UserView
        {
            Id = created.Id,
            Username = "ada",
            IsEnabled = true,
            UserTypeId = graph.AdminType.Id,
            Roles = [graph.EditorRole],
            Groups = null
        });
        Assert.Equal([graph.EditorRole.Id], (await All(graph.UserRoles)).Select(link => link.RoleId));
        userGroups = await All(graph.UserGroups);
        Assert.Equal((created.Id, graph.Engineering.Id), (userGroups[0].UserId, userGroups[0].GroupId));
        Assert.Equal(["admin", "editor"], (await All(graph.Roles)).Select(role => role.Name).OrderBy(name => name));
        Assert.Equal(["engineering", "ops"], (await All(graph.Groups)).Select(group => group.Name).OrderBy(name => name));

        await repo.Delete(new UserView { Id = created.Id, Roles = null, Groups = null });
        Assert.Empty(await All(graph.Users));
        Assert.Empty(await All(graph.UserRoles));
        Assert.Empty(await All(graph.UserGroups));
        Assert.Equal(["admin", "editor"], (await All(graph.Roles)).Select(role => role.Name).OrderBy(name => name));
        Assert.Equal(["engineering", "ops"], (await All(graph.Groups)).Select(group => group.Name).OrderBy(name => name));
        Assert.Equal("admin", Assert.Single(await All(graph.UserTypes)).Name);
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
