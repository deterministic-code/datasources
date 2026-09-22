using System.Diagnostics;
using MySqlConnector;
using Npgsql;
using YamlDotNet.Serialization;
using YamlDotNet.Serialization.NamingConventions;

namespace UserRolesSimpleCli.Tests;

[Collection("e2e")]
public sealed class CliE2ETests(ITestOutputHelper output)
{
    [Fact]
    [Trait("Category", "e2e")]
    public async Task Sqlite() =>
        await Run("sqlite", $"sqlite:///{Path.Combine(Path.GetTempPath(), $"user-roles-{Guid.NewGuid():N}.db")}", null);

    [Fact]
    [Trait("Category", "e2e")]
    public async Task Mysql()
    {
        EnsureCompose("mysql");
        var url = Environment.GetEnvironmentVariable("MYSQL_URL");
        if (string.IsNullOrEmpty(url)) url = "mysql://ds:ds@127.0.0.1:3306/datasources";
        var name = $"ds_{Guid.NewGuid():N}"[..18];
        await using (var admin = new MySqlConnection(ToMysql(url)))
        {
            await admin.OpenAsync();
            await using var create = admin.CreateCommand();
            create.CommandText = $"CREATE DATABASE `{name}`";
            await create.ExecuteNonQueryAsync();
        }
        try
        {
            await Run("mysql", ReplaceMysqlDb(url, name), null);
        }
        finally
        {
            await using var admin = new MySqlConnection(ToMysql(url));
            await admin.OpenAsync();
            await using var drop = admin.CreateCommand();
            drop.CommandText = $"DROP DATABASE `{name}`";
            await drop.ExecuteNonQueryAsync();
        }
    }

    [Fact]
    [Trait("Category", "e2e")]
    public async Task Postgres()
    {
        EnsureCompose("postgres");
        var url = Environment.GetEnvironmentVariable("POSTGRES_URL");
        if (string.IsNullOrEmpty(url)) url = "postgres://ds:ds@127.0.0.1:5432/datasources";
        var schema = $"ds_{Guid.NewGuid():N}"[..18];
        await using (var admin = new NpgsqlConnection(ToPostgres(url)))
        {
            await admin.OpenAsync();
            await using var create = admin.CreateCommand();
            create.CommandText = $"CREATE SCHEMA {schema}";
            await create.ExecuteNonQueryAsync();
        }
        try
        {
            await Run("postgres", url, schema);
        }
        finally
        {
            await using var admin = new NpgsqlConnection(ToPostgres(url));
            await admin.OpenAsync();
            await using var drop = admin.CreateCommand();
            drop.CommandText = $"DROP SCHEMA {schema} CASCADE";
            await drop.ExecuteNonQueryAsync();
        }
    }

    async Task Run(string provider, string url, string? schema)
    {
        var sample = SampleDir();
        var spec = LoadSpec();
        await File.WriteAllTextAsync(Path.Combine(sample, ".env"), $"""
            INSTALLED_PROVIDERS=sqlite,mysql,postgres
            DATASOURCE_URL={url}
            DATABASE_SCHEMA={schema}
            """);
        var env = new Dictionary<string, string>
        {
            ["DATASOURCE_URL"] = url,
            ["DATABASE_SCHEMA"] = schema ?? "",
            ["CLI_NOW"] = spec.Now
        };
        Require(Exec("npm", $"run setup -- --cwd {sample} --ecosystem csharp --yes", RepoRoot()));
        foreach (var step in spec.Steps)
        {
            var got = Cli(sample, env, [.. step.Argv]);
            var cmd = string.Join(" ", step.Argv);
            if (Verbose())
            {
                Log($"{provider}: {cmd}");
                Log($"  assert exit {got.Exit} == {step.Expect.Exit}");
                Log($"  assert stdout [{got.Stdout}] == [{step.Expect.Stdout}]");
                Log($"  assert stderr [{got.Stderr}] == [{step.Expect.Stderr}]");
            }
            if (got.Exit != step.Expect.Exit || got.Stdout != step.Expect.Stdout || got.Stderr != step.Expect.Stderr)
                Assert.Fail($"{provider}: {cmd}: first mismatch\nexpected exit {step.Expect.Exit} stdout [{step.Expect.Stdout}] stderr [{step.Expect.Stderr}]\nactual exit {got.Exit} stdout [{got.Stdout}] stderr [{got.Stderr}]");
        }
    }

    static CliSpec LoadSpec() =>
        new DeserializerBuilder()
            .WithNamingConvention(CamelCaseNamingConvention.Instance)
            .IgnoreUnmatchedProperties()
            .Build()
            .Deserialize<CliSpec>(File.ReadAllText(Path.GetFullPath(Path.Combine(SampleDir(), "..", "..", "user-roles-simple-cli-shared", "cli.yaml"))))
        ?? throw new InvalidOperationException("spec");

    static (int Exit, string Stdout, string Stderr) Cli(string sample, IReadOnlyDictionary<string, string> env, string[] args)
    {
        var dll = Path.Combine(sample, "bin/Debug/net10.0/UserRolesSimpleCli.dll");
        return Exec("dotnet", string.Join(" ", args.Prepend(Quote(dll)).Select(Quote)), sample, env);
    }

    void Log(string line)
    {
        output.WriteLine(line);
        Console.WriteLine(line);
    }

    static bool Verbose() =>
        Environment.GetEnvironmentVariable("VERBOSE") is "1" or "true";

    static void EnsureCompose(string service)
    {
        if (!string.IsNullOrEmpty(Environment.GetEnvironmentVariable("SKIP_DOCKER"))) return;
        Require(Exec("docker", $"compose up -d --wait {service}", RepoRoot()));
    }

    static string Quote(string value) =>
        value.Contains(' ') ? $"\"{value}\"" : value;

    static void Require((int Exit, string Stdout, string Stderr) result)
    {
        if (result.Exit != 0)
            throw new InvalidOperationException($"{result.Stderr}\n{result.Stdout}");
    }

    static (int Exit, string Stdout, string Stderr) Exec(string file, string arguments, string cwd, IReadOnlyDictionary<string, string>? env = null)
    {
        var start = new ProcessStartInfo(file, arguments)
        {
            WorkingDirectory = cwd,
            RedirectStandardOutput = true,
            RedirectStandardError = true
        };
        if (env is not null)
            foreach (var pair in env)
                start.Environment[pair.Key] = pair.Value;
        var process = Process.Start(start) ?? throw new InvalidOperationException($"failed {file}");
        var output = process.StandardOutput.ReadToEnd();
        var error = process.StandardError.ReadToEnd();
        process.WaitForExit();
        return (process.ExitCode, output, error);
    }

    static string SampleDir()
    {
        for (var dir = new DirectoryInfo(AppContext.BaseDirectory); dir is not null; dir = dir.Parent)
            if (File.Exists(Path.Combine(dir.FullName, "UserRolesSimpleCli.csproj")))
                return dir.FullName;
        throw new InvalidOperationException("sample not found");
    }

    static string RepoRoot()
    {
        for (var dir = new DirectoryInfo(SampleDir()); dir is not null; dir = dir.Parent)
            if (File.Exists(Path.Combine(dir.FullName, "package.json")) &&
                File.ReadAllText(Path.Combine(dir.FullName, "package.json")).Contains("\"setup\""))
                return dir.FullName;
        throw new InvalidOperationException("repo root not found");
    }

    static string ToPostgres(string url)
    {
        if (!url.StartsWith("postgres://", StringComparison.OrdinalIgnoreCase) &&
            !url.StartsWith("postgresql://", StringComparison.OrdinalIgnoreCase))
            return url;
        var uri = new Uri(url);
        var user = uri.UserInfo.Split(':');
        var port = uri.Port > 0 ? uri.Port : 5432;
        var database = uri.AbsolutePath.Trim('/');
        var password = user.Length > 1 ? Uri.UnescapeDataString(user[1]) : "";
        return $"Host={uri.Host};Port={port};Username={Uri.UnescapeDataString(user[0])};Password={password};Database={database}";
    }

    static string ToMysql(string url)
    {
        if (!url.StartsWith("mysql://", StringComparison.OrdinalIgnoreCase)) return url;
        var uri = new Uri(url);
        var user = uri.UserInfo.Split(':');
        var port = uri.Port > 0 ? uri.Port : 3306;
        var database = uri.AbsolutePath.Trim('/');
        var password = user.Length > 1 ? Uri.UnescapeDataString(user[1]) : "";
        return $"Server={uri.Host};Port={port};User ID={Uri.UnescapeDataString(user[0])};Password={password};Database={database}";
    }

    static string ReplaceMysqlDb(string url, string name)
    {
        var uri = new Uri(url);
        return new UriBuilder(uri) { Path = name }.Uri.ToString();
    }

    sealed class CliSpec
    {
        public string Now { get; set; } = "";
        public List<CliStep> Steps { get; set; } = [];
    }

    sealed class CliStep
    {
        public List<string> Argv { get; set; } = [];
        public CliExpect Expect { get; set; } = new();
    }

    sealed class CliExpect
    {
        public int Exit { get; set; }
        public string Stdout { get; set; } = "";
        public string Stderr { get; set; } = "";
    }
}
