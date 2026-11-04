using System.Text.Json;
using Backend.Types.Datasource;
using Backend.Types.View;
using Categories = Backend.Types.View.Categories;
using ComplexGameRest;
using Datasources;

await using var ds = Datasource.Default;
var api = new Api(
    ds.CreateRepository<Users>("Users"),
    ds.CreateRepository<Categories>("Categories"),
    ds.CreateRepository<Games>("Games"),
    ds.CreateRepository<Reviews>("Reviews"),
    ds.CreateRepository<GameStates>("GameStates"),
    ds.CreateRepository<Contact>("Contacts"));
var roles = ds.CreateRepository<Role>("Roles");
var groups = ds.CreateRepository<Group>("Groups");
var sources = ds.CreateRepository<ContactSource>("ContactSources");
var role = await roles.Create(new Role { Name = "player", Description = "plays" });
var group = await groups.Create(new Group { Name = "guild", Description = "guild" });
await sources.Create(new ContactSource { Name = "Manual" });

var user = await Call(api, "POST", "/users", Json(new
{
    username = "ada",
    password = "secret",
    settings = new Dictionary<string, string> { ["theme"] = "dark" },
    roles = new[] { new { id = role.Id, name = role.Name, description = role.Description } },
    groups = new[] { new { id = group.Id, name = group.Name, description = group.Description } },
}), 201);
var userId = user.GetProperty("id").GetInt64();
var listed = await Call(api, "GET", "/users?username=ada", null, 200);
if (listed.GetArrayLength() != 1) throw new InvalidOperationException("get users username");
await Call(api, "PUT", $"/users?id={userId}", Json(new
{
    username = "ada-byron",
    password = "secret",
    settings = new Dictionary<string, string> { ["theme"] = "sepia" },
    roles = new[] { new { id = role.Id, name = role.Name } },
    groups = new[] { new { id = group.Id, name = group.Name } },
}), 200);
var loaded = await Call(api, "GET", $"/users?id={userId}", null, 200);
if (loaded[0].GetProperty("username").GetString() != "ada-byron") throw new InvalidOperationException("put users");

var root = await Call(api, "POST", "/categories", Json(new { name = "arcade", description = "root" }), 201);
var rootId = root.GetProperty("id").GetInt64();
var child = await Call(api, "POST", "/categories", Json(new { name = "shooter", description = "child", categoryId = rootId }), 201);
var childId = child.GetProperty("id").GetInt64();
var roots = await Call(api, "GET", "/categories?category_id=", null, 200);
if (roots.GetArrayLength() != 1 || roots[0].GetProperty("id").GetInt64() != rootId)
    throw new InvalidOperationException("get category roots");
var children = await Call(api, "GET", $"/categories?categoryId={rootId}", null, 200);
if (children.GetArrayLength() != 1 || children[0].GetProperty("id").GetInt64() != childId)
    throw new InvalidOperationException("get category children");

var game = await Call(api, "POST", "/games", Json(new
{
    name = "star",
    description = "space",
    location = "orbit",
    settings = new Dictionary<string, string> { ["lives"] = "3" },
    categories = new[] { new { id = childId, name = "shooter" } },
}), 201);
var gameId = game.GetProperty("id").GetInt64();
await Call(api, "GET", $"/games?id={gameId}", null, 200);
await Call(api, "PUT", $"/games?id={gameId}", Json(new
{
    name = "star-2",
    description = "space",
    location = "orbit",
    settings = new Dictionary<string, string> { ["lives"] = "5" },
    categories = new[] { new { id = childId, name = "shooter" } },
}), 200);

var review = await Call(api, "POST", "/reviews", Json(new { gameId, userId, description = "fun", rating = "4.50", isApproved = true }), 201);
var reviewId = review.GetProperty("id").GetInt64();
await Call(api, "GET", $"/reviews?gameId={gameId}&userId={userId}", null, 200);
await Call(api, "PUT", $"/reviews?id={reviewId}", Json(new { gameId, userId, description = "great", rating = "5.00", isApproved = true }), 200);

var state = await Call(api, "POST", "/game_states", Json(new { gameId, userId, state = "intro" }), 201);
var stateId = state.GetProperty("id").GetInt64();
await Call(api, "GET", $"/game_states?gameId={gameId}&userId={userId}", null, 200);
await Call(api, "PUT", $"/game_states?id={stateId}", Json(new { gameId, userId, state = "boss" }), 200);

var contact = await Call(api, "POST", "/contacts", Json(new
{
    firstName = "Ada",
    lastName = "Lovelace",
    contactSourceName = "Manual",
    userId = userId.ToString(),
}), 201);
var contactId = contact.GetProperty("id").GetInt64();
await Call(api, "GET", $"/contacts?userId={userId}", null, 200);
await Call(api, "PUT", $"/contacts?id={contactId}", Json(new
{
    firstName = "Ada Byron",
    lastName = "Lovelace",
    contactSourceName = "Manual",
    userId = userId.ToString(),
}), 200);

foreach (var target in new[]
{
    $"/contacts?id={contactId}",
    $"/reviews?id={reviewId}",
    $"/game_states?id={stateId}",
    $"/games?id={gameId}",
    $"/categories?id={childId}",
    $"/categories?id={rootId}",
    $"/users?id={userId}",
})
{
    await Call(api, "DELETE", target, null, 204);
    await Call(api, "DELETE", target, null, 404);
    await Call(api, "PUT", target, "{}", 404);
}

static string Json(object value) => JsonSerializer.Serialize(value);

static async Task<JsonElement> Call(Api api, string method, string target, string? body, int status)
{
    var (got, text) = await api.Handle(method, target, body);
    if (got != status) throw new InvalidOperationException($"{method} {target} => {got} {text}");
    return text.Length == 0 ? default : JsonSerializer.Deserialize<JsonElement>(text);
}
