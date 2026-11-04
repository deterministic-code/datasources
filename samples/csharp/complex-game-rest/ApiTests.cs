using Backend.Types.Datasource;
using Backend.Types.View;
using Categories = Backend.Types.View.Categories;
using ComplexGameRest;
using Datasources;
using Xunit;

public sealed class ApiTests
{
    [Fact]
    public async Task MapsDatasourceErrors()
    {
        var users = Repo<Users>();
        users.Error = new DatasourceException("missing");
        var api = new Api(users, Repo<Categories>(), Repo<Games>(), Repo<Reviews>(), Repo<GameStates>(), Repo<Contact>());
        var result = await api.Handle("GET", "/users", null);
        Assert.Equal(400, result.Status);
        Assert.Equal("missing", result.Body);
    }

    [Fact]
    public async Task RethrowsUnexpectedErrors()
    {
        var users = Repo<Users>();
        users.Error = new InvalidOperationException("boom");
        var api = new Api(users, Repo<Categories>(), Repo<Games>(), Repo<Reviews>(), Repo<GameStates>(), Repo<Contact>());
        await Assert.ThrowsAsync<InvalidOperationException>(() => api.Handle("GET", "/users", null));
    }

    [Fact]
    public async Task RejectsUnknownRoutesLookupsAndMissingRows()
    {
        var api = new Api(Repo<Users>(), Repo<Categories>(), Repo<Games>(), Repo<Reviews>(), Repo<GameStates>(), Repo<Contact>());
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
        var users = Repo(new Users { Id = 1, Username = "ada", Settings = [], Roles = [], Groups = [] });
        var categories = Repo(new Categories { Id = 2, Name = "arcade" });
        var games = Repo(new Games { Id = 3, Name = "star", Settings = [], Categories = [] });
        var reviews = Repo(new Reviews { Id = 4, Description = "fun" });
        var states = Repo(new GameStates { Id = 5, State = "intro" });
        var contacts = Repo(new Contact { Id = 6, FirstName = "Ada" });
        var api = new Api(users, categories, games, reviews, states, contacts);
        Assert.Equal(201, (await api.Handle("POST", "/users/", """{"username":"ada","password":"x","uuid":"11111111-1111-1111-1111-111111111111","created":"2020-01-02T03:04:05Z","version":"AA==","settings":{"theme":"dark"},"roles":[{"name":"player"}],"groups":[{"name":"guild"}]}""")).Status);
        Assert.Equal(200, (await api.Handle("GET", "/users", null)).Status);
        Assert.Equal(200, (await api.Handle("GET", "/users/?id=1&username=ada", null)).Status);
        Assert.Equal(200, (await api.Handle("PUT", "/users/?id=1", """{"username":"ada","password":"x"}""")).Status);
        Assert.Equal(204, (await api.Handle("DELETE", "/users/?id=1", null)).Status);
        Assert.Equal(200, (await api.Handle("GET", "/categories", null)).Status);
        Assert.Equal(200, (await api.Handle("GET", "/categories?categoryId=", null)).Status);
        Assert.Equal(200, (await api.Handle("GET", "/categories?category_id=2", null)).Status);
        Assert.Equal(201, (await api.Handle("POST", "/categories/", """{"name":"arcade"}""")).Status);
        Assert.Equal(200, (await api.Handle("PUT", "/categories/?id=2", """{"name":"arcade"}""")).Status);
        Assert.Equal(204, (await api.Handle("DELETE", "/categories/?id=2", null)).Status);
        Assert.Equal(201, (await api.Handle("POST", "/games/", """{"name":"star","settings":{"lives":"3"},"categories":[{"name":"arcade"}]}""")).Status);
        Assert.Equal(200, (await api.Handle("GET", "/games/?id=3&name=star", null)).Status);
        Assert.Equal(200, (await api.Handle("PUT", "/games/?id=3", """{"name":"star"}""")).Status);
        Assert.Equal(204, (await api.Handle("DELETE", "/games/?id=3", null)).Status);
        Assert.Equal(201, (await api.Handle("POST", "/reviews/", """{"gameId":3,"userId":1,"description":"fun","rating":"4.5"}""")).Status);
        Assert.Equal(200, (await api.Handle("GET", "/reviews/?gameId=3&userId=1", null)).Status);
        Assert.Equal(200, (await api.Handle("PUT", "/reviews/?id=4", """{"description":"great"}""")).Status);
        Assert.Equal(204, (await api.Handle("DELETE", "/reviews/?id=4", null)).Status);
        Assert.Equal(201, (await api.Handle("POST", "/game_states/", """{"gameId":3,"userId":1,"state":"intro"}""")).Status);
        Assert.Equal(200, (await api.Handle("GET", "/game_states/?id=5", null)).Status);
        Assert.Equal(200, (await api.Handle("PUT", "/game_states/?id=5", """{"state":"boss"}""")).Status);
        Assert.Equal(204, (await api.Handle("DELETE", "/game_states/?id=5", null)).Status);
        Assert.Equal(201, (await api.Handle("POST", "/contacts/", """{"firstName":"Ada","lastName":"Lovelace","email":"ada@example.com","notes":"poet","contactSourceName":"Manual","userId":"1","uuid":"22222222-2222-2222-2222-222222222222","created":"2020-01-02T03:04:05Z","version":"AA=="}""")).Status);
        Assert.Equal(200, (await api.Handle("GET", "/contacts/?userId=1", null)).Status);
        Assert.Equal(200, (await api.Handle("PUT", "/contacts/?id=6", """{"firstName":"Ada","lastName":"Lovelace","contactSourceName":"Manual","userId":"1"}""")).Status);
        Assert.Equal(204, (await api.Handle("DELETE", "/contacts/?id=6", null)).Status);
        Assert.Equal(201, (await api.Handle("POST", "/users/", """{"username":"x","password":"y","uuid":"","created":"0001-01-01T00:00:00","version":null,"settings":null,"roles":null,"groups":null}""")).Status);
        Assert.Equal(201, (await api.Handle("POST", "/games/", """{"name":"z","settings":null,"categories":null}""")).Status);
        Assert.Equal(201, (await api.Handle("POST", "/contacts/", """{"firstName":"A","lastName":"B","notes":"","uuid":"","created":"0001-01-01T00:00:00","version":null,"addresses":null,"phones":null,"settings":null,"prefs":null}""")).Status);
        Assert.Equal(201, (await api.Handle("POST", "/users/", " ")).Status);
        Assert.Equal(200, (await api.Handle("GET", "/users/?id=&lonely&username=ada", null)).Status);
        Assert.Equal(400, (await api.Handle("GET", "/users/?id=1&id=2", null)).Status);
        Clock.Now = () => new DateTime(2020, 1, 2, 3, 4, 5, DateTimeKind.Utc);
        Clock.Uuid = () => "00000000-0000-0000-0000-000000000000";
        Assert.Equal("00000000-0000-0000-0000-000000000000", Clock.Uuid());
        Assert.Equal(new DateTime(2020, 1, 2, 3, 4, 5, DateTimeKind.Utc), Clock.Now());
    }

    static Fake<T> Repo<T>(T? row = default) where T : class, new() => new(row);

    sealed class Fake<T>(T? row) : ICRUDRepository<T> where T : class, new()
    {
        public Exception? Error { get; set; }
        public Type Type => typeof(T);
        public bool IsSchemaRequired => false;
        public IReadOnlyList<string> Ids => ["Id"];

        public async IAsyncEnumerable<T> Read(IStatement? query = null, IReadOnlyList<Parameter>? parameters = null, ReadonlyRepositoryMappingOptions<T>? options = null)
        {
            if (Error is not null) throw Error;
            if (row is not null) yield return row;
            await Task.CompletedTask;
        }

        public IAsyncEnumerable<T> Read(IWhere where, ReadonlyRepositoryMappingOptions<T>? options = null) =>
            Read();

        public Task<T?> ReadSingle(IStatement? query = null, IReadOnlyList<Parameter>? parameters = null, ReadonlyRepositoryMappingOptions<T>? options = null) =>
            Task.FromResult(row);

        public Task<T?> ReadSingle(IWhere where, ReadonlyRepositoryMappingOptions<T>? options = null) =>
            Task.FromResult(row);

        public Task<T> Create(T item) => Task.FromResult(row ?? item);

        public Task<Response> Update(T item, IWhere? byWhere = null) =>
            Task.FromResult(new Response(Value.Null.Instance, 1));

        public Task<Response> Delete(T item, IWhere? byWhere = null) =>
            Task.FromResult(new Response(Value.Null.Instance, 1));

        public Task<T?> ReadSingleById(Value id) => Task.FromResult(row);

        public Task<T?> ReadSingleById(Row ids) => Task.FromResult(row);
    }
}
