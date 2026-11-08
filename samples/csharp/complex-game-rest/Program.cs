using System.Text.Json;
using Backend.Types.Datasource;
using ComplexGameRest;
using Datasources;

await using var ds = Datasource.Default;
var connection = await ds.CreateConnection();
await connection.Open();
var api = new Api(
    new UsersRepository(connection),
    new CategoriesRepository(connection),
    new GamesRepository(connection),
    new ReviewsRepository(connection),
    new GameStatesRepository(connection),
    new ContactsRepository(connection));
var roles = new RolesRepository(connection);
var groups = new GroupsRepository(connection);
var sources = new ContactSourcesRepository(connection);
var role = await roles.Create(new Role { Name = "player", Description = "plays" });
var group = await groups.Create(new Group { Name = "guild", Description = "guild" });
await sources.Create(new ContactSource { Name = "Manual" });
var addressTypes = new AddressTypesRepository(connection);
await addressTypes.Create(new AddressType { Name = "Home" });
await addressTypes.Create(new AddressType { Name = "Work" });
await new PhoneTypesRepository(connection).Create(new PhoneType { Name = "Mobile" });

var user = await Call(api, "POST", "/users", Json(new
{
    username = "ada",
    password = "secret",
    settings = new Dictionary<string, string> { ["theme"] = "dark" },
    roles = new[] { new { name = role.Name, description = role.Description } },
    groups = new[] { new { name = group.Name, description = group.Description } },
}), 201);
var userId = user.GetProperty("id").GetInt64();
var listed = await Call(api, "GET", "/users?username=ada", null, 200);
if (listed.GetArrayLength() != 1) throw new InvalidOperationException("get users username");
await Call(api, "PUT", $"/users?id={userId}", Json(new
{
    username = "ada-byron",
    password = "secret",
    settings = new Dictionary<string, string> { ["theme"] = "sepia" },
    roles = new[] { new { name = role.Name } },
    groups = new[] { new { name = group.Name } },
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
    addresses = new object[]
    {
        new { addressTypeName = "Home", line1 = "12 Analytical Engine", city = "London", country = "GB" },
        new { addressTypeName = "Work", line1 = "1 Strand", city = "London", country = "GB" },
    },
    phones = new object[] { new { phoneTypeName = "Mobile", number = "555-0100" } },
}), 201);
var contactId = contact.GetProperty("id").GetInt64();
var created = await Call(api, "GET", $"/contacts?id={contactId}", null, 200);
RequireAdaChildren(created, "Home", "London", "555-0100");
if (AddressByLine1(ContactRow(created), "1 Strand").GetProperty("addressTypeName").GetString() != "Work")
    throw new InvalidOperationException("create work address");
var engine = AddressByLine1(ContactRow(created), "12 Analytical Engine");
var phone = ContactRow(created).GetProperty("phones")[0];
await Call(api, "PUT", $"/contacts?id={contactId}", Json(new
{
    firstName = "Ada Byron",
    lastName = "Lovelace",
    contactSourceName = "Manual",
    userId = userId.ToString(),
    addresses = new object[]
    {
        new
        {
            id = engine.GetProperty("id").GetInt64(),
            addressTypeName = "Work",
            line1 = "12 Analytical Engine",
            city = "Cambridge",
            country = "FR",
        },
        new { addressTypeName = "Home", line1 = "1 College", city = "Cambridge", country = "GB" },
    },
    phones = new object[]
    {
        new
        {
            id = phone.GetProperty("id").GetInt64(),
            phoneTypeName = "Mobile",
            number = "555-0111",
        },
    },
}), 200);
var updated = await Call(api, "GET", $"/contacts?id={contactId}", null, 200);
RequireAdaChildren(updated, "Work", "Cambridge", "555-0111");
if (AddressByLine1(ContactRow(updated), "1 College").GetProperty("addressTypeName").GetString() != "Home")
    throw new InvalidOperationException("update home address");

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

static JsonElement ContactRow(JsonElement listed)
{
    if (listed.ValueKind != JsonValueKind.Array || listed.GetArrayLength() != 1)
        throw new InvalidOperationException("contact list");
    return listed[0];
}

static JsonElement AddressByLine1(JsonElement contact, string line1)
{
    foreach (var address in contact.GetProperty("addresses").EnumerateArray())
        if (address.GetProperty("line1").GetString() == line1) return address;
    throw new InvalidOperationException(line1);
}

static void RequireAdaChildren(JsonElement listed, string engineType, string engineCity, string number)
{
    var contact = ContactRow(listed);
    if (contact.GetProperty("addresses").GetArrayLength() != 2
        || contact.GetProperty("phones").GetArrayLength() != 1)
        throw new InvalidOperationException("child counts");
    var engine = AddressByLine1(contact, "12 Analytical Engine");
    if (engine.GetProperty("addressTypeName").GetString() != engineType
        || engine.GetProperty("city").GetString() != engineCity)
        throw new InvalidOperationException("engine address");
    if (engine.GetProperty("contactId").ValueKind != JsonValueKind.Number
        || engine.GetProperty("contactId").GetInt64() != contact.GetProperty("id").GetInt64())
        throw new InvalidOperationException("address contact id");
    var phone = contact.GetProperty("phones")[0];
    if (phone.GetProperty("number").GetString() != number
        || phone.GetProperty("phoneTypeName").GetString() != "Mobile")
        throw new InvalidOperationException("phone");
    if (phone.GetProperty("contactId").ValueKind != JsonValueKind.Number
        || phone.GetProperty("contactId").GetInt64() != contact.GetProperty("id").GetInt64())
        throw new InvalidOperationException("phone contact id");
}

static string Json(object value) => JsonSerializer.Serialize(value);

static async Task<JsonElement> Call(Api api, string method, string target, string? body, int status)
{
    var (got, text) = await api.Handle(method, target, body);
    if (got != status) throw new InvalidOperationException($"{method} {target} => {got} {text}");
    return text.Length == 0 ? default : JsonSerializer.Deserialize<JsonElement>(text);
}
