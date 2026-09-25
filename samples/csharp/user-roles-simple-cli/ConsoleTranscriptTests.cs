using System.Diagnostics;
using Datasources;
using Datasources.Mysql;
using Datasources.Postgres;
using Datasources.Sqlite;
using MySqlConnector;
using Npgsql;
using YamlDotNet.Serialization;
using YamlDotNet.Serialization.NamingConventions;

namespace UserRolesSimpleCli.Tests;

[Collection("e2e")]
public sealed class ConsoleTranscriptTests
{
    [Fact]
    public Task Sqlite() => Match("sqlite", () => Connection("sqlite"));

    [Fact]
    [Trait("Category", "e2e")]
    public async Task Mysql()
    {
        FreshCompose("mysql");
        var url = Environment.GetEnvironmentVariable("MYSQL_URL") ?? "mysql://ds:ds@127.0.0.1:3306/datasources";
        var name = $"ds_{Guid.NewGuid():N}"[..18];
        try
        {
            await using (var admin = new MySqlConnection(ToMysql(url)))
            {
                await admin.OpenAsync();
                await using var create = admin.CreateCommand();
                create.CommandText = $"CREATE DATABASE `{name}`";
                await create.ExecuteNonQueryAsync();
            }
            await Match("mysql", () => new MysqlDriver().Connection(new Dictionary<string, string?> { ["url"] = ReplaceMysqlDb(url, name) }));
        }
        finally
        {
            try
            {
                await using var admin = new MySqlConnection(ToMysql(url));
                await admin.OpenAsync();
                await using var drop = admin.CreateCommand();
                drop.CommandText = $"DROP DATABASE `{name}`";
                await drop.ExecuteNonQueryAsync();
            }
            finally
            {
                StopCompose("mysql");
            }
        }
    }

    [Fact]
    [Trait("Category", "e2e")]
    public async Task Postgres()
    {
        FreshCompose("postgres");
        var url = Environment.GetEnvironmentVariable("POSTGRES_URL") ?? "postgres://ds:ds@127.0.0.1:5432/datasources";
        var schema = $"ds_{Guid.NewGuid():N}"[..18];
        try
        {
            await using (var admin = new NpgsqlConnection(ToPostgres(url)))
            {
                await admin.OpenAsync();
                await using var create = admin.CreateCommand();
                create.CommandText = $"CREATE SCHEMA {schema}";
                await create.ExecuteNonQueryAsync();
            }
            await Match("postgres", () => new PostgresDriver().Connection(new Dictionary<string, string?> { ["url"] = url, ["schema"] = schema }));
        }
        finally
        {
            try
            {
                await using var admin = new NpgsqlConnection(ToPostgres(url));
                await admin.OpenAsync();
                await using var drop = admin.CreateCommand();
                drop.CommandText = $"DROP SCHEMA {schema} CASCADE";
                await drop.ExecuteNonQueryAsync();
            }
            finally
            {
                StopCompose("postgres");
            }
        }
    }

    static async Task Match(string provider, Func<IConnection> connect)
    {
        var path = Path.Combine(Shared(), $"{provider}.log");
        if (Environment.GetEnvironmentVariable("RECORD_CONSOLE") == "1")
        {
            var lines = new List<string>();
            await Run(connect(), line => lines.Add(ExpectConsoleMiddleware.Signature(line)));
            await File.WriteAllLinesAsync(path, lines);
            return;
        }
        var expected = (await File.ReadAllLinesAsync(path)).Where(line => line.Length > 0).ToArray();
        var specNow = DateTimeOffset.Parse(Spec().Now, null, System.Globalization.DateTimeStyles.RoundtripKind);
        var middleware = new ExpectConsoleMiddleware(expected, () => specNow);
        await Run(connect(), middleware);
        middleware.Complete();
    }

    static async Task Run(IConnection connection, ICommandMiddleware middleware)
    {
        var spec = Spec();
        await connection.Open();
        var command = connection.CreateCommand();
        await Migrate.Apply(command);
        var now = DateTimeOffset.Parse(spec.Now, null, System.Globalization.DateTimeStyles.RoundtripKind);
        command.AddMiddleware(middleware);
        var app = new App(command, () => now.UtcDateTime, _ => "hash", () => [1, 2, 3]);
        foreach (var step in spec.Steps)
            await app.Run(Parse.Args([.. step.Argv]));
    }

    static async Task Run(IConnection connection, Action<string> write) =>
        await Run(connection, new ConsoleCommandMiddleware(write, () => DateTimeOffset.Parse(Spec().Now, null, System.Globalization.DateTimeStyles.RoundtripKind), new Dictionary<string, string> { ["isVerbose"] = "true" }));

    static CliSpec Spec() =>
        new DeserializerBuilder()
            .WithNamingConvention(CamelCaseNamingConvention.Instance)
            .IgnoreUnmatchedProperties()
            .Build()
            .Deserialize<CliSpec>(File.ReadAllText(Path.Combine(Shared(), "cli.yaml")));

    static IConnection Connection(string provider) => provider switch
    {
        "sqlite" => new SqliteDriver().Connection(new Dictionary<string, string?> { ["filename"] = ":memory:" }),
        "mysql" => new MysqlDriver().Connection(new Dictionary<string, string?> { ["url"] = Environment.GetEnvironmentVariable("MYSQL_URL") ?? "mysql://ds:ds@127.0.0.1:3306/datasources" }),
        "postgres" => new PostgresDriver().Connection(new Dictionary<string, string?>
        {
            ["url"] = Environment.GetEnvironmentVariable("POSTGRES_URL") ?? "postgres://ds:ds@127.0.0.1:5432/datasources",
            ["schema"] = Environment.GetEnvironmentVariable("DATABASE_SCHEMA")
        }),
        _ => throw new InvalidOperationException(provider)
    };

    static void FreshCompose(string service)
    {
        if (!string.IsNullOrEmpty(Environment.GetEnvironmentVariable("SKIP_DOCKER"))) return;
        Exec("docker", $"compose rm -sf -v {service}");
        Exec("docker", $"rm -f datasources-{service}-1");
        Require(Exec("docker", $"compose up -d --force-recreate --wait {service}"));
        if (service == "mysql")
            Require(Exec("docker", "compose exec -T mysql mysql -uroot -proot -e \"GRANT ALL PRIVILEGES ON *.* TO 'ds'@'%'; FLUSH PRIVILEGES;\""));
    }

    static void StopCompose(string service)
    {
        if (!string.IsNullOrEmpty(Environment.GetEnvironmentVariable("SKIP_DOCKER"))) return;
        Exec("docker", $"compose rm -sf -v {service}");
        Exec("docker", $"rm -f datasources-{service}-1");
    }

    static void Require((int Exit, string Stdout, string Stderr) result)
    {
        if (result.Exit != 0)
            throw new InvalidOperationException($"{result.Stderr}\n{result.Stdout}");
    }

    static (int Exit, string Stdout, string Stderr) Exec(string file, string arguments)
    {
        var start = new ProcessStartInfo(file, arguments)
        {
            WorkingDirectory = RepoRoot(),
            RedirectStandardOutput = true,
            RedirectStandardError = true
        };
        var process = Process.Start(start) ?? throw new InvalidOperationException($"failed {file}");
        var output = process.StandardOutput.ReadToEnd();
        var error = process.StandardError.ReadToEnd();
        process.WaitForExit();
        return (process.ExitCode, output, error);
    }

    static string RepoRoot()
    {
        for (var dir = new DirectoryInfo(Shared()); dir is not null; dir = dir.Parent)
            if (File.Exists(Path.Combine(dir.FullName, "package.json")) &&
                File.ReadAllText(Path.Combine(dir.FullName, "package.json")).Contains("\"setup\""))
                return dir.FullName;
        throw new InvalidOperationException("repo root not found");
    }

    static string ToPostgres(string url)
    {
        var uri = new Uri(url);
        var user = uri.UserInfo.Split(':');
        var port = uri.Port > 0 ? uri.Port : 5432;
        var database = uri.AbsolutePath.Trim('/');
        var password = user.Length > 1 ? Uri.UnescapeDataString(user[1]) : "";
        return $"Host={uri.Host};Port={port};Username={Uri.UnescapeDataString(user[0])};Password={password};Database={database}";
    }

    static string ToMysql(string url)
    {
        var uri = new Uri(url);
        var user = uri.UserInfo.Split(':');
        var port = uri.Port > 0 ? uri.Port : 3306;
        var database = uri.AbsolutePath.Trim('/');
        var password = user.Length > 1 ? Uri.UnescapeDataString(user[1]) : "";
        return $"Server={uri.Host};Port={port};User ID={Uri.UnescapeDataString(user[0])};Password={password};Database={database}";
    }

    static string ReplaceMysqlDb(string url, string name) =>
        new UriBuilder(new Uri(url)) { Path = name }.Uri.ToString();

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
}
