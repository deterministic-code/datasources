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

        var byId = await repo.ReadSingle(new WhereClause("id", eq, 1));
        var disabled = new List<User>();
        await foreach (var row in repo.Read(new WhereClause("username", eq, "grace").And("is_enabled", eq, 0)))
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
        var byId = await repo.ReadSingle(new WhereClause("Key", eq, Ada));
        var byLogin = new List<UserAccount>();
        await foreach (var row in repo.Read(new WhereClause("Login", eq, "ada").And("Active", eq, true)))
            byLogin.Add(row);
        var later = new List<UserAccount>();
        await foreach (var row in repo.Read(new WhereClause("Active", eq, true).And("Opened", gt, opened)))
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

    sealed class UserAccount
    {
        public Guid Key { get; set; }
        public string Login { get; set; } = "";
        public bool Active { get; set; }
        public DateTime Opened { get; set; }
        public int UserTypeId { get; set; }
        public string UserType { get; set; } = "";
    }

    sealed class UserType
    {
        public int Id { get; set; }
        public string Name { get; set; } = "";
    }

    sealed class UserView
    {
        public Guid Key { get; set; }
        public string Login { get; set; } = "";
        public bool Active { get; set; }
        public DateTime Opened { get; set; }
        public string UserType { get; set; } = "";
    }

    [Fact]
    public async Task FillsUserTypeOnUserView()
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
        var users = ds.CreateReadonlyRepository<UserAccount>(connection, new ReadonlyRepositoryOptions<UserAccount>
        {
            Entity = "user",
            ReadColumns = ["Key", "Login", "Active", "Opened", "UserTypeId"],
            ColumnMappings = new DictionaryMapper<string>(new Dictionary<string, string>
            {
                ["Key"] = "id",
                ["Login"] = "username",
                ["Active"] = "is_enabled",
                ["Opened"] = "created",
                ["UserTypeId"] = "user_type_id",
            }),
        });
        var casing = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
        var types = ds.CreateReadonlyRepository<UserType>(connection, new ReadonlyRepositoryOptions<UserType>
        {
            Entity = "UserType",
            EntityMapper = casing,
            ColumnMappings = casing,
        });
        var enriched = new EnrichedReadonlyRepository<UserAccount>(users);
        enriched.AddEnriched("UserType", "UserTypeId", types, "Id", "Name");
        var account = await enriched.ReadSingle(new WhereClause("Key", eq, Ada));
        var usersView = new UserView
        {
            Key = account!.Key,
            Login = account.Login,
            Active = account.Active,
            Opened = account.Opened,
            UserType = account.UserType,
        };

        Assert.Equal(1, account.UserTypeId);
        Assert.Equal("admin", usersView.UserType);
        Assert.Equal("ada", usersView.Login);
        Assert.Equal(Ada, usersView.Key);
        Assert.True(usersView.Active);
        Assert.Equal(2020, usersView.Opened.Year);
        Assert.Null(typeof(UserView).GetProperty("UserTypeId"));
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
