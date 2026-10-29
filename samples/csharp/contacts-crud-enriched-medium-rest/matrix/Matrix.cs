using System.Text.Json;
using ContactsCrudEnrichedMediumRest;
using Datasources;
using Datasources.Mysql;
using Datasources.Postgres;
using Datasources.Sqlite;
using YamlDotNet.Serialization;
using YamlDotNet.Serialization.NamingConventions;

var provider = Environment.GetEnvironmentVariable("PROVIDER") ?? "sqlite";
var url = Environment.GetEnvironmentVariable("DATASOURCE_URL") ?? "sqlite:///:memory:";
var log = Environment.GetEnvironmentVariable("E2E_MATRIX_LOG") ?? throw new InvalidOperationException("E2E_MATRIX_LOG");
var spec = new DeserializerBuilder()
    .WithNamingConvention(CamelCaseNamingConvention.Instance)
    .IgnoreUnmatchedProperties()
    .Build()
    .Deserialize<RoundTrip>(File.ReadAllText(Path.Combine(Shared(), "round-trip.yaml")));
var connection = provider switch
{
    "sqlite" => new SqliteDriver().Connection(new Dictionary<string, string?> { ["DATASOURCE_URL"] = url }),
    "mysql" => new MysqlDriver().Connection(new Dictionary<string, string?> { ["DATASOURCE_URL"] = url }),
    "postgres" => new PostgresDriver().Connection(new Dictionary<string, string?> { ["DATASOURCE_URL"] = url }),
    _ => throw new InvalidOperationException(provider)
};
var lines = new List<string>();
try
{
    await connection.Open();
    var stamp = DateTimeOffset.Parse(spec.Now, null, System.Globalization.DateTimeStyles.RoundtripKind);
    var n = 0;
    Clock.Now = () => stamp.UtcDateTime;
    Clock.Uuid = () => $"00000000-0000-0000-0000-{++n:D12}";
    var middleware = new ConsoleCommandMiddleware(lines.Add, () => stamp, new Dictionary<string, string> { ["isVerbose"] = "true" });
    var contacts = new ContactRepository(connection);
    var legacy = new LegacyContactRepository(connection);
    contacts.AddMiddleware(middleware);
    contacts.Address.AddMiddleware(middleware);
    contacts.Phone.AddMiddleware(middleware);
    contacts.ContactSource.AddMiddleware(middleware);
    contacts.ContactSettings.AddMiddleware(middleware);
    contacts.ContactPreferences.AddMiddleware(middleware);
    contacts.Address.AddressType.AddMiddleware(middleware);
    contacts.Phone.PhoneType.AddMiddleware(middleware);
    legacy.AddMiddleware(middleware);
    var api = new Api(contacts, legacy);
    var saved = new Dictionary<string, string>(StringComparer.Ordinal);
    foreach (var step in spec.Steps)
    {
        var path = SubstituteText(step.Path, saved);
        var body = step.Body is null ? null : JsonSerializer.Serialize(SubstituteNode(Normalize(step.Body), saved));
        var result = await api.Handle(step.Method, path, body);
        if (step.Expect is not null)
        {
            if (step.Expect.Status is { } status && result.Status != status)
                throw new InvalidOperationException($"{step.Method} {path}: expected status {status}, got {result.Status} {result.Body}");
            if (step.Expect.Body is not null)
                Contains(
                    result.Body.Length == 0 ? default : JsonDocument.Parse(result.Body).RootElement,
                    SubstituteNode(Normalize(step.Expect.Body), saved),
                    $"{step.Method} {path}");
        }
        if (step.SaveId)
            saved["id"] = JsonDocument.Parse(result.Body).RootElement.GetProperty("id").ToString();
        foreach (var (name, pointer) in step.Save)
            saved[name] = At(JsonDocument.Parse(result.Body).RootElement, pointer);
    }
}
finally
{
    Directory.CreateDirectory(Path.GetDirectoryName(log)!);
    await File.WriteAllTextAsync(log, lines.Count == 0 ? "" : string.Join('\n', lines) + "\n");
    try { await connection.Close(); } catch { }
}

static string Shared()
{
    for (var dir = new DirectoryInfo(AppContext.BaseDirectory); dir is not null; dir = dir.Parent)
    {
        var path = Path.Combine(dir.FullName, "samples", "contacts-crud-enriched-medium-rest-shared");
        if (Directory.Exists(path)) return path;
    }
    throw new InvalidOperationException("shared spec");
}

static object? Normalize(object? node) => node switch
{
    Dictionary<object, object> map => map.ToDictionary(entry => entry.Key.ToString()!, entry => Normalize(entry.Value)),
    Dictionary<string, object> map => map.ToDictionary(entry => entry.Key, entry => Normalize(entry.Value)),
    List<object> list => list.Select(Normalize).ToList(),
    _ => node
};

static string SubstituteText(string text, IReadOnlyDictionary<string, string> saved) =>
    saved.Aggregate(text, (current, pair) => current.Replace($"${pair.Key}", pair.Value));

static object? SubstituteNode(object? node, IReadOnlyDictionary<string, string> saved) => node switch
{
    Dictionary<string, object> map => map.ToDictionary(entry => entry.Key, entry => SubstituteNode(entry.Value, saved)),
    List<object> list => list.Select(item => SubstituteNode(item, saved)).ToList(),
    string text when text.StartsWith('$') && saved.TryGetValue(text[1..], out var value) =>
        long.TryParse(value, out var number) ? number : value,
    _ => node
};

static void Contains(JsonElement actual, object? expected, string path)
{
    switch (expected)
    {
        case null:
            if (actual.ValueKind is not JsonValueKind.Null and not JsonValueKind.Undefined)
                throw new InvalidOperationException($"{path}: expected null, got {actual}");
            break;
        case Dictionary<string, object> map:
            if (actual.ValueKind != JsonValueKind.Object)
                throw new InvalidOperationException($"{path}: expected object, got {actual}");
            foreach (var (key, value) in map)
                Contains(actual.TryGetProperty(key, out var child) ? child : default, value, $"{path}.{key}");
            break;
        case List<object> list:
            if (actual.ValueKind != JsonValueKind.Array)
                throw new InvalidOperationException($"{path}: expected array, got {actual}");
            if (actual.GetArrayLength() != list.Count)
                throw new InvalidOperationException($"{path}: expected {list.Count} items, got {actual.GetArrayLength()}");
            for (var i = 0; i < list.Count; i++)
                Contains(actual[i], list[i], $"{path}[{i}]");
            break;
        default:
            var want = expected is bool or long or int or double ? JsonSerializer.Serialize(expected) : expected.ToString();
            var got = actual.ValueKind == JsonValueKind.String ? actual.GetString() : actual.GetRawText();
            if (!string.Equals(want, got, StringComparison.Ordinal) && want != JsonSerializer.Serialize(got))
                throw new InvalidOperationException($"{path}: expected {want}, got {got}");
            break;
    }
}

static string At(JsonElement root, string pointer)
{
    var current = root;
    foreach (var part in pointer.Split('.'))
        current = int.TryParse(part, out var index) ? current[index] : current.GetProperty(part);
    return current.ValueKind switch
    {
        JsonValueKind.Number => current.GetRawText(),
        JsonValueKind.String => current.GetString() ?? "",
        _ => current.GetRawText()
    };
}

sealed class RoundTrip
{
    public string Now { get; set; } = "";
    public List<RoundTripStep> Steps { get; set; } = [];
}

sealed class RoundTripStep
{
    public string Method { get; set; } = "";
    public string Path { get; set; } = "";
    public object? Body { get; set; }
    public bool SaveId { get; set; }
    public Dictionary<string, string> Save { get; set; } = [];
    public RoundTripExpect? Expect { get; set; }
}

sealed class RoundTripExpect
{
    public int? Status { get; set; }
    public object? Body { get; set; }
}
