using Datasources;
using Datasources.SqlSpecs;
using Datasources.Sqlite;
using static Datasources.Cmp;

namespace Datasources.Sqlite.Tests;

public class ReadonlyRepositoryIntegrationTests
{
    public ReadonlyRepositoryIntegrationTests()
    {
        DatasourceFactory.Reset();
        Environment.SetEnvironmentVariable("PROVIDER", "sqlite");
        Environment.SetEnvironmentVariable("DATASOURCE_URL", "sqlite:///:memory:");
        Environment.SetEnvironmentVariable("INSTALLED_PROVIDERS", "sqlite");
    }

    static readonly Guid Ada = Guid.Parse("11111111-1111-1111-1111-111111111111");
    static readonly Guid Grace = Guid.Parse("22222222-2222-2222-2222-222222222222");
    static readonly Guid Lin = Guid.Parse("33333333-3333-3333-3333-333333333333");

    sealed class User
    {
        public long Id { get; set; }
        public string Username { get; set; } = "";
        public bool IsEnabled { get; set; }
    }

    static IMapper<string, string> Snake()
    {
        return new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
    }

    static IMapper<string, string> SnakeThenPlural()
    {
        return new TransformMapper<string, string>(
            name => new PluralizerMapper().Map(Snake().Map(name)),
            name => Snake().ReverseMap(new PluralizerMapper().ReverseMap(name)));
    }

    sealed class UserAccount
    {
        public Guid Key { get; set; }
        public string Login { get; set; } = "";
        public bool Active { get; set; }
        public DateTime Opened { get; set; }
    }

    [Fact]
    public async Task ReadsUsersByIdAndByUsernameAndEnabled()
    {
        var ds = Datasource.Default;
        var connection = await Open(ds);
        await SchemaSpec.Apply(connection, "minimal-users");
        await Execute(connection, "INSERT INTO users (id, username, is_enabled) VALUES (1, 'ada', 1)");
        await Execute(connection, "INSERT INTO users (id, username, is_enabled) VALUES (2, 'grace', 0)");
        await Execute(connection, "INSERT INTO users (id, username, is_enabled) VALUES (3, 'bea', 1)");
        var repo = ds.CreateReadonlyRepository<User>(connection, new ReadonlyRepositoryMappingOptions<User>
        {
            EntityMapper = SnakeThenPlural(),
            PropertyMappings = Snake(),
        });

        var byId = await repo.ReadSingle(new Where("Id", eq, 1));
        var row = await repo.Read(new Where("Username", eq, "grace").And("IsEnabled", eq, 0)).SingleAsync();

        Assert.Equal("ada", byId!.Username);
        Assert.True(byId.IsEnabled);
        Assert.Equal(2L, row.Id);
        Assert.Equal("grace", row.Username);
        Assert.False(row.IsEnabled);
    }

    sealed class UserConverter : TypeConverter<User, Row>
    {
        public override User Convert(Row from) => new()
        {
            Id = (long)from["Id"].ToObject()!,
            Username = (string)from["Username"].ToObject()!,
            IsEnabled = (bool)from["IsEnabled"].ToObject()!
        };

        public override Row ReverseConvert(User to) => new()
        {
            ["Id"] = to.Id,
            ["Username"] = to.Username,
            ["IsEnabled"] = to.IsEnabled
        };
    }

    [Fact]
    public async Task MapsAnObjectExplicitly()
    {
        var driver = new SqliteDriver();
        var ds = new Datasource("sqlite", driver, new Dictionary<string, string?> { ["DATASOURCE_URL"] = "sqlite:///:memory:" });
        var connection = await Open(ds);
        await SchemaSpec.Apply(connection, "minimal-users");
        await Execute(connection, """
            INSERT INTO users (id, username, is_enabled) VALUES
                (1, 'ada', 1),
                (2, 'grace', 0);
            """);
        var usersRepo = ds.CreateReadonlyRepository<User>(connection, new ReadonlyRepositoryMappingOptions<User>
        {
            EntityMapper = SnakeThenPlural(),
            PropertyMappings = Snake(),
            ObjectConverter = new UserConverter()
        });

        var user = await usersRepo.ReadSingle(new Where("Id", eq, 1));

        Assert.Equal(1L, user!.Id);
        Assert.Equal("ada", user.Username);
        Assert.True(user.IsEnabled);
    }

    [Fact]
    public async Task ReadsUserByGuidUsernameAndCreated()
    {
        var ds = new Datasource("sqlite", new SqliteDriver(), new Dictionary<string, string?> { ["DATASOURCE_URL"] = "sqlite:///:memory:" });
        var connection = await Open(ds);
        await SchemaSpec.Apply(connection, "user-account");
        await Execute(connection, $"""
            INSERT INTO user (id, username, is_enabled, created) VALUES
                ('{Ada}', 'ada', 1, '2020-01-01T00:00:00.000Z'),
                ('{Grace}', 'grace', 1, '2024-06-01T00:00:00.000Z'),
                ('{Lin}', 'lin', 0, '2024-06-01T00:00:00.000Z');
            """);
        var repo = ds.CreateReadonlyRepository<UserAccount>(connection, new ReadonlyRepositoryMappingOptions<UserAccount>
        {
            EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["UserAccount"] = "user" }),
            PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
            {
                ["Key"] = "id",
                ["Login"] = "username",
                ["Active"] = "is_enabled",
                ["Opened"] = "created",
            }),
        });

        var opened = new DateTime(2024, 1, 1, 0, 0, 0, DateTimeKind.Utc);
        var byId = await repo.ReadSingle(new Where("Key", eq, Ada));
        var byLogin = new List<UserAccount>();
        await foreach (var row in repo.Read(new Where("Login", eq, "ada").And("Active", eq, true)))
            byLogin.Add(row);
        var later = new List<UserAccount>();
        await foreach (var row in repo.Read(new Where("Active", eq, true).And("Opened", gt, opened)))
            later.Add(row);

        Assert.Equal("ada", byId!.Login);
        Assert.Equal(Ada, byId.Key);
        Assert.True(byId.Active);
        Assert.Equal(2020, byId.Opened.Year);
        Assert.Equal("ada", Assert.Single(byLogin).Login);
        Assert.Equal("grace", Assert.Single(later).Login);
    }

    static async Task<IConnection> Open(IDatasource ds)
    {
        var connection = await ds.CreateConnection();
        await connection.Open();
        return connection;
    }

    static async Task Execute(IConnection connection, string sql)
    {
        var command = connection.CreateCommand();
        foreach (var statement in sql.Split(';', StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries))
            await command.Execute(new Native(statement));
    }
}

public class UsersViewCrudTests
{
    public UsersViewCrudTests()
    {
        DatasourceFactory.Reset();
        Environment.SetEnvironmentVariable("PROVIDER", "sqlite");
        Environment.SetEnvironmentVariable("DATASOURCE_URL", "sqlite:///:memory:");
        Environment.SetEnvironmentVariable("INSTALLED_PROVIDERS", "sqlite");
    }

    record UserType
    {
        public long Id { get; set; }
        public string Name { get; set; } = "";
    }

    record UserRole
    {
        public long UserId { get; set; }
        public string Name { get; set; } = "";
    }

    record User
    {
        public long Key { get; set; }
        public string Login { get; set; } = "";
        public bool Active { get; set; }
        public DateTime Opened { get; set; }
        public UserType UserType { get; set; } = new();
        public long Version { get; set; }
        public List<UserRole> Roles { get; set; } = [];
    }

    [Fact]
    public async Task UpdatesUsersViewWhenTheVersionMatches()
    {
        var ds = Datasource.Default;
        var connection = await Open(ds);
        await SchemaSpec.Apply(connection, "user-view-version");
        var casing = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
        var usersRepo = new CRUDRepository<User>(connection, new CRUDRepositoryOptions<User>
        {
            Ids = ["Key"],
            EntityMapper = casing,
            PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
            {
                ["Key"] = "id",
                ["Login"] = "username",
                ["Active"] = "is_enabled",
                ["Opened"] = "created",
                ["UserType"] = "user_type_id",
                ["Version"] = "version",
            }),
        });
        var userTypesRepo = new CRUDRepository<UserType>(connection, new CRUDRepositoryOptions<UserType>
        {
            Ids = ["Id"],
            EntityMapper = casing,
            PropertyMappings = casing,
        });
        var userRolesRepo = new CRUDRepository<UserRole>(connection, new CRUDRepositoryOptions<UserRole>
        {
            Ids = [],
            EntityMapper = casing,
            PropertyMappings = casing,
        });
        usersRepo.AddEnriched(new("UserType", "UserType", "Id"), userTypesRepo);
        usersRepo.AddEnriched(new("Roles", "Key", "UserId", Type: EnrichmentPropertyType.List), userRolesRepo);
        var admin = await userTypesRepo.Create(new UserType { Name = "admin" });
        var user = new User
        {
            Login = "ada",
            Active = true,
            Opened = new DateTime(2020, 1, 1, 0, 0, 0, DateTimeKind.Utc),
            UserType = admin,
            Version = 1,
            Roles = [new UserRole { Name = "admin" }, new UserRole { Name = "editor" }],
        };
        var created = await usersRepo.Create(user);
        var grace = await usersRepo.Create(new User
        {
            Login = "grace",
            Active = true,
            Opened = new DateTime(2024, 6, 1, 0, 0, 0, DateTimeKind.Utc),
            UserType = admin,
            Version = 1,
            Roles = [new UserRole { Name = "viewer" }],
        });
        var loaded = await usersRepo.ReadSingle(new Where("Key", eq, user.Key));
        var version = loaded!.Version;
        var member = await userTypesRepo.Create(new UserType { Name = "member" });
        loaded.Login = "ada-v2";
        loaded.UserType = member;
        loaded.Roles.Add(new UserRole { Name = "owner" });
        loaded.Version = version + 1;
        var updated = await usersRepo.Update(loaded, new Where("Key", eq, loaded.Key).And("Version", eq, version));
        var current = await usersRepo.ReadSingle(new Where("Key", eq, loaded.Key));
        var stale = await usersRepo.Update(loaded, new Where("Key", eq, loaded.Key).And("Version", eq, version));
        var removed = await usersRepo.Delete(loaded);
        var gone = await usersRepo.ReadSingleById(loaded.Key);
        Assert.Equal(2L, current!.Version);
        Assert.Equal("ada-v2", current.Login);
        Assert.True(current.Active);
        Assert.Equal(member.Id, current.UserType.Id);
        Assert.Equal("member", current.UserType.Name);
        Assert.Equal(["admin", "editor", "owner"], current.Roles.Select(role => role.Name).OrderBy(name => name));
        var roles = new List<UserRole>();
        await foreach (var role in userRolesRepo.Read(new Where("UserId", eq, loaded.Key)))
            roles.Add(role);

        Assert.Equal(1L, admin.Id);
        Assert.Equal(1L, created.Key);
        Assert.Equal(1L, user.Key);
        Assert.Equal(2L, grace.Key);
        Assert.Equal(2L, member.Id);
        Assert.Equal(1, updated.RowsAffected);
        Assert.Equal(0, stale.RowsAffected);
        Assert.Equal(1, removed.RowsAffected);
        Assert.Null(gone);
        Assert.Empty(roles);
    }

    static async Task<IConnection> Open(IDatasource ds)
    {
        var connection = await ds.CreateConnection();
        await connection.Open();
        return connection;
    }

    static async Task Execute(IConnection connection, string sql)
    {
        var command = connection.CreateCommand();
        foreach (var statement in sql.Split(';', StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries))
            await command.Execute(new Native(statement));
    }
}

public class UserViewEnrichmentTests
{
    static readonly Guid Ada = Guid.Parse("11111111-1111-1111-1111-111111111111");
    static readonly Guid Grace = Guid.Parse("22222222-2222-2222-2222-222222222222");
    static readonly Guid Lin = Guid.Parse("33333333-3333-3333-3333-333333333333");

    sealed class UserType
    {
        public int Id { get; set; }
        public string Name { get; set; } = "";
    }

    sealed class UsersView
    {
        public Guid Key { get; set; }
        public string Login { get; set; } = "";
        public bool Active { get; set; }
        public DateTime Opened { get; set; }
        public string UserType { get; set; } = "";
    }

    [Fact]
    public async Task FillsUserTypeOnUsersView()
    {
        var ds = new Datasource("sqlite", new SqliteDriver(), new Dictionary<string, string?> { ["DATASOURCE_URL"] = "sqlite:///:memory:" });
        var connection = await Open(ds);
        await SchemaSpec.Apply(connection, "user-type-lookup");
        await Execute(connection, $"""
            INSERT INTO user_type (id, name) VALUES
                (1, 'admin'),
                (2, 'user'),
                (3, 'service');
            INSERT INTO user (id, username, is_enabled, created, user_type_id) VALUES
                ('{Ada}', 'ada', 1, '2020-01-01T00:00:00.000Z', 1),
                ('{Grace}', 'grace', 1, '2024-06-01T00:00:00.000Z', 2),
                ('{Lin}', 'lin', 0, '2024-06-01T00:00:00.000Z', 3);
            """);
        var casing = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
        var usersRepo = (ReadonlyRepository<UsersView>)ds.CreateReadonlyRepository<UsersView>(connection, new ReadonlyRepositoryMappingOptions<UsersView>
        {
            EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["UsersView"] = "user" }),
            PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
            {
                ["Key"] = "id",
                ["Login"] = "username",
                ["Active"] = "is_enabled",
                ["Opened"] = "created",
                ["UserType"] = "user_type_id",
            }),
        });
        var typesRepo = ds.CreateReadonlyRepository<UserType>(connection, new ReadonlyRepositoryMappingOptions<UserType>
        {
            EntityMapper = casing,
            PropertyMappings = casing,
        });
        usersRepo.AddEnriched(new("UserType", "UserType", "Id", "Name"), typesRepo);
        var usersView = await usersRepo.ReadSingle(new Where("Key", eq, Ada));

        Assert.Equal("admin", usersView!.UserType);
        Assert.Equal("ada", usersView.Login);
        Assert.Equal(Ada, usersView.Key);
        Assert.True(usersView.Active);
        Assert.Equal(2020, usersView.Opened.Year);
        Assert.Null(typeof(UsersView).GetProperty("UserTypeId"));
    }

    static async Task<IConnection> Open(IDatasource ds)
    {
        var connection = await ds.CreateConnection();
        await connection.Open();
        return connection;
    }

    static async Task Execute(IConnection connection, string sql)
    {
        var command = connection.CreateCommand();
        foreach (var statement in sql.Split(';', StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries))
            await command.Execute(new Native(statement));
    }
}

public class UsersViewRoleTests
{
    static readonly Guid Ada = Guid.Parse("11111111-1111-1111-1111-111111111111");
    static readonly Guid Grace = Guid.Parse("22222222-2222-2222-2222-222222222222");
    static readonly Guid Lin = Guid.Parse("33333333-3333-3333-3333-333333333333");

    sealed class UserType
    {
        public int Id { get; set; }
        public string Name { get; set; } = "";
    }

    sealed class UserRole
    {
        public Guid UserId { get; set; }
        public string Name { get; set; } = "";
    }

    sealed class UsersView
    {
        public Guid Key { get; set; }
        public string Login { get; set; } = "";
        public bool Active { get; set; }
        public DateTime Opened { get; set; }
        public string UserType { get; set; } = "";
        public List<UserRole> Roles { get; set; } = [];
    }

    [Fact]
    public async Task FillsRolesWithHashset()
    {
        var ds = new Datasource("sqlite", new SqliteDriver(), new Dictionary<string, string?> { ["DATASOURCE_URL"] = "sqlite:///:memory:" });
        var connection = await Open(ds);
        await SchemaSpec.Apply(connection, "user-roles-read");
        await Execute(connection, Seed());
        var casing = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
        var usersRepo = new ReadonlyRepository<UsersView>(connection, new ReadonlyRepositoryMappingOptions<UsersView>
        {
            EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["UsersView"] = "user" }),
            PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
            {
                ["Key"] = "id",
                ["Login"] = "username",
                ["Active"] = "is_enabled",
                ["Opened"] = "created",
                ["UserType"] = "user_type_id",
            }),
        });
        var strategy = new HashsetOptimizedReadonlyEnrichmentStrategy<UsersView>();
        var typesRepo = ds.CreateReadonlyRepository<UserType>(connection, new ReadonlyRepositoryMappingOptions<UserType>
        {
            EntityMapper = casing,
            PropertyMappings = casing,
        });
        var rolesRepo = ds.CreateReadonlyRepository<UserRole>(connection, new ReadonlyRepositoryMappingOptions<UserRole>
        {
            EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["UserRole"] = "user_roles" }),
            PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
            {
                ["UserId"] = "user_id",
                ["Name"] = "name",
            }),
        });
        usersRepo.AddEnriched(new("UserType", "UserType", "Id", "Name"), typesRepo, strategy);
        usersRepo.AddEnriched(new("Roles", "Key", "UserId", Type: EnrichmentPropertyType.List), rolesRepo, strategy);
        var usersView = await usersRepo.ReadSingle(new Where("Key", eq, Ada));

        Assert.Equal("admin", usersView!.UserType);
        Assert.Equal(["admin", "editor"], usersView.Roles.Select(role => role.Name).OrderBy(name => name));
        Assert.Equal(usersView.Roles.Count, usersView.Roles.Count(role => role.UserId == Ada));
    }

    [Fact]
    public async Task FillsRolesFromAJoin()
    {
        var ds = new Datasource("sqlite", new SqliteDriver(), new Dictionary<string, string?> { ["DATASOURCE_URL"] = "sqlite:///:memory:" });
        var connection = await Open(ds);
        await SchemaSpec.Apply(connection, "user-roles-read");
        await Execute(connection, Seed());
        var usersRepo = new ReadonlyRepository<UsersView>(connection, new ReadonlyRepositoryMappingOptions<UsersView>
        {
            Entity = "User",
            EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string>
            {
                ["User"] = "user",
                ["UserType"] = "user_type",
                ["UserRole"] = "user_roles",
                ["Role"] = "roles",
            }),
            PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
            {
                ["Key"] = "id",
                ["Login"] = "username",
                ["Active"] = "is_enabled",
                ["Opened"] = "created",
                ["UserType"] = "user_type.name",
                ["UserTypeId"] = "user_type_id",
                ["UserId"] = "user_id",
                ["RoleId"] = "role_id",
                ["Name"] = "roles.name",
                ["Id"] = "id",
            }),
        });
        var joined = new SelectStatement(new()
        {
            Table = "User",
            Fields =
            [
                "User.Key",
                "User.Login",
                "User.Active",
                "User.Opened",
                "User.UserType",
                "UserRole.UserId",
                "UserRole.RoleId",
                "Role.Name",
            ],
            Joins =
            [
                new Join(JoinKind.Left, "UserType", "UserType.Id", "User.UserTypeId"),
                new Join(JoinKind.Left, "UserRole", "UserRole.UserId", "User.Key"),
                new Join(JoinKind.Left, "Role", "Role.Id", "UserRole.RoleId"),
            ],
        });
        usersRepo.AddEnriched(
            new("Roles", "Key", "UserId", Type: EnrichmentPropertyType.List),
            usersRepo,
            new NestedToObjectEnrichmentStrategy<UsersView>(new TypeMap
            {
                Key = "Key",
                Fields = new Dictionary<string, TypeMapField>
                {
                    ["Key"] = "Key",
                    ["Login"] = "Login",
                    ["Active"] = "Active",
                    ["Opened"] = "Opened",
                    ["UserType"] = "UserType",
                    ["Roles"] = new TypeMapGroup
                    {
                        Key = "RoleId",
                        Fields = new Dictionary<string, TypeMapField>
                        {
                            ["RoleId"] = "RoleId",
                            ["UserId"] = "UserId",
                            ["Name"] = "Name",
                        }
                    }
                }
            }));
        var items = new List<UsersView>();
        await foreach (var item in usersRepo.Read(joined))
            items.Add(item);
        var ada = Assert.Single(items, item => item.Login == "ada");
        var grace = Assert.Single(items, item => item.Login == "grace");
        var lin = Assert.Single(items, item => item.Login == "lin");

        Assert.Equal(3, items.Count);
        Assert.Equal(Ada, ada.Key);
        Assert.Equal("ada", ada.Login);
        Assert.True(ada.Active);
        Assert.Equal(new DateTime(2020, 1, 1, 0, 0, 0, DateTimeKind.Utc), ada.Opened);
        Assert.Equal("admin", ada.UserType);
        Assert.Equal(["admin", "editor"], ada.Roles.Select(role => role.Name).OrderBy(name => name));
        Assert.Equal(ada.Roles.Count, ada.Roles.Count(role => role.UserId == Ada));
        Assert.Equal(Grace, grace.Key);
        Assert.Equal("grace", grace.Login);
        Assert.True(grace.Active);
        Assert.Equal(new DateTime(2024, 6, 1, 0, 0, 0, DateTimeKind.Utc), grace.Opened);
        Assert.Equal("user", grace.UserType);
        Assert.Equal(["viewer"], grace.Roles.Select(role => role.Name));
        Assert.Equal(grace.Roles.Count, grace.Roles.Count(role => role.UserId == Grace));
        Assert.Equal(Lin, lin.Key);
        Assert.Equal("lin", lin.Login);
        Assert.False(lin.Active);
        Assert.Equal(new DateTime(2024, 6, 1, 0, 0, 0, DateTimeKind.Utc), lin.Opened);
        Assert.Equal("service", lin.UserType);
        Assert.Empty(lin.Roles);
    }

    static string Seed() => $"""
        INSERT INTO user_type (id, name) VALUES
            (1, 'admin'),
            (2, 'user'),
            (3, 'service');
        INSERT INTO user (id, username, is_enabled, created, user_type_id) VALUES
            ('{Ada}', 'ada', 1, '2020-01-01T00:00:00.000Z', 1),
            ('{Grace}', 'grace', 1, '2024-06-01T00:00:00.000Z', 2),
            ('{Lin}', 'lin', 0, '2024-06-01T00:00:00.000Z', 3);
        INSERT INTO roles (id, name) VALUES
            (1, 'admin'),
            (2, 'editor'),
            (3, 'viewer');
        INSERT INTO user_roles (user_id, role_id) VALUES
            ('{Ada}', 1),
            ('{Ada}', 2),
            ('{Grace}', 3);
        """;

    static async Task<IConnection> Open(IDatasource ds)
    {
        var connection = await ds.CreateConnection();
        await connection.Open();
        return connection;
    }

    static async Task Execute(IConnection connection, string sql)
    {
        var command = connection.CreateCommand();
        foreach (var statement in sql.Split(';', StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries))
            await command.Execute(new Native(statement));
    }
}

public class UserCrudTests
{
    public UserCrudTests()
    {
        DatasourceFactory.Reset();
        Environment.SetEnvironmentVariable("PROVIDER", "sqlite");
        Environment.SetEnvironmentVariable("DATASOURCE_URL", "sqlite:///:memory:");
        Environment.SetEnvironmentVariable("INSTALLED_PROVIDERS", "sqlite");
    }

    sealed class User
    {
        public long Id { get; set; }
        public string Username { get; set; } = "";
        public bool IsEnabled { get; set; }
    }

    [Fact]
    public async Task WritesUserFromTheType()
    {
        var ds = Datasource.Default;
        var connection = await Open(ds);
        await SchemaSpec.Apply(connection, "user-pascal");
        var casing = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
        var usersRepo = ds.CreateRepository<User>(connection, new CRUDRepositoryOptions<User>
        {
            PropertyMappings = casing,
            Ids = ["Id"],
        });
        var created = await usersRepo.Create(new User { Username = "ada", IsEnabled = true });
        var byId = await usersRepo.ReadSingleById(created.Id);
        var matches = new List<User>();
        await foreach (var row in usersRepo.Read(new Where("Username", eq, "ada").And("IsEnabled", eq, true)))
            matches.Add(row);
        created.Username = "ada-v2";
        var updated = await usersRepo.Update(created);
        var renamed = await usersRepo.ReadSingleById(created.Id);
        var removed = await usersRepo.Delete(created);
        var gone = await usersRepo.ReadSingleById(created.Id);

        Assert.True(created.Id > 0);
        Assert.Equal(1, updated.RowsAffected);
        Assert.Equal(1, removed.RowsAffected);
        Assert.Equal(created.Id, byId!.Id);
        Assert.Equal("ada", byId.Username);
        Assert.True(byId.IsEnabled);
        Assert.Equal("ada", Assert.Single(matches).Username);
        Assert.Equal("ada-v2", renamed!.Username);
        Assert.Null(gone);
    }

    static async Task<IConnection> Open(IDatasource ds)
    {
        var connection = await ds.CreateConnection();
        await connection.Open();
        return connection;
    }

    static async Task Execute(IConnection connection, string sql)
    {
        var command = connection.CreateCommand();
        foreach (var statement in sql.Split(';', StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries))
            await command.Execute(new Native(statement));
    }
}

public class MultiTenantAccountTests
{
    sealed class Account
    {
        public int TenantId { get; set; }
        public int Id { get; set; }
        public string Name { get; set; } = "";
    }

    [Fact]
    public async Task ReadsUpdatesAndDeletesByTenantAndId()
    {
        var ds = new Datasource("sqlite", new SqliteDriver(), new Dictionary<string, string?> { ["DATASOURCE_URL"] = "sqlite:///:memory:" });
        var connection = await Open(ds);
        await SchemaSpec.Apply(connection, "multi-tenant");
        await Execute(connection, """
            INSERT INTO account (tenant_id, id, name) VALUES
                (1, 1, 'ada'),
                (2, 1, 'grace');
            """);
        var casing = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
        var accountsRepo = new CRUDRepository<Account>(connection, new CRUDRepositoryOptions<Account>
        {
            EntityMapper = casing,
            PropertyMappings = casing,
            Ids = ["TenantId", "Id"],
        });
        var ada = await accountsRepo.ReadSingleById(new Row { ["TenantId"] = 1, ["Id"] = 1 });
        ada!.Name = "ada-v2";
        var updated = await accountsRepo.Update(ada);
        var renamed = await accountsRepo.ReadSingleById(new Row { ["TenantId"] = 1, ["Id"] = 1 });
        var removed = await accountsRepo.Delete(ada);
        var gone = await accountsRepo.ReadSingleById(new Row { ["TenantId"] = 1, ["Id"] = 1 });
        var grace = await accountsRepo.ReadSingleById(new Row { ["TenantId"] = 2, ["Id"] = 1 });

        Assert.Equal(1, updated.RowsAffected);
        Assert.Equal(1, removed.RowsAffected);
        Assert.Equal("ada-v2", renamed!.Name);
        Assert.Equal(1, renamed.TenantId);
        Assert.Null(gone);
        Assert.Equal("grace", grace!.Name);
        Assert.Equal(2, grace.TenantId);
    }

    static async Task<IConnection> Open(IDatasource ds)
    {
        var connection = await ds.CreateConnection();
        await connection.Open();
        return connection;
    }

    static async Task Execute(IConnection connection, string sql)
    {
        var command = connection.CreateCommand();
        foreach (var statement in sql.Split(';', StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries))
            await command.Execute(new Native(statement));
    }
}

