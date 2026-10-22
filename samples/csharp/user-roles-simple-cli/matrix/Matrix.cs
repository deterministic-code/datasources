using Datasources;
using Datasources.Mysql;
using Datasources.Postgres;
using Datasources.Sqlite;
using UserRolesSimpleCli;
using YamlDotNet.Serialization;
using YamlDotNet.Serialization.NamingConventions;

var provider = Environment.GetEnvironmentVariable("PROVIDER") ?? "sqlite";
var url = Environment.GetEnvironmentVariable("DATASOURCE_URL") ?? "sqlite:///:memory:";
var log = Environment.GetEnvironmentVariable("E2E_MATRIX_LOG") ?? throw new InvalidOperationException("E2E_MATRIX_LOG");
var spec = new DeserializerBuilder()
    .WithNamingConvention(CamelCaseNamingConvention.Instance)
    .IgnoreUnmatchedProperties()
    .Build()
    .Deserialize<CliSpec>(File.ReadAllText(Path.Combine(Shared(), "cli.yaml")));
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
    var now = DateTimeOffset.Parse(spec.Now, null, System.Globalization.DateTimeStyles.RoundtripKind);
    command.AddMiddleware(new ConsoleCommandMiddleware(lines.Add, () => now, new Dictionary<string, string> { ["isVerbose"] = "true" }));
    var app = new App(command, () => now.UtcDateTime, _ => "hash", () => [1, 2, 3], false);
    foreach (var step in spec.Steps)
        await app.Run(Parse.Args([.. step.Argv]));
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
        var path = Path.Combine(dir.FullName, "samples", "user-roles-simple-cli-shared");
        if (Directory.Exists(path)) return path;
    }
    throw new InvalidOperationException("shared spec");
}

sealed class CliSpec
{
    public string Now { get; set; } = "";
    public List<CliStep> Steps { get; set; } = [];
}

sealed class CliStep
{
    public List<string> Argv { get; set; } = [];
}
