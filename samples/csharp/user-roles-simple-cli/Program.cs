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
        var provider = Env("PROVIDER") ?? "sqlite";
        var url = Env("DATASOURCE_URL") ?? "sqlite:///./app.sqlite";
        Migraters.Up(provider, url, Env("MIGRATIONS_ROOT") ?? "sql");
        DatasourceFactory.Register("sqlite", new SqliteDriver());
        DatasourceFactory.Register("mysql", new MysqlDriver());
        DatasourceFactory.Register("postgres", new PostgresDriver());
        await using var ds = Datasource.Default;
        var connection = await ds.CreateConnection();
        await connection.Open();
        var command = await ds.CreateCommand(connection);
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
