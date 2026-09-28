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
        var name = ProviderFromUrl.From(url) ?? throw new InvalidOperationException("unknown provider");
        IDriver driver = name switch
        {
            "sqlite" => new SqliteDriver(),
            "mysql" => new MysqlDriver(),
            "postgres" => new PostgresDriver(),
            _ => throw new InvalidOperationException($"unknown provider {name}")
        };
        var options = new Dictionary<string, string?> { ["url"] = url };
        if (name == "postgres")
            options["schema"] = schema;
        var connection = driver.Connection(options);
        await connection.Open();
        var command = connection.CreateCommand();
        await Migrate.Apply(command, driver);
        var stamp = Env("CLI_NOW");
        command.AddMiddleware(new ConsoleCommandMiddleware(
            Console.Error.WriteLine,
            stamp is null ? null : () => DateTimeOffset.Parse(stamp, null, System.Globalization.DateTimeStyles.RoundtripKind)));
        Console.Write(await new App(command, stamp is null
            ? null
            : () => DateTime.Parse(stamp, null, System.Globalization.DateTimeStyles.RoundtripKind)).Run(parsed));
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
