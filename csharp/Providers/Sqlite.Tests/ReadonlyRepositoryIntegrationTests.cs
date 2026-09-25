using Datasources;
using Datasources.Sqlite;
using static Datasources.Cmp;

namespace Datasources.Sqlite.Tests;

public class ReadonlyRepositoryIntegrationTests
{
    static readonly Guid Ada = Guid.Parse("11111111-1111-1111-1111-111111111111");
    static readonly Guid Grace = Guid.Parse("22222222-2222-2222-2222-222222222222");
    static readonly Guid Lin = Guid.Parse("33333333-3333-3333-3333-333333333333");

    sealed class User
    {
        public int id { get; set; }
        public string username { get; set; } = "";
        public bool is_enabled { get; set; }
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
        var ds = new Datasource("sqlite", new SqliteDriver(), new Dictionary<string, string?> { ["filename"] = ":memory:" });
        var connection = await Open(ds, """
            CREATE TABLE users (
                id INTEGER PRIMARY KEY,
                username TEXT NOT NULL,
                is_enabled INTEGER NOT NULL
            );
            INSERT INTO users (id, username, is_enabled) VALUES
                (1, 'ada', 1),
                (2, 'grace', 0);
            """);
        var repo = ds.CreateReadonlyRepository<User>(connection, new ReadonlyRepositoryOptions<User>
        {
            EntityMapper = new PluralizerMapper(),
        });

        var byId = await repo.ReadSingle(new Where("id", eq, 1));
        var disabled = new List<User>();
        await foreach (var row in repo.Read(new Where("username", eq, "grace").And("is_enabled", eq, 0)))
            disabled.Add(row);

        Assert.Equal("ada", byId!.username);
        Assert.True(byId.is_enabled);
        var grace = Assert.Single(disabled);
        Assert.Equal(2, grace.id);
        Assert.False(grace.is_enabled);
    }

    [Fact]
    public async Task ReadsUserByGuidUsernameAndCreated()
    {
        var ds = new Datasource("sqlite", new SqliteDriver(), new Dictionary<string, string?> { ["filename"] = ":memory:" });
        var connection = await Open(ds, $"""
            CREATE TABLE user (
                id TEXT PRIMARY KEY,
                username TEXT NOT NULL,
                is_enabled INTEGER NOT NULL,
                created TEXT NOT NULL
            );
            INSERT INTO user (id, username, is_enabled, created) VALUES
                ('{Ada}', 'ada', 1, '2020-01-01T00:00:00.000Z'),
                ('{Grace}', 'grace', 1, '2024-06-01T00:00:00.000Z'),
                ('{Lin}', 'lin', 0, '2024-06-01T00:00:00.000Z');
            """);
        var repo = ds.CreateReadonlyRepository<UserAccount>(connection, new ReadonlyRepositoryOptions<UserAccount>
        {
            Entity = "user",
            ColumnMappings = new DictionaryMapper<string>(new Dictionary<string, string>
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
        var ds = new Datasource("sqlite", new SqliteDriver(), new Dictionary<string, string?> { ["filename"] = ":memory:" });
        var connection = await Open(ds, $"""
            CREATE TABLE user_type (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL
            );
            INSERT INTO user_type (id, name) VALUES
                (1, 'admin'),
                (2, 'user'),
                (3, 'service');
            CREATE TABLE user (
                id TEXT PRIMARY KEY,
                username TEXT NOT NULL,
                is_enabled INTEGER NOT NULL,
                created TEXT NOT NULL,
                user_type_id INTEGER NOT NULL
            );
            INSERT INTO user (id, username, is_enabled, created, user_type_id) VALUES
                ('{Ada}', 'ada', 1, '2020-01-01T00:00:00.000Z', 1),
                ('{Grace}', 'grace', 1, '2024-06-01T00:00:00.000Z', 2),
                ('{Lin}', 'lin', 0, '2024-06-01T00:00:00.000Z', 3);
            """);
        var casing = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
        var usersRepo = ds.CreateReadonlyEnrichedRepository<UsersView>(connection, new ReadonlyRepositoryOptions<UsersView>
        {
            EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["UsersView"] = "user" }),
            ColumnMappings = new DictionaryMapper<string>(new Dictionary<string, string>
            {
                ["Key"] = "id",
                ["Login"] = "username",
                ["Active"] = "is_enabled",
                ["Opened"] = "created",
                ["UserType"] = "user_type_id",
            }),
        });
        var typesRepo = ds.CreateReadonlyRepository<UserType>(connection, new ReadonlyRepositoryOptions<UserType>
        {
            EntityMapper = casing,
            ColumnMappings = casing,
        });
        usersRepo.AddEnriched("UserType", "UserType", typesRepo, "Id", "Name");
        var usersView = await usersRepo.ReadSingle(new Where("Key", eq, Ada));

        Assert.Equal("admin", usersView!.UserType);
        Assert.Equal("ada", usersView.Login);
        Assert.Equal(Ada, usersView.Key);
        Assert.True(usersView.Active);
        Assert.Equal(2020, usersView.Opened.Year);
        Assert.Null(typeof(UsersView).GetProperty("UserTypeId"));
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
        var ds = new Datasource("sqlite", new SqliteDriver(), new Dictionary<string, string?> { ["filename"] = ":memory:" });
        var connection = await Open(ds, Seed());
        var casing = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
        var users = ds.CreateReadonlyRepository<UsersView>(connection, new ReadonlyRepositoryOptions<UsersView>
        {
            EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["UsersView"] = "user" }),
            ColumnMappings = new DictionaryMapper<string>(new Dictionary<string, string>
            {
                ["Key"] = "id",
                ["Login"] = "username",
                ["Active"] = "is_enabled",
                ["Opened"] = "created",
                ["UserType"] = "user_type_id",
            }),
        });
        var usersRepo = new EnrichedReadonlyRepository<UsersView>(
            users,
            new HashsetOptimizedReadonlyEnrichmentStrategy<UsersView>());
        var typesRepo = ds.CreateReadonlyRepository<UserType>(connection, new ReadonlyRepositoryOptions<UserType>
        {
            EntityMapper = casing,
            ColumnMappings = casing,
        });
        var rolesRepo = ds.CreateReadonlyRepository<UserRole>(connection, new ReadonlyRepositoryOptions<UserRole>
        {
            Entity = "user_roles",
            ColumnMappings = new DictionaryMapper<string>(new Dictionary<string, string>
            {
                ["UserId"] = "user_id",
                ["Name"] = "name",
            }),
            Query = new TransformMapper<ISelectStatement, ISelectStatement>(
                query => new SelectStatement(new()
                {
                    Table = "user_roles",
                    Fields = ["user_roles.user_id", "roles.name"],
                    Where = query.Where,
                    Joins = [new Join(JoinKind.Inner, "roles", "roles.id", "user_roles.role_id")],
                }),
                query => query),
        });
        usersRepo.AddEnriched("UserType", "UserType", typesRepo, "Id", "Name");
        usersRepo.AddEnriched("Roles", "Key", rolesRepo, "UserId", many: true);
        var usersView = await usersRepo.ReadSingle(new Where("Key", eq, Ada));

        Assert.Equal("admin", usersView!.UserType);
        Assert.Equal(["admin", "editor"], usersView.Roles.Select(role => role.Name).OrderBy(name => name));
        Assert.Equal(usersView.Roles.Count, usersView.Roles.Count(role => role.UserId == Ada));
    }

    [Fact]
    public async Task FillsRolesFromAJoin()
    {
        var ds = new Datasource("sqlite", new SqliteDriver(), new Dictionary<string, string?> { ["filename"] = ":memory:" });
        var connection = await Open(ds, Seed());
        var source = ds.CreateReadonlyRepository<Row>(connection, new ReadonlyRepositoryOptions<Row>
        {
            ColumnMappings = new DictionaryMapper<string>(new Dictionary<string, string>
            {
                ["Key"] = "id",
                ["Login"] = "username",
                ["Active"] = "is_enabled",
                ["Opened"] = "created",
                ["UserType"] = "user_type.name",
                ["UserId"] = "user_id",
                ["RoleId"] = "role_id",
                ["Name"] = "roles.name",
            }),
        });
        var usersRepo = new EnrichedReadonlyRepository<UsersView>(
            ds.CreateReadonlyRepository<UsersView>(connection),
            new FlatToNestedReadonlyEnrichmentStrategy<UsersView>(
                source,
                new FlatToNestedMapper(new TypeMap
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
                }),
                new MappedValueMapper<UsersView>(),
                new SelectStatement(new()
                {
                    Table = "user",
                    Fields =
                    [
                        "user.id",
                        "user.username",
                        "user.is_enabled",
                        "user.created",
                        "user_type.name",
                        "user_roles.user_id",
                        "user_roles.role_id",
                        "roles.name",
                    ],
                    Joins =
                    [
                        new Join(JoinKind.Left, "user_type", "user_type.id", "user.user_type_id"),
                        new Join(JoinKind.Left, "user_roles", "user_roles.user_id", "user.id"),
                        new Join(JoinKind.Left, "roles", "roles.id", "user_roles.role_id"),
                    ],
                })));
        var items = new List<UsersView>();
        await foreach (var item in usersRepo.Read())
            items.Add(item);
        var ada = Assert.Single(items, item => item.Login == "ada");
        var grace = Assert.Single(items, item => item.Login == "grace");
        var lin = Assert.Single(items, item => item.Login == "lin");

        Assert.Equal("admin", ada.UserType);
        Assert.Equal(["admin", "editor"], ada.Roles.Select(role => role.Name).OrderBy(name => name));
        Assert.Equal(ada.Roles.Count, ada.Roles.Count(role => role.UserId == Ada));
        Assert.Equal(["viewer"], grace.Roles.Select(role => role.Name));
        Assert.Empty(lin.Roles);
    }

    static string Seed() => $"""
        CREATE TABLE user_type (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
        );
        INSERT INTO user_type (id, name) VALUES
            (1, 'admin'),
            (2, 'user'),
            (3, 'service');
        CREATE TABLE user (
            id TEXT PRIMARY KEY,
            username TEXT NOT NULL,
            is_enabled INTEGER NOT NULL,
            created TEXT NOT NULL,
            user_type_id INTEGER NOT NULL
        );
        INSERT INTO user (id, username, is_enabled, created, user_type_id) VALUES
            ('{Ada}', 'ada', 1, '2020-01-01T00:00:00.000Z', 1),
            ('{Grace}', 'grace', 1, '2024-06-01T00:00:00.000Z', 2),
            ('{Lin}', 'lin', 0, '2024-06-01T00:00:00.000Z', 3);
        CREATE TABLE roles (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
        );
        INSERT INTO roles (id, name) VALUES
            (1, 'admin'),
            (2, 'editor'),
            (3, 'viewer');
        CREATE TABLE user_roles (
            user_id TEXT NOT NULL,
            role_id INTEGER NOT NULL
        );
        INSERT INTO user_roles (user_id, role_id) VALUES
            ('{Ada}', 1),
            ('{Ada}', 2),
            ('{Grace}', 3);
        """;

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

public class UserCrudTests
{
    sealed class User
    {
        public int id { get; set; }
        public string username { get; set; } = "";
        public bool is_enabled { get; set; }
    }

    [Fact]
    public async Task WritesUserFromTheType()
    {
        var ds = new Datasource("sqlite", new SqliteDriver(), new Dictionary<string, string?> { ["filename"] = ":memory:" });
        var connection = await Open(ds, """
            CREATE TABLE User (
                id INTEGER PRIMARY KEY,
                is_enabled INTEGER NOT NULL,
                username TEXT NOT NULL
            );
            """);
        var usersRepo = new CRUDRepository<User>(connection);
        var created = new User { username = "ada", is_enabled = true };
        await usersRepo.Create(created);
        var byId = await usersRepo.ReadSingleById(created.id);
        var matches = new List<User>();
        await foreach (var row in usersRepo.Read(new Where("username", eq, "ada").And("is_enabled", eq, true)))
            matches.Add(row);
        created.username = "ada-v2";
        await usersRepo.Update(created);
        var renamed = await usersRepo.ReadSingleById(created.id);
        await usersRepo.Delete(created);
        var gone = await usersRepo.ReadSingleById(created.id);

        Assert.Equal(1, created.id);
        Assert.Equal(1, byId!.id);
        Assert.Equal("ada", byId.username);
        Assert.True(byId.is_enabled);
        Assert.Equal("ada", Assert.Single(matches).username);
        Assert.Equal("ada-v2", renamed!.username);
        Assert.Null(gone);
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

public class UsersViewCrudTests
{
    sealed class UserType
    {
        public int Id { get; set; }
        public string Name { get; set; } = "";
    }

    sealed class UserRole
    {
        public int UserId { get; set; }
        public string Name { get; set; } = "";
    }

    sealed class UsersView
    {
        public int Key { get; set; }
        public string Login { get; set; } = "";
        public bool Active { get; set; }
        public DateTime Opened { get; set; }
        public int UserTypeId { get; set; }
        public string UserType { get; set; } = "";
        public int Version { get; set; }
        public List<UserRole> Roles { get; set; } = [];
    }

    [Fact]
    public async Task UpdatesUsersViewWhenTheVersionMatches()
    {
        var ds = new Datasource("sqlite", new SqliteDriver(), new Dictionary<string, string?> { ["filename"] = ":memory:" });
        var connection = await Open(ds, """
            CREATE TABLE user_type (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL
            );
            CREATE TABLE user (
                id INTEGER PRIMARY KEY,
                username TEXT NOT NULL,
                is_enabled INTEGER NOT NULL,
                created TEXT NOT NULL,
                user_type_id INTEGER NOT NULL,
                version INTEGER NOT NULL
            );
            CREATE TABLE user_role (
                user_id INTEGER NOT NULL,
                name TEXT NOT NULL
            );
            """);
        var casing = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
        var usersRepo = new CRUDRepository<UsersView>(connection, new CRUDRepositoryOptions<UsersView>
        {
            Ids = ["Key"],
            ReadProperties = ["Key", "Login", "Active", "Opened", "UserTypeId", "Version"],
            EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["UsersView"] = "user" }),
            ColumnMappings = new DictionaryMapper<string>(new Dictionary<string, string>
            {
                ["Key"] = "id",
                ["Login"] = "username",
                ["Active"] = "is_enabled",
                ["Opened"] = "created",
                ["UserTypeId"] = "user_type_id",
                ["Version"] = "version",
            }),
        });
        var userTypesRepo = new CRUDRepository<UserType>(connection, new CRUDRepositoryOptions<UserType>
        {
            Ids = ["Id"],
            EntityMapper = casing,
            ColumnMappings = casing,
        });
        var userRolesRepo = new CRUDRepository<UserRole>(connection, new CRUDRepositoryOptions<UserRole>
        {
            Ids = [],
            EntityMapper = casing,
            ColumnMappings = casing,
        });
        usersRepo.AddEnriched("UserType", "UserTypeId", userTypesRepo, "Id", "Name");
        usersRepo.AddEnriched("Roles", "Key", userRolesRepo, "UserId", many: true);
        var admin = new UserType { Name = "admin" };
        await userTypesRepo.Create(admin);
        var user = new UsersView
        {
            Login = "ada",
            Active = true,
            Opened = new DateTime(2020, 1, 1, 0, 0, 0, DateTimeKind.Utc),
            UserTypeId = admin.Id,
            Version = 1,
            Roles = [new UserRole { Name = "admin" }, new UserRole { Name = "editor" }],
        };
        await usersRepo.Create(user);
        var loaded = await usersRepo.ReadSingle(new Where("Key", eq, user.Key));
        var version = loaded!.Version;
        loaded.Login = "ada-v2";
        loaded.Version = version + 1;
        var updated = await usersRepo.Update(loaded, new Where("Key", eq, loaded.Key).And("Version", eq, version));
        var current = await usersRepo.ReadSingle(new Where("Key", eq, loaded.Key));
        var stale = await usersRepo.Update(loaded, new Where("Key", eq, loaded.Key).And("Version", eq, version));
        await usersRepo.Delete(loaded);
        var gone = await usersRepo.ReadSingleById(loaded.Key);
        var roles = new List<UserRole>();
        await foreach (var role in userRolesRepo.Read(new Where("UserId", eq, loaded.Key)))
            roles.Add(role);

        Assert.Equal(1, admin.Id);
        Assert.Equal(1, user.Key);
        Assert.Equal("admin", loaded.UserType);
        Assert.Equal(["admin", "editor"], loaded.Roles.Select(role => role.Name).OrderBy(name => name));
        Assert.Equal(1, updated.RowsAffected);
        Assert.Equal(2, current!.Version);
        Assert.Equal("ada-v2", current.Login);
        Assert.Equal(0, stale.RowsAffected);
        Assert.Null(gone);
        Assert.Empty(roles);
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
