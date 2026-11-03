using Datasources;
using Datasources.Mysql;
using Datasources.Postgres;
using Datasources.Sqlite;
using UserRolesSimpleCli;
using YamlDotNet.Serialization;
using YamlDotNet.Serialization.NamingConventions;

var log = Environment.GetEnvironmentVariable("E2E_MATRIX_LOG") ?? throw new InvalidOperationException("E2E_MATRIX_LOG");
var spec = new DeserializerBuilder()
    .WithNamingConvention(CamelCaseNamingConvention.Instance)
    .IgnoreUnmatchedProperties()
    .Build()
    .Deserialize<CliSpec>(File.ReadAllText(Path.Combine(Shared(), "cli.yaml")));
DatasourceFactory.Register("sqlite", new SqliteDriver());
DatasourceFactory.Register("mysql", new MysqlDriver());
DatasourceFactory.Register("postgres", new PostgresDriver());
await using var ds = Datasource.Default;
var connection = await ds.CreateConnection();
var lines = new List<string>();
try
{
    await connection.Open();
    var command = await ds.CreateCommand(connection);
    var now = DateTimeOffset.Parse(spec.Now, null, System.Globalization.DateTimeStyles.RoundtripKind);
    command.AddMiddleware(new ConsoleCommandMiddleware(lines.Add, () => now, new Dictionary<string, string> { ["isVerbose"] = "true" }));
    foreach (var step in spec.Steps)
        await Program.Run(command, [.. step.Argv], _ => "hash", false);
}
finally
{
    Directory.CreateDirectory(Path.GetDirectoryName(log)!);
    await File.WriteAllTextAsync(log, lines.Count == 0 ? "" : string.Join('\n', lines) + "\n");
    try { await connection.Close(); } catch { }
}

static string Shared() =>
    Path.GetFullPath(Path.Combine(Directory.GetCurrentDirectory(), "..", "..", "..", "user-roles-simple-cli-shared"));

sealed class CliSpec
{
    public string Now { get; set; } = "";
    public List<CliStep> Steps { get; set; } = [];
}

sealed class CliStep
{
    public List<string> Argv { get; set; } = [];
}
