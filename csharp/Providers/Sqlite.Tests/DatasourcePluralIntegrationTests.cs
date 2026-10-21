using Datasources;
using Datasources.SqlSpecs;
using static Datasources.Cmp;

namespace Datasources.Sqlite.Tests;

[Collection("DatasourceFactory")]
public class DatasourcePluralIntegrationTests
{
    public DatasourcePluralIntegrationTests()
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
    public async Task CreatesUsingDynamicAfterRegister()
    {
        dynamic ds = Datasource.Default;
        var connection = await Open(ds);
        await SchemaSpec.Apply(connection, "user-pascal");
        Register(ds);
        User created = await ds.Users.Create(new User { Username = "ada", IsEnabled = true });
        User? loaded = await ds.Users.ReadSingle();
        Response updated = await ds.Users.Update(created);
        Response deleted = await ds.Users.Delete(created);
        Assert.True(created.Id > 0);
        Assert.Equal("ada", loaded!.Username);
        Assert.Equal(1, updated.RowsAffected);
        Assert.Equal(1, deleted.RowsAffected);
    }

    [Fact]
    public async Task WritesUserThroughPluralName()
    {
        var ds = (Datasource)Datasource.Default;
        var connection = await Open(ds);
        await SchemaSpec.Apply(connection, "user-pascal");
        Register(ds);
        dynamic dyn = ds;
        User created = await dyn.Users.Create(new User { Username = "ada", IsEnabled = true });
        User? byId = await dyn.Users.ReadSingleById(created.Id);
        ICRUDRepository<User> usersRepo = dyn.Users;
        var matches = new List<User>();
        await foreach (var row in usersRepo.Read(new Where("Username", eq, "ada").And("IsEnabled", eq, true)))
            matches.Add(row);
        created.Username = "ada-v2";
        Response updated = await dyn.Users.Update(created);
        User? renamed = await dyn.Users.ReadSingleById(created.Id);
        Response removed = await dyn.Users.Delete(created);
        User? gone = await dyn.Users.ReadSingleById(created.Id);

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

    [Fact]
    public async Task ReadsThroughAccessCreateReadonly()
    {
        var ds = (Datasource)Datasource.Default;
        var connection = await Open(ds);
        await SchemaSpec.Apply(connection, "user-pascal");
        Register(ds);
        dynamic dyn = ds;
        User created = await dyn.Users.Create(new User { Username = "ada", IsEnabled = true });
        var usersRepo = (IReadonlyRepository<User>)ds.Repositories.Access("Users", connection).CreateReadonly();
        var loaded = await usersRepo.ReadSingle(new Where("Id", eq, created.Id));

        Assert.Equal("ada", loaded!.Username);
        Assert.True(loaded.IsEnabled);
        Assert.Equal(created.Id, loaded.Id);
    }

    static void Register(IDatasource ds)
    {
        var options = new CRUDRepositoryOptions<User>
        {
            PropertyMappings = new ChangeCasingMapper(Casing.Pascal, Casing.Snake),
            Ids = ["Id"],
        };
        ds.Repositories.Register(
            "User",
            "Users",
            conn => ds.CreateRepository<User>(conn, options),
            conn => ds.CreateReadonlyRepository<User>(conn, options));
    }

    static async Task<IConnection> Open(IDatasource ds)
    {
        var connection = await ds.CreateConnection();
        await connection.Open();
        return connection;
    }
}
