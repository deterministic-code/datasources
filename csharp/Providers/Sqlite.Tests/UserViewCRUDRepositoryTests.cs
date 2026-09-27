using Datasources;
using Datasources.Sqlite;

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

    static async Task<Graph> GraphAsync(IWriteEnrichmentStrategy<UserView>? write = null)
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
        var usersRepo = BuildRepo<UserView>(connection, "Users", write);
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

    static CRUDRepository<T> BuildRepo<T>(IConnection connection, string entity, IWriteEnrichmentStrategy<T>? write = null) where T : new() =>
        new(connection, new CRUDRepositoryOptions<T>
        {
            Entity = entity,
            EntityMapper = Snake,
            ColumnMappings = Snake,
            Ids = CreateRepositoryOptions.FromType<T>().Ids,
            Write = write,
        });

    static CRUDRepository<UserView> Enrich(CRUDRepository<UserView> users, Graph graph)
    {
        users.AddEnriched(new("UserType", "UserTypeId", "Id"), graph.UserTypesRepo);
        users.AddLinked(new("Roles", "Id", "Id", Association: new("UserId", "RoleId")), graph.RolesRepo, graph.UserRolesRepo);
        users.AddLinked(new("Groups", "Id", "Id", Association: new("UserId", "GroupId")), graph.GroupsRepo, graph.UserGroupsRepo);
        return users;
    }

    static async Task<UserView> CreateAda(Graph graph, List<Role>? roles, List<Group>? groups)
    {
        var usersRepo = Enrich(graph.UsersRepo, graph);
        return await usersRepo.Create(new UserView
        {
            Username = "ada",
            IsEnabled = true,
            UserTypeId = graph.AdminType.Id,
            Roles = roles,
            Groups = groups,
        });
    }

    [Fact]
    public async Task CreateUpdateAndDeleteSyncsLinkRows()
    {
        var graph = await GraphAsync();
        var usersRepo = graph.UsersRepo;
        usersRepo.AddEnriched(new("UserType", "UserTypeId", "Id"), graph.UserTypesRepo);
        usersRepo.AddLinked(new("Roles", "Id", "Id", Association: new("UserId", "RoleId")), graph.RolesRepo, graph.UserRolesRepo);
        usersRepo.AddLinked(new("Groups", "Id", "Id", Association: new("UserId", "GroupId")), graph.GroupsRepo, graph.UserGroupsRepo);
        var created = await usersRepo.Create(new UserView
        {
            Username = "ada",
            IsEnabled = true,
            UserTypeId = graph.AdminType.Id,
            Roles = [graph.AdminRole, graph.EditorRole],
            Groups = [graph.Engineering],
        });
        await usersRepo.Update(new UserView
        {
            Id = created.Id,
            Username = "ada",
            IsEnabled = true,
            UserTypeId = graph.AdminType.Id,
            Roles = [graph.EditorRole],
            Groups = [graph.Engineering, graph.Ops],
        });
        var updated = await usersRepo.ReadSingleById(created.Id);
        await usersRepo.Delete(new UserView { Id = created.Id });

        Assert.True(created.Id > 0);
        Assert.Equal("admin", created.UserType!.Name);
        Assert.Equal(["admin", "editor"], created.Roles!.Select(role => role.Name).OrderBy(name => name));
        Assert.Equal(
            new[] { graph.AdminRole.Id, graph.EditorRole.Id }.OrderBy(id => id),
            created.Roles!.Select(role => role.Id).OrderBy(id => id));
        Assert.Equal(["engineering"], created.Groups!.Select(group => group.Name));
        Assert.Equal([graph.Engineering.Id], created.Groups!.Select(group => group.Id));
        Assert.Equal(["editor"], updated!.Roles!.Select(role => role.Name));
        Assert.Equal([graph.EditorRole.Id], updated.Roles!.Select(role => role.Id));
        Assert.Equal(["engineering", "ops"], updated.Groups!.Select(group => group.Name).OrderBy(name => name));
        Assert.Equal(
            new[] { graph.Engineering.Id, graph.Ops.Id }.OrderBy(id => id),
            updated.Groups!.Select(group => group.Id).OrderBy(id => id));
        Assert.Empty(await usersRepo.Read().ToListAsync());
        Assert.Empty(await graph.UserRolesRepo.Read().ToListAsync());
        Assert.Empty(await graph.UserGroupsRepo.Read().ToListAsync());
        Assert.Equal(["admin", "editor"], (await graph.RolesRepo.Read().ToListAsync()).Select(role => role.Name).OrderBy(name => name));
        Assert.Equal(["engineering", "ops"], (await graph.GroupsRepo.Read().ToListAsync()).Select(group => group.Name).OrderBy(name => name));
        Assert.Equal("admin", Assert.Single(await graph.UserTypesRepo.Read().ToListAsync()).Name);
    }

    [Fact]
    public async Task NullListClearsThatAssociation()
    {
        var graph = await GraphAsync();
        var created = await CreateAda(graph, [graph.AdminRole, graph.EditorRole], [graph.Engineering]);
        await graph.UsersRepo.Update(new UserView
        {
            Id = created.Id,
            Username = "ada",
            IsEnabled = true,
            UserTypeId = graph.AdminType.Id,
            Roles = null,
            Groups = [graph.Engineering],
        });
        Assert.Empty(await graph.UserRolesRepo.Read().ToListAsync());
        var userGroups = await graph.UserGroupsRepo.Read().ToListAsync();
        Assert.Equal((created.Id, graph.Engineering.Id), (userGroups[0].UserId, userGroups[0].GroupId));
    }

    [Fact]
    public async Task EmptyListClearsThatAssociation()
    {
        var graph = await GraphAsync();
        var created = await CreateAda(graph, [graph.AdminRole, graph.EditorRole], [graph.Engineering]);
        await graph.UsersRepo.Update(new UserView
        {
            Id = created.Id,
            Username = "ada",
            IsEnabled = true,
            UserTypeId = graph.AdminType.Id,
            Roles = [],
            Groups = [graph.Engineering],
        });
        Assert.Empty(await graph.UserRolesRepo.Read().ToListAsync());
        var userGroups = await graph.UserGroupsRepo.Read().ToListAsync();
        Assert.Equal((created.Id, graph.Engineering.Id), (userGroups[0].UserId, userGroups[0].GroupId));
    }

    [Fact]
    public async Task DuplicateRelatedIdsInsertOneRow()
    {
        var graph = await GraphAsync();
        var created = await CreateAda(graph, [graph.AdminRole, graph.AdminRole], [graph.Engineering]);
        var links = await graph.UserRolesRepo.Read().ToListAsync();
        Assert.Equal([created.Id], links.Select(link => link.UserId));
        Assert.Equal([graph.AdminRole.Id], links.Select(link => link.RoleId));
    }

    [Fact]
    public async Task MissingRelatedIdThrows()
    {
        var graph = await GraphAsync();
        var usersRepo = Enrich(graph.UsersRepo, graph);
        await Assert.ThrowsAsync<DatasourceException>(() => usersRepo.Create(new UserView
        {
            Username = "ada",
            IsEnabled = true,
            UserTypeId = graph.AdminType.Id,
            Roles = [new Role()],
        }));
    }

    [Fact]
    public async Task LazyCreateAndUpdateSkipUnloadedCollectionsAndDeleteCascades()
    {
        var graph = await GraphAsync(new LazyWriteEnrichmentStrategy<UserView>());
        var userViewRepo = Enrich(graph.UsersRepo, graph);

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
