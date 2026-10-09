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
    var command = connection.CreateCommand();
    if (provider == "sqlite") await command.Execute(new Native("PRAGMA foreign_keys = ON"));
    foreach (var sql in Statements(await File.ReadAllTextAsync(Environment.GetEnvironmentVariable("E2E_MATRIX_SQL") ?? throw new InvalidOperationException("E2E_MATRIX_SQL"))))
        await command.Execute(new Native(sql));
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
    var id = "";
    foreach (var step in spec.Steps)
    {
        var path = step.Path.Replace("$id", id);
        var body = step.Body is null ? null : JsonSerializer.Serialize(Normalize(step.Body));
        var result = await api.Handle(step.Method, path, body);
        if (step.SaveId)
            id = JsonDocument.Parse(result.Body).RootElement.GetProperty("id").ToString();
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

static IEnumerable<string> Statements(string sql)
{
    var cleaned = string.Join('\n', sql.Split('\n').Where(line => !line.TrimStart().StartsWith("--", StringComparison.Ordinal)));
    var depth = 0;
    var start = 0;
    var dollar = false;
    for (var i = 0; i < cleaned.Length; i++)
    {
        if (i + 1 < cleaned.Length && cleaned[i] == '$' && cleaned[i + 1] == '$')
        {
            dollar = !dollar;
            i++;
            continue;
        }
        if (dollar) continue;
        if (Word(cleaned, i, "BEGIN")) depth++;
        if (cleaned[i] != ';') continue;
        if (EndsWithWord(cleaned, start, i, "END") && depth > 0) depth--;
        if (depth != 0) continue;
        var text = WithSeedVersion(cleaned[start..(i + 1)].Trim());
        if (text.Length > 0) yield return text;
        start = i + 1;
    }
}

static string WithSeedVersion(string sql)
{
    if (!sql.StartsWith("INSERT INTO", StringComparison.Ordinal) || sql.Contains("\"version\"") || sql.Contains("`version`"))
        return sql;
    var values = sql.IndexOf(") VALUES (", StringComparison.Ordinal);
    if (values < 0) return sql;
    var quote = sql.Contains('`') ? "`" : "\"";
    return $"{sql[..values]}, {quote}version{quote}) VALUES ({sql[(values + ") VALUES (".Length)..^2]}, X'00');";
}

static bool Word(string sql, int i, string token)
{
    if (!sql.AsSpan(i).StartsWith(token, StringComparison.Ordinal)) return false;
    if (i > 0 && char.IsLetterOrDigit(sql[i - 1])) return false;
    var end = i + token.Length;
    return end == sql.Length || !char.IsLetterOrDigit(sql[end]);
}

static bool EndsWithWord(string sql, int start, int semicolon, string token)
{
    var i = semicolon - 1;
    while (i >= start && char.IsWhiteSpace(sql[i])) i--;
    if (i < start + token.Length - 1 || sql.AsSpan(i - token.Length + 1, token.Length).ToString() != token)
        return false;
    var before = i - token.Length;
    return before < start || !char.IsLetterOrDigit(sql[before]);
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
}
