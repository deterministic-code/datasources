using System.Text.Json;
using Backend.Types.Datasource;
using Backend.Types.View;
using Datasources;
using Categories = Backend.Types.View.Categories;

namespace ComplexGameRest;

public sealed class Api
{
    readonly ICRUDRepository<Users> users;
    readonly ICRUDRepository<Categories> categories;
    readonly ICRUDRepository<Games> games;
    readonly ICRUDRepository<Reviews> reviews;
    readonly ICRUDRepository<GameStates> states;
    readonly ICRUDRepository<Contact> contacts;

    public Api(
        ICRUDRepository<Users> users,
        ICRUDRepository<Categories> categories,
        ICRUDRepository<Games> games,
        ICRUDRepository<Reviews> reviews,
        ICRUDRepository<GameStates> states,
        ICRUDRepository<Contact> contacts)
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
                "POST /users" => Post(users, StampUser(Read<Users>(body))),
                "PUT /users" => Put(users, query, StampUser(Read<Users>(body))),
                "DELETE /users" => Delete(users, query),
                "GET /categories" => GetCategories(query),
                "POST /categories" => Post(categories, StampCategory(Read<Categories>(body))),
                "PUT /categories" => Put(categories, query, StampCategory(Read<Categories>(body))),
                "DELETE /categories" => Delete(categories, query),
                "GET /games" => Get(games, query, ["Id", "Name"]),
                "POST /games" => Post(games, StampGame(Read<Games>(body))),
                "PUT /games" => Put(games, query, StampGame(Read<Games>(body))),
                "DELETE /games" => Delete(games, query),
                "GET /reviews" => Get(reviews, query, ["Id", "GameId", "UserId"]),
                "POST /reviews" => Post(reviews, StampReview(Read<Reviews>(body))),
                "PUT /reviews" => Put(reviews, query, StampReview(Read<Reviews>(body))),
                "DELETE /reviews" => Delete(reviews, query),
                "GET /game_states" => Get(states, query, ["Id", "GameId", "UserId"]),
                "POST /game_states" => Post(states, StampState(Read<GameStates>(body))),
                "PUT /game_states" => Put(states, query, StampState(Read<GameStates>(body))),
                "DELETE /game_states" => Delete(states, query),
                "GET /contacts" => Get(contacts, query, ["Id", "UserId"]),
                "POST /contacts" => Post(contacts, StampContact(Read<Contact>(body))),
                "PUT /contacts" => Put(contacts, query, StampContact(Read<Contact>(body))),
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

    static async Task<(int Status, string Body)> Put<T>(ICRUDRepository<T> repo, Dictionary<string, string> query, T row)
    {
        var id = RequiredNumber(query, "id");
        if (await repo.ReadSingle(new Where(new Datasources.Operator.Equals("Id", id))) is null)
            return (404, "");
        SetId(row, id);
        await repo.Update(row);
        return (200, JsonObject.ToString(await repo.ReadSingle(new Where(new Datasources.Operator.Equals("Id", id)))));
    }

    static async Task<(int Status, string Body)> Delete<T>(ICRUDRepository<T> repo, Dictionary<string, string> query)
    {
        var id = RequiredNumber(query, "id");
        var row = await repo.ReadSingle(new Where(new Datasources.Operator.Equals("Id", id)));
        if (row is null) return (404, "");
        await repo.Delete(row);
        return (204, "");
    }

    static Users StampUser(Users user)
    {
        Stamp(user);
        user.Settings ??= [];
        user.Roles ??= [];
        user.Groups ??= [];
        foreach (var role in user.Roles) Stamp(role);
        foreach (var group in user.Groups) Stamp(group);
        return user;
    }

    static Categories StampCategory(Categories category)
    {
        Stamp(category);
        return category;
    }

    static Games StampGame(Games game)
    {
        Stamp(game);
        game.Settings ??= [];
        game.Categories ??= [];
        foreach (var category in game.Categories) Stamp(category);
        return game;
    }

    static Reviews StampReview(Reviews review)
    {
        Stamp(review);
        return review;
    }

    static GameStates StampState(GameStates state)
    {
        Stamp(state);
        return state;
    }

    static Contact StampContact(Contact contact)
    {
        contact.Email = contact.Email ?? "";
        contact.Notes = string.IsNullOrEmpty(contact.Notes) ? "none" : contact.Notes;
        contact.Uuid = string.IsNullOrEmpty(contact.Uuid) ? Clock.Uuid() : contact.Uuid;
        contact.Created = contact.Created == default ? Clock.Now() : contact.Created;
        contact.Updated = Clock.Now();
        contact.Version ??= [0];
        contact.Addresses ??= [];
        contact.Phones ??= [];
        contact.Settings ??= [];
        contact.Prefs ??= [];
        return contact;
    }

    static void Stamp(Base row)
    {
        row.Uuid = string.IsNullOrEmpty(row.Uuid) ? Clock.Uuid() : row.Uuid;
        row.Created = row.Created == default ? Clock.Now() : row.Created;
        row.Updated = Clock.Now();
        row.Version ??= [0];
    }

    static void SetId<T>(T row, long id)
    {
        if (row is Users user) user.Id = id;
        else if (row is Categories category) category.Id = id;
        else if (row is Games game) game.Id = id;
        else if (row is Reviews review) review.Id = id;
        else if (row is GameStates state) state.Id = id;
        else if (row is Contact contact) contact.Id = id;
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
