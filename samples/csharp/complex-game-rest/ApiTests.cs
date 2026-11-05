using System.Text.Json;
using Backend.Types.Datasource;
using ComplexGameRest;
using Datasources;
using Datasources.Sqlite;

public sealed class ApiTests
{
    [Fact]
    public async Task MapsDatasourceErrors()
    {
        var opened = await Open();
        var result = await opened.Api.Handle("GET", "/users", null);
        Assert.Equal(400, result.Status);
        Assert.False(string.IsNullOrEmpty(result.Body));
    }

    [Fact]
    public async Task RejectsUnknownRoutesLookupsAndMissingRows()
    {
        var opened = await Migrated();
        var api = opened.Api;
        Assert.Equal(404, (await api.Handle("GET", "/", null)).Status);
        Assert.Equal(404, (await api.Handle("GET", "", null)).Status);
        Assert.Equal(404, (await api.Handle("GET", "/nope", null)).Status);
        Assert.Equal(400, (await api.Handle("POST", "/users/", "null")).Status);
        Assert.Equal(400, (await api.Handle("POST", "/users/", "{")).Status);
        Assert.Equal(400, (await api.Handle("PUT", "/users/?id=abc", "{}")).Status);
        Assert.Equal(404, (await api.Handle("PUT", "/users/?id=99", """{"username":"ada","password":"x"}""")).Status);
        Assert.Equal(404, (await api.Handle("DELETE", "/users/?id=99", null)).Status);
        Assert.Equal(400, (await api.Handle("DELETE", "/users/", null)).Status);
        Assert.Equal(400, (await api.Handle("GET", "/categories?categoryId=nope", null)).Status);
        Assert.Equal(404, (await api.Handle("PUT", "/games/?id=99", """{"name":"star"}""")).Status);
        Assert.Equal(404, (await api.Handle("DELETE", "/reviews/?id=99", null)).Status);
        Assert.Equal(404, (await api.Handle("PUT", "/game_states/?id=99", """{"state":"x"}""")).Status);
        Assert.Equal(404, (await api.Handle("PUT", "/contacts/?id=99", """{"firstName":"Ada"}""")).Status);
        Assert.Equal(404, (await api.Handle("PUT", "/categories/?id=99", """{"name":"root"}""")).Status);
        _ = Clock.Now();
        _ = Clock.Uuid();
    }

    [Fact]
    public async Task WritesAndReadsEachRoute()
    {
        var opened = await Migrated();
        var api = opened.Api;
        await new RolesRepository(opened.Connection).Create(new Role { Name = "player", Description = "plays" });
        await new GroupsRepository(opened.Connection).Create(new Group { Name = "guild", Description = "guild" });
        await new ContactSourcesRepository(opened.Connection).Create(new ContactSource { Name = "Manual" });
        var userId = Id(await api.Handle("POST", "/users/", """{"username":"ada","password":"x","uuid":"11111111-1111-1111-1111-111111111111","created":"2020-01-02T03:04:05Z","version":"AA==","settings":{"theme":"dark"},"roles":[{"name":"player"}],"groups":[{"name":"guild"}]}"""));
        Assert.Equal(200, (await api.Handle("GET", "/users", null)).Status);
        Assert.Equal(200, (await api.Handle("GET", $"/users/?id={userId}&username=ada", null)).Status);
        Assert.Equal(200, (await api.Handle("PUT", $"/users/?id={userId}", """{"username":"ada","password":"x"}""")).Status);
        Assert.Equal(204, (await api.Handle("DELETE", $"/users/?id={userId}", null)).Status);
        Assert.Equal(200, (await api.Handle("GET", "/categories", null)).Status);
        Assert.Equal(200, (await api.Handle("GET", "/categories?categoryId=", null)).Status);
        var categoryId = Id(await api.Handle("POST", "/categories/", """{"name":"arcade"}"""));
        Assert.Equal(200, (await api.Handle("GET", $"/categories?category_id={categoryId}", null)).Status);
        Assert.Equal(200, (await api.Handle("PUT", $"/categories/?id={categoryId}", """{"name":"arcade"}""")).Status);
        Assert.Equal(204, (await api.Handle("DELETE", $"/categories/?id={categoryId}", null)).Status);
        var gameId = Id(await api.Handle("POST", "/games/", """{"name":"star","settings":{"lives":"3"},"categories":[{"name":"arcade"}]}"""));
        Assert.Equal(200, (await api.Handle("GET", $"/games/?id={gameId}&name=star", null)).Status);
        Assert.Equal(200, (await api.Handle("PUT", $"/games/?id={gameId}", """{"name":"star"}""")).Status);
        Assert.Equal(204, (await api.Handle("DELETE", $"/games/?id={gameId}", null)).Status);
        userId = Id(await api.Handle("POST", "/users/", """{"username":"ada","password":"x"}"""));
        gameId = Id(await api.Handle("POST", "/games/", """{"name":"star"}"""));
        var reviewId = Id(await api.Handle("POST", "/reviews/", $"{{\"gameId\":{gameId},\"userId\":{userId},\"description\":\"fun\",\"rating\":\"4.5\"}}"));
        Assert.Equal(200, (await api.Handle("GET", $"/reviews/?gameId={gameId}&userId={userId}", null)).Status);
        Assert.Equal(200, (await api.Handle("PUT", $"/reviews/?id={reviewId}", $"{{\"gameId\":{gameId},\"userId\":{userId},\"description\":\"great\"}}")).Status);
        Assert.Equal(204, (await api.Handle("DELETE", $"/reviews/?id={reviewId}", null)).Status);
        var stateId = Id(await api.Handle("POST", "/game_states/", $"{{\"gameId\":{gameId},\"userId\":{userId},\"state\":\"intro\"}}"));
        Assert.Equal(200, (await api.Handle("GET", $"/game_states/?id={stateId}", null)).Status);
        Assert.Equal(200, (await api.Handle("PUT", $"/game_states/?id={stateId}", $"{{\"gameId\":{gameId},\"userId\":{userId},\"state\":\"boss\"}}")).Status);
        Assert.Equal(204, (await api.Handle("DELETE", $"/game_states/?id={stateId}", null)).Status);
        var contactId = Id(await api.Handle("POST", "/contacts/", $"{{\"firstName\":\"Ada\",\"lastName\":\"Lovelace\",\"email\":\"ada@example.com\",\"notes\":\"poet\",\"contactSourceName\":\"Manual\",\"userId\":\"{userId}\",\"uuid\":\"22222222-2222-2222-2222-222222222222\",\"created\":\"2020-01-02T03:04:05Z\",\"version\":\"AA==\"}}"));
        Assert.Equal(200, (await api.Handle("GET", $"/contacts/?userId={userId}", null)).Status);
        Assert.Equal(200, (await api.Handle("PUT", $"/contacts/?id={contactId}", $"{{\"firstName\":\"Ada\",\"lastName\":\"Lovelace\",\"contactSourceName\":\"Manual\",\"userId\":\"{userId}\"}}")).Status);
        Assert.Equal(204, (await api.Handle("DELETE", $"/contacts/?id={contactId}", null)).Status);
        Assert.Equal(201, (await api.Handle("POST", "/users/", """{"username":"x","password":"y","uuid":"","created":"0001-01-01T00:00:00","version":null,"settings":null,"roles":null,"groups":null}""")).Status);
        Assert.Equal(201, (await api.Handle("POST", "/games/", """{"name":"z","settings":null,"categories":null}""")).Status);
        Assert.Equal(201, (await api.Handle("POST", "/contacts/", """{"firstName":"A","lastName":"B","notes":"","uuid":"","created":"0001-01-01T00:00:00","version":null,"addresses":null,"phones":null,"settings":null,"prefs":null}""")).Status);
        Assert.Equal(201, (await api.Handle("POST", "/users/", " ")).Status);
        Assert.Equal(200, (await api.Handle("GET", "/users/?id=&lonely&username=x", null)).Status);
        Assert.Equal(400, (await api.Handle("GET", "/users/?id=1&id=2", null)).Status);
        Clock.Now = () => new DateTime(2020, 1, 2, 3, 4, 5, DateTimeKind.Utc);
        Clock.Uuid = () => "00000000-0000-0000-0000-000000000000";
        Assert.Equal("00000000-0000-0000-0000-000000000000", Clock.Uuid());
        Assert.Equal(new DateTime(2020, 1, 2, 3, 4, 5, DateTimeKind.Utc), Clock.Now());
    }

    sealed record Opened(Api Api, IConnection Connection);

    static long Id((int Status, string Body) result)
    {
        Assert.Equal(201, result.Status);
        return JsonSerializer.Deserialize<JsonElement>(result.Body).GetProperty("id").GetInt64();
    }

    static async Task<Opened> Open()
    {
        var ds = new Datasource("sqlite", new SqliteDriver(), new Dictionary<string, string?> { ["DATASOURCE_URL"] = "sqlite:///:memory:" });
        var connection = await ds.CreateConnection();
        await connection.Open();
        return new(
            new Api(
                new UsersRepository(connection),
                new CategoriesRepository(connection),
                new GamesRepository(connection),
                new ReviewsRepository(connection),
                new GameStatesRepository(connection),
                new ContactsRepository(connection)),
            connection);
    }

    static async Task<Opened> Migrated()
    {
        var opened = await Open();
        var sql = await File.ReadAllTextAsync(Path.Combine(Root(), "sql/sqlite/migrations/0001_initial_up.sql"));
        foreach (var batch in sql.Split("GO", StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries))
            await opened.Connection.CreateCommand().Execute(Native.N(batch));
        return opened;
    }

    static string Root()
    {
        var dir = AppContext.BaseDirectory;
        while (dir is not null && !File.Exists(Path.Combine(dir, "sql/sqlite/migrations/0001_initial_up.sql")))
            dir = Directory.GetParent(dir)?.FullName;
        return dir ?? throw new InvalidOperationException("sql/sqlite/migrations/0001_initial_up.sql");
    }
}
