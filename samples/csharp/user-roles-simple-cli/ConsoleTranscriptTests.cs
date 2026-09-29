using System.Diagnostics;
using System.Text.RegularExpressions;
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
    public Task Sqlite() => Match(() => Connection("sqlite"), false, "sqlite.log");

    [Fact]
    public void SqlLogsMatch()
    {
        var sql = new[] { "sqlite", "mysql", "postgres" }.Select(SqlText).ToArray();
        Assert.Equal(sql[0], sql[1]);
        Assert.Equal(sql[0], sql[2]);
        var full = new[] { "sqlite", "mysql", "postgres" }.Select(DialectFile).ToArray();
        Assert.Equal(full[0], full[1]);
        Assert.Equal(full[0], full[2]);
    }

    [Fact]
    public void NormalizeDropsDialectNoise()
    {
        Assert.Equal(
            "[<date>]-[datasource][Request]-[{password: <token>, updated: <token>, blob: <token>, id: <uuid>}]",
            Normalize("[2026-09-22T00:00:00.000Z]-[datasource][Request]-[{password: hash, updated: AQID, blob: ab12, id: 11111111-1111-1111-1111-111111111111}] 2ms"));
        Assert.Equal(
            "[<date>]-[datasource][Start]-[INSERT INTO users (roles) VALUES ($name)]",
            Normalize("[2026-09-22T00:00:00.000Z]-[datasource][Start]-[INSERT INTO `users` (\"roles\") VALUES ($name)] 3μs"));
        Assert.Equal("count: 1234", Normalize("count: 1234"));
        Assert.Equal("plain", Normalize("plain"));
        Assert.Equal("[x] kept", Normalize("[x] kept"));
        Assert.Equal(
            ["[<date>]-[datasource][Request]-[{is_enabled: 1}]"],
            Dialect(["[2026-09-22T00:00:00.000Z]-[datasource][Request]-[{is_enabled: True}]"]));
        Assert.Equal(
            [
                "[<date>]-[datasource][Start]-[UPDATE users SET is_enabled = $is_enabled]",
                "[<date>]-[datasource][AfterCommand]-[{result: <id>, rowsAffected: 1}]"
            ],
            Dialect([
                "[2026-09-22T00:00:00.000Z]-[datasource][Start]-[UPDATE users SET is_enabled = $is_enabled]",
                "[2026-09-22T00:00:00.000Z]-[datasource][AfterCommand]-[{result: 5, rowsAffected: 1}]"
            ]));
        Assert.Equal(
            "[<date>]-[datasource][AfterCommand]-[{result: 1, rowsAffected: 1}]",
            Dialect([
                "[2026-09-22T00:00:00.000Z]-[datasource][Start]-[INSERT INTO users (name) VALUES ($name)]",
                "[2026-09-22T00:00:00.000Z]-[datasource][AfterCommand]-[{result: 1, rowsAffected: 1}]"
            ])[1]);
    }

    [Fact]
    [Trait("Category", "e2e")]
    public async Task Mysql()
    {
        FreshCompose("mysql");
        var url = Environment.GetEnvironmentVariable("MYSQL_URL") ?? "mysql://ds:ds@127.0.0.1:3306/datasources";
        var name = $"ds_{Guid.NewGuid():N}"[..18];
        var procedures = $"ds_{Guid.NewGuid():N}"[..18];
        try
        {
            await using (var admin = new MySqlConnection(ToMysql(url)))
            {
                await admin.OpenAsync();
                await using var create = admin.CreateCommand();
                create.CommandText = $"CREATE DATABASE `{name}`";
                await create.ExecuteNonQueryAsync();
            }
            await using (var admin = new MySqlConnection(ToMysql(url)))
            {
                await admin.OpenAsync();
                await using var create = admin.CreateCommand();
                create.CommandText = $"CREATE DATABASE `{procedures}`";
                await create.ExecuteNonQueryAsync();
            }
            await Match(() => new MysqlDriver().Connection(new Dictionary<string, string?> { ["DATASOURCE_URL"] = ReplaceMysqlDb(url, name) }), false, "mysql.log");
            await Match(() => new MysqlDriver().Connection(new Dictionary<string, string?> { ["DATASOURCE_URL"] = ReplaceMysqlDb(url, procedures) }), true, "mysql-procedures.log");
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
                await using var dropProcedures = admin.CreateCommand();
                dropProcedures.CommandText = $"DROP DATABASE `{procedures}`";
                await dropProcedures.ExecuteNonQueryAsync();
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
            await Match(() => new PostgresDriver().Connection(new Dictionary<string, string?> { ["DATASOURCE_URL"] = url.Contains('?') ? $"{url}&schema={schema}" : $"{url}?schema={schema}" }), false, "postgres.log");
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

    static async Task Match(Func<IConnection> connect, bool procedures, string file)
    {
        var path = Path.Combine(Shared(), file);
        if (Environment.GetEnvironmentVariable("RECORD_CONSOLE") == "1")
        {
            var lines = new List<string>();
            await Run(connect(), line => lines.Add(ExpectConsoleMiddleware.Signature(line)), procedures);
            await File.WriteAllLinesAsync(path, lines);
            return;
        }
        var expected = (await File.ReadAllLinesAsync(path)).Where(line => line.Length > 0).Select(Normalize).ToArray();
        var specNow = DateTimeOffset.Parse(Spec().Now, null, System.Globalization.DateTimeStyles.RoundtripKind);
        var middleware = new ExpectConsoleMiddleware(expected, () => specNow, Normalize);
        await Run(connect(), middleware, procedures);
        middleware.Complete();
    }

    static async Task Run(IConnection connection, ICommandMiddleware middleware, bool procedures)
    {
        var spec = Spec();
        await connection.Open();
        var command = connection.CreateCommand();
        await Migrate.Apply(command);
        var now = DateTimeOffset.Parse(spec.Now, null, System.Globalization.DateTimeStyles.RoundtripKind);
        command.AddMiddleware(middleware);
        var app = new App(command, () => now.UtcDateTime, _ => "hash", () => [1, 2, 3], procedures);
        foreach (var step in spec.Steps)
            await app.Run(Parse.Args([.. step.Argv]));
    }

    static async Task Run(IConnection connection, Action<string> write, bool procedures) =>
        await Run(connection, new ConsoleCommandMiddleware(write, () => DateTimeOffset.Parse(Spec().Now, null, System.Globalization.DateTimeStyles.RoundtripKind), new Dictionary<string, string> { ["isVerbose"] = "true" }), procedures);

    static string[] SqlText(string provider) =>
        File.ReadAllLines(Path.Combine(Shared(), $"{provider}.log"))
            .Where(line => line.Contains("][Start]-", StringComparison.Ordinal) || line.Contains("][Finish]-", StringComparison.Ordinal))
            .Select(Normalize)
            .ToArray();

    static string[] DialectFile(string provider) =>
        Dialect(File.ReadAllLines(Path.Combine(Shared(), $"{provider}.log")));

    static string[] Dialect(IEnumerable<string> lines)
    {
        var update = false;
        return lines.Where(line => line.Length > 0).Select(line =>
        {
            var text = Normalize(line).Replace(": True", ": 1", StringComparison.Ordinal).Replace(": False", ": 0", StringComparison.Ordinal);
            if (text.Contains("][Start]-", StringComparison.Ordinal))
                update = text.Contains("][Start]-[UPDATE ", StringComparison.Ordinal);
            return update && text.Contains("][AfterCommand]-", StringComparison.Ordinal)
                ? UpdateResult.Replace(text, "result: <id>")
                : text;
        }).ToArray();
    }

    static string Normalize(string line)
    {
        var text = ExpectConsoleMiddleware.Signature(line);
        text = Date.Replace(text, "<date>");
        text = Uuid.Replace(text, "<uuid>");
        text = Ident.Replace(text, "$1");
        text = Password.Replace(text, "password: <token>");
        text = Updated.Replace(text, "updated: <token>");
        return Token.Replace(text, match => match.Value.Any(char.IsDigit) && match.Value.Any(char.IsLetter) ? "<token>" : match.Value);
    }

    static readonly Regex Date = new(@"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z", RegexOptions.Compiled);
    static readonly Regex Uuid = new(@"\b[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}\b", RegexOptions.Compiled);
    static readonly Regex Ident = new(@"[""`]([A-Za-z_][A-Za-z0-9_]*)[""`]", RegexOptions.Compiled);
    static readonly Regex Password = new(@"password: [^,}\]]+", RegexOptions.Compiled);
    static readonly Regex Updated = new(@"updated: [^,}\]]+", RegexOptions.Compiled);
    static readonly Regex Token = new(@"[A-Za-z0-9+/]{4,}={0,2}", RegexOptions.Compiled);
    static readonly Regex UpdateResult = new(@"result: [^,}]+", RegexOptions.Compiled);

    static CliSpec Spec() =>
        new DeserializerBuilder()
            .WithNamingConvention(CamelCaseNamingConvention.Instance)
            .IgnoreUnmatchedProperties()
            .Build()
            .Deserialize<CliSpec>(File.ReadAllText(Path.Combine(Shared(), "cli.yaml")));

    static IConnection Connection(string provider) => provider switch
    {
        "sqlite" => new SqliteDriver().Connection(new Dictionary<string, string?> { ["DATASOURCE_URL"] = "sqlite:///:memory:" }),
        "mysql" => new MysqlDriver().Connection(new Dictionary<string, string?> { ["DATASOURCE_URL"] = Environment.GetEnvironmentVariable("MYSQL_URL") ?? "mysql://ds:ds@127.0.0.1:3306/datasources" }),
        "postgres" => new PostgresDriver().Connection(new Dictionary<string, string?>
        {
            ["DATASOURCE_URL"] = Environment.GetEnvironmentVariable("DATASOURCE_URL")
                ?? Environment.GetEnvironmentVariable("POSTGRES_URL")
                ?? "postgres://ds:ds@127.0.0.1:5432/datasources"
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
