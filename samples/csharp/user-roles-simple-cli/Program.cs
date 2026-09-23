using Datasources;
using Datasources.Mysql;
using Datasources.Postgres;
using Datasources.Sqlite;
using UserRolesSimpleCli;

return await Run(args);

static async Task<int> Run(string[] args)
{
    try
    {
        var parsed = Parse.Args(args);
        var url = Env("DATASOURCE_URL") ?? throw new InvalidOperationException("DATASOURCE_URL is required");
        var schema = Env("DATABASE_SCHEMA");
        var connection = (ProviderFromUrl.From(url) ?? throw new InvalidOperationException("unknown provider")) switch
        {
            "sqlite" => new SqliteDriver().Connection(new Dictionary<string, string?> { ["url"] = url }),
            "mysql" => new MysqlDriver().Connection(new Dictionary<string, string?> { ["url"] = url }),
            "postgres" => new PostgresDriver().Connection(new Dictionary<string, string?> { ["url"] = url, ["schema"] = schema }),
            var provider => throw new InvalidOperationException($"unknown provider {provider}")
        };
        await connection.Open();
        var command = connection.CreateCommand();
        await Migrate.Apply(command);
        command.AddMiddleware(new ConsoleCommandMiddleware(Console.Error.WriteLine));
        Console.Write(await new App(command, Env("CLI_NOW") is { } stamp
            ? () => DateTime.Parse(stamp, null, System.Globalization.DateTimeStyles.RoundtripKind)
            : null).Run(parsed));
        return 0;
    }
    catch (Exception ex)
    {
        Console.Error.WriteLine(ex.Message);
        return 1;
    }
}

static string? Env(string key)
{
    var value = Environment.GetEnvironmentVariable(key);
    if (!string.IsNullOrWhiteSpace(value)) return value.Trim();
    if (!File.Exists(".env")) return null;
    foreach (var line in File.ReadAllLines(".env"))
    {
        var i = line.IndexOf('=');
        if (i > 0 && line[..i].Trim() == key)
        {
            var raw = line[(i + 1)..].Trim();
            return string.IsNullOrWhiteSpace(raw) ? null : raw;
        }
    }
    return null;
}
