using Datasources;
using Datasources.SqlSpecs;
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
        var ds = new Datasource("sqlite", new SqliteDriver(), new Dictionary<string, string?> { ["DATASOURCE_URL"] = "sqlite:///:memory:" });
        var connection = await Open(ds, "user-view-links");
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
            Ids = CreateCRUDRepositoryOptions.FromType<T>().Ids,
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
        var createdRoleLinks = await graph.UserRolesRepo.Read().ToListAsync();
        var createdGroupLinks = await graph.UserGroupsRepo.Read().ToListAsync();
        var saved = await usersRepo.Update(new UserView
        {
            Id = created.Id,
            Username = "ada",
            IsEnabled = true,
            UserTypeId = graph.AdminType.Id,
            Roles = [graph.EditorRole],
            Groups = [graph.Engineering, graph.Ops],
        });
        var updated = await usersRepo.ReadSingleById(created.Id);
        var updatedRoleLinks = await graph.UserRolesRepo.Read().ToListAsync();
        var updatedGroupLinks = await graph.UserGroupsRepo.Read().ToListAsync();
        var removed = await usersRepo.Delete(new UserView { Id = created.Id });

        Assert.True(created.Id > 0);
        Assert.Equal("ada", created.Username);
        Assert.True(created.IsEnabled);
        Assert.Equal(graph.AdminType.Id, created.UserTypeId);
        Assert.Equal("admin", created.UserType!.Name);
        Assert.Equal(["admin", "editor"], created.Roles!.Select(role => role.Name).OrderBy(name => name));
        Assert.Equal(
            new[] { graph.AdminRole.Id, graph.EditorRole.Id }.OrderBy(id => id),
            created.Roles!.Select(role => role.Id).OrderBy(id => id));
        Assert.Equal(["engineering"], created.Groups!.Select(group => group.Name));
        Assert.Equal([graph.Engineering.Id], created.Groups!.Select(group => group.Id));
        Assert.Equal([created.Id, created.Id], createdRoleLinks.Select(link => link.UserId));
        Assert.Equal(
            new[] { graph.AdminRole.Id, graph.EditorRole.Id }.OrderBy(id => id),
            createdRoleLinks.Select(link => link.RoleId).OrderBy(id => id));
        Assert.Equal((created.Id, graph.Engineering.Id), (createdGroupLinks[0].UserId, createdGroupLinks[0].GroupId));
        Assert.Equal(1, saved.RowsAffected);
        Assert.Equal(created.Id, updated!.Id);
        Assert.Equal("ada", updated.Username);
        Assert.True(updated.IsEnabled);
        Assert.Equal(graph.AdminType.Id, updated.UserTypeId);
        Assert.Equal(["editor"], updated.Roles!.Select(role => role.Name));
        Assert.Equal([graph.EditorRole.Id], updated.Roles!.Select(role => role.Id));
        Assert.Equal(["engineering", "ops"], updated.Groups!.Select(group => group.Name).OrderBy(name => name));
        Assert.Equal(
            new[] { graph.Engineering.Id, graph.Ops.Id }.OrderBy(id => id),
            updated.Groups!.Select(group => group.Id).OrderBy(id => id));
        Assert.Equal([created.Id], updatedRoleLinks.Select(link => link.UserId));
        Assert.Equal([graph.EditorRole.Id], updatedRoleLinks.Select(link => link.RoleId));
        Assert.Equal([created.Id, created.Id], updatedGroupLinks.Select(link => link.UserId));
        Assert.Equal(
            new[] { graph.Engineering.Id, graph.Ops.Id }.OrderBy(id => id),
            updatedGroupLinks.Select(link => link.GroupId).OrderBy(id => id));
        Assert.Equal(1, removed.RowsAffected);
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
        var saved = await graph.UsersRepo.Update(new UserView
        {
            Id = created.Id,
            Username = "ada",
            IsEnabled = true,
            UserTypeId = graph.AdminType.Id,
            Roles = null,
            Groups = [graph.Engineering],
        });
        var loaded = await graph.UsersRepo.ReadSingleById(created.Id);
        var userRoles = await graph.UserRolesRepo.Read().ToListAsync();
        var userGroups = await graph.UserGroupsRepo.Read().ToListAsync();
        var roles = await graph.RolesRepo.Read().ToListAsync();
        var groups = await graph.GroupsRepo.Read().ToListAsync();

        Assert.True(created.Id > 0);
        Assert.Equal(1, saved.RowsAffected);
        Assert.Equal(created.Id, loaded!.Id);
        Assert.Equal("ada", loaded.Username);
        Assert.Equal(graph.AdminType.Id, loaded.UserTypeId);
        Assert.Equal("admin", loaded.UserType!.Name);
        Assert.Empty(loaded.Roles!);
        Assert.Equal([graph.Engineering.Id], loaded.Groups!.Select(group => group.Id));
        Assert.Empty(userRoles);
        Assert.Equal([created.Id], userGroups.Select(link => link.UserId));
        Assert.Equal([graph.Engineering.Id], userGroups.Select(link => link.GroupId));
        Assert.Equal(["admin", "editor"], roles.Select(role => role.Name).OrderBy(name => name));
        Assert.Equal(["engineering", "ops"], groups.Select(group => group.Name).OrderBy(name => name));
    }

    [Fact]
    public async Task EmptyListClearsThatAssociation()
    {
        var graph = await GraphAsync();
        var created = await CreateAda(graph, [graph.AdminRole, graph.EditorRole], [graph.Engineering]);
        var saved = await graph.UsersRepo.Update(new UserView
        {
            Id = created.Id,
            Username = "ada",
            IsEnabled = true,
            UserTypeId = graph.AdminType.Id,
            Roles = [],
            Groups = [graph.Engineering],
        });
        var loaded = await graph.UsersRepo.ReadSingleById(created.Id);
        var userRoles = await graph.UserRolesRepo.Read().ToListAsync();
        var userGroups = await graph.UserGroupsRepo.Read().ToListAsync();
        var roles = await graph.RolesRepo.Read().ToListAsync();
        var groups = await graph.GroupsRepo.Read().ToListAsync();

        Assert.True(created.Id > 0);
        Assert.Equal(1, saved.RowsAffected);
        Assert.Equal(created.Id, loaded!.Id);
        Assert.Equal("ada", loaded.Username);
        Assert.Equal(graph.AdminType.Id, loaded.UserTypeId);
        Assert.Equal("admin", loaded.UserType!.Name);
        Assert.Empty(loaded.Roles!);
        Assert.Equal([graph.Engineering.Id], loaded.Groups!.Select(group => group.Id));
        Assert.Empty(userRoles);
        Assert.Equal([created.Id], userGroups.Select(link => link.UserId));
        Assert.Equal([graph.Engineering.Id], userGroups.Select(link => link.GroupId));
        Assert.Equal(["admin", "editor"], roles.Select(role => role.Name).OrderBy(name => name));
        Assert.Equal(["engineering", "ops"], groups.Select(group => group.Name).OrderBy(name => name));
    }

    [Fact]
    public async Task DuplicateRelatedIdsInsertOneRow()
    {
        var graph = await GraphAsync();
        var created = await CreateAda(graph, [graph.AdminRole, graph.AdminRole], [graph.Engineering]);
        var roleLinks = await graph.UserRolesRepo.Read().ToListAsync();
        var groupLinks = await graph.UserGroupsRepo.Read().ToListAsync();
        var roles = await graph.RolesRepo.Read().ToListAsync();

        Assert.True(created.Id > 0);
        Assert.Equal([created.Id], roleLinks.Select(link => link.UserId));
        Assert.Equal([graph.AdminRole.Id], roleLinks.Select(link => link.RoleId));
        Assert.Equal((created.Id, graph.Engineering.Id), (groupLinks[0].UserId, groupLinks[0].GroupId));
        Assert.Equal(["admin", "editor"], roles.Select(role => role.Name).OrderBy(name => name));
    }

    [Fact]
    public async Task MissingRelatedIdThrows()
    {
        var graph = await GraphAsync();
        var usersRepo = Enrich(graph.UsersRepo, graph);
        var error = await Assert.ThrowsAsync<DatasourceException>(() => usersRepo.Create(new UserView
        {
            Username = "ada",
            IsEnabled = true,
            UserTypeId = graph.AdminType.Id,
            Roles = [new Role()],
        }));
        var users = await usersRepo.Read().ToListAsync();
        var roleLinks = await graph.UserRolesRepo.Read().ToListAsync();

        Assert.Contains("Id", error.Message);
        Assert.True(Assert.Single(users).Id > 0);
        Assert.Equal("ada", users[0].Username);
        Assert.Empty(roleLinks);
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
        var createdRoleLinks = await graph.UserRolesRepo.Read().ToListAsync();
        var createdGroupLinks = await graph.UserGroupsRepo.Read().ToListAsync();
        var skipped = await userViewRepo.Update(new UserView
        {
            Id = created.Id,
            Username = "ada",
            IsEnabled = false,
            UserTypeId = graph.AdminType.Id,
            Roles = null,
            Groups = null
        });
        var afterSkip = await userViewRepo.ReadSingleById(created.Id);
        var skippedRoleLinks = await graph.UserRolesRepo.Read().ToListAsync();
        var skippedGroupLinks = await graph.UserGroupsRepo.Read().ToListAsync();
        var linked = await userViewRepo.Update(new UserView
        {
            Id = created.Id,
            Username = "ada",
            IsEnabled = false,
            UserTypeId = graph.AdminType.Id,
            Roles = [graph.EditorRole],
            Groups = null
        });
        var linkedRoleLinks = await graph.UserRolesRepo.Read().ToListAsync();
        var linkedGroupLinks = await graph.UserGroupsRepo.Read().ToListAsync();
        var roles = await graph.RolesRepo.Read().ToListAsync();
        var groups = await graph.GroupsRepo.Read().ToListAsync();
        var removed = await userViewRepo.Delete(new UserView { Id = created.Id, Roles = null, Groups = null });

        Assert.True(created.Id > 0);
        Assert.Empty(createdRoleLinks);
        Assert.Equal((created.Id, graph.Engineering.Id), (createdGroupLinks[0].UserId, createdGroupLinks[0].GroupId));
        Assert.Equal(1, skipped.RowsAffected);
        Assert.Equal(created.Id, afterSkip!.Id);
        Assert.Equal("ada", afterSkip.Username);
        Assert.False(afterSkip.IsEnabled);
        Assert.Empty(skippedRoleLinks);
        Assert.Equal((created.Id, graph.Engineering.Id), (skippedGroupLinks[0].UserId, skippedGroupLinks[0].GroupId));
        Assert.Equal(1, linked.RowsAffected);
        Assert.Equal([created.Id], linkedRoleLinks.Select(link => link.UserId));
        Assert.Equal([graph.EditorRole.Id], linkedRoleLinks.Select(link => link.RoleId));
        Assert.Equal((created.Id, graph.Engineering.Id), (linkedGroupLinks[0].UserId, linkedGroupLinks[0].GroupId));
        Assert.Equal(["admin", "editor"], roles.Select(role => role.Name).OrderBy(name => name));
        Assert.Equal(["engineering", "ops"], groups.Select(group => group.Name).OrderBy(name => name));
        Assert.Equal(1, removed.RowsAffected);
        Assert.Empty(await graph.UsersRepo.Read().ToListAsync());
        Assert.Empty(await graph.UserRolesRepo.Read().ToListAsync());
        Assert.Empty(await graph.UserGroupsRepo.Read().ToListAsync());
        Assert.Equal(["admin", "editor"], (await graph.RolesRepo.Read().ToListAsync()).Select(role => role.Name).OrderBy(name => name));
        Assert.Equal(["engineering", "ops"], (await graph.GroupsRepo.Read().ToListAsync()).Select(group => group.Name).OrderBy(name => name));
        Assert.Equal("admin", Assert.Single(await graph.UserTypesRepo.Read().ToListAsync()).Name);
    }

    static async Task<IConnection> Open(IDatasource ds, string schema)
    {
        var connection = await ds.CreateConnection();
        await connection.Open();
        var command = connection.CreateCommand();
        foreach (var statement in SchemaSpec.Tables(schema, static id => $"\"{id.Replace("\"", "\"\"")}\""))
            await command.Execute(statement);
        return connection;
    }
}
