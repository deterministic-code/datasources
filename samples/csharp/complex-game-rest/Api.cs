using System.Text.Json;
using Backend.Types.Datasource;
using Backend.Types.View;
using Datasources;
using Category = Backend.Types.View.Category;
using Game = Backend.Types.View.Game;
using User = Backend.Types.View.User;

namespace ComplexGameRest;

public sealed class Api
{
    readonly UsersRepository users;
    readonly CategoriesRepository categories;
    readonly GamesRepository games;
    readonly ReviewsRepository reviews;
    readonly GameStatesRepository states;
    readonly ContactsRepository contacts;

    public Api(
        UsersRepository users,
        CategoriesRepository categories,
        GamesRepository games,
        ReviewsRepository reviews,
        GameStatesRepository states,
        ContactsRepository contacts)
    {
        this.users = users;
        this.categories = categories;
        this.games = games;
        this.reviews = reviews;
        this.states = states;
        this.contacts = contacts;
    }

    public async Task<(int Status, string Body)> Handle(string method, string target, string? body)
    {
        try
        {
            var split = target.IndexOf('?');
            var raw = (split < 0 ? target : target[..split]).TrimEnd('/');
            var path = raw.Length == 0 ? "/" : raw;
            var query = Parse(split < 0 ? "" : target[(split + 1)..]);
            return await ($"{method.ToUpperInvariant()} {path}" switch
            {
                "GET /users" => Get(users, query, ["Id", "Username"]),
                "POST /users" => Post(users, Read<User>(body)),
                "PUT /users" => Put(users, query, body, (incoming, current) => incoming with { Id = current.Id }),
                "DELETE /users" => Delete(users, query),
                "GET /categories" => GetCategories(query),
                "POST /categories" => Post(categories, Read<Category>(body)),
                "PUT /categories" => Put(categories, query, body, (incoming, current) => incoming with { Id = current.Id }),
                "DELETE /categories" => Delete(categories, query),
                "GET /games" => Get(games, query, ["Id", "Name"]),
                "POST /games" => Post(games, Read<Game>(body)),
                "PUT /games" => Put(games, query, body, (incoming, current) => incoming with { Id = current.Id }),
                "DELETE /games" => Delete(games, query),
                "GET /reviews" => Get(reviews, query, ["Id", "GameId", "UserId"]),
                "POST /reviews" => Post(reviews, Read<Review>(body)),
                "PUT /reviews" => Put(reviews, query, body, (incoming, current) => incoming with { Id = current.Id }),
                "DELETE /reviews" => Delete(reviews, query),
                "GET /game_states" => Get(states, query, ["Id", "GameId", "UserId"]),
                "POST /game_states" => Post(states, Read<GameState>(body)),
                "PUT /game_states" => Put(states, query, body, (incoming, current) => incoming with { Id = current.Id }),
                "DELETE /game_states" => Delete(states, query),
                "GET /contacts" => Get(contacts, query, ["Id", "UserId"]),
                "POST /contacts" => Post(contacts, Read<Contact>(body)),
                "PUT /contacts" => Put(contacts, query, body, (incoming, current) => incoming with { Id = current.Id }),
                "DELETE /contacts" => Delete(contacts, query),
                _ => Task.FromResult((404, "")),
            });
        }
        catch (RequestError error)
        {
            return (error.Status, error.Message);
        }
        catch (Exception error) when (error is DatasourceException or JsonException or ArgumentException or FormatException)
        {
            return (400, error.Message);
        }
    }

    async Task<(int Status, string Body)> GetCategories(Dictionary<string, string> query)
    {
        var raw = query.GetValueOrDefault("categoryId") ?? query.GetValueOrDefault("category_id") ?? "";
        IWhere filter = raw.Length == 0
            ? new Where(new Datasources.Operator.IsNull(new Identifier("CategoryId")))
            : new Where(new Datasources.Operator.Equals("CategoryId", long.Parse(raw)));
        return (200, JsonObject.ToString(await List(categories, filter)));
    }

    static async Task<(int Status, string Body)> Get<T>(ICRUDRepository<T> repo, Dictionary<string, string> query, string[] keys) =>
        (200, JsonObject.ToString(await List(repo, WhereOf(query, keys))));

    static async Task<(int Status, string Body)> Post<T>(ICRUDRepository<T> repo, T row) =>
        (201, JsonObject.ToString(await repo.Create(row)));

    static async Task<(int Status, string Body)> Put<T>(
        ICRUDRepository<T> repo,
        Dictionary<string, string> query,
        string? body,
        Func<T, T, T> withId) where T : new()
    {
        var id = RequiredNumber(query, "id");
        var by = new Where(new Datasources.Operator.Equals("Id", id));
        if (await repo.ReadSingle(by) is not { } current)
            return (404, "");
        await repo.Update(withId(Read<T>(body), current));
        return (200, JsonObject.ToString(await repo.ReadSingle(by)));
    }

    static async Task<(int Status, string Body)> Delete<T>(ICRUDRepository<T> repo, Dictionary<string, string> query)
    {
        var id = RequiredNumber(query, "id");
        var row = await repo.ReadSingle(new Where(new Datasources.Operator.Equals("Id", id)));
        if (row is null) return (404, "");
        await repo.Delete(row);
        return (204, "");
    }

    static async Task<List<T>> List<T>(IReadonlyRepository<T> repo, IWhere? query)
    {
        var rows = new List<T>();
        await foreach (var row in query is null ? repo.Read() : repo.Read(query)) rows.Add(row);
        return rows;
    }

    static IWhere? WhereOf(Dictionary<string, string> query, string[] keys)
    {
        IExpression? combined = null;
        foreach (var key in keys.Where(key => query.GetValueOrDefault(ToCamel(key), "").Length > 0))
        {
            var raw = query[ToCamel(key)];
            IExpression part = new Datasources.Operator.Equals(key, NumberKey(key) ? long.Parse(raw) : (Value)raw);
            combined = combined is null ? part : new Datasources.Operator.And(combined, part);
        }
        return combined is null ? null : new Where(combined);
    }

    static bool NumberKey(string key) => key is "Id" or "UserId" or "GameId" or "CategoryId";

    static string ToCamel(string name) => char.ToLowerInvariant(name[0]) + name[1..];

    static T Read<T>(string? body) where T : new() => JsonObject.FromString<T>(body);

    static long RequiredNumber(Dictionary<string, string> query, string key) =>
        long.TryParse(Required(query, key), out var value) ? value : throw new RequestError($"{key} is required");

    static string Required(Dictionary<string, string> query, string key) =>
        (query.GetValueOrDefault(key) ?? "").Length > 0 ? query[key] : throw new RequestError($"{key} is required");

    static Dictionary<string, string> Parse(string query) =>
        query.Split('&', StringSplitOptions.RemoveEmptyEntries)
            .Select(part =>
            {
                var eq = part.IndexOf('=');
                return (
                    Uri.UnescapeDataString(eq < 0 ? part : part[..eq]),
                    Uri.UnescapeDataString(eq < 0 ? "" : part[(eq + 1)..]));
            })
            .ToDictionary(pair => pair.Item1, pair => pair.Item2);

    sealed class RequestError(string message, int status = 400) : Exception(message)
    {
        public int Status { get; } = status;
    }
}
