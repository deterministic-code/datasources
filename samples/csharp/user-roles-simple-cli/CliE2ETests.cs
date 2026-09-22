using System.Diagnostics;
using System.Text.Json;
using MySqlConnector;
using Npgsql;

namespace UserRolesSimpleCli.Tests;

[Collection("e2e")]
public sealed class CliE2ETests
{
    [Fact]
    [Trait("Category", "e2e")]
    public async Task Sqlite() =>
        await Run("sqlite", $"sqlite:///{Path.Combine(Path.GetTempPath(), $"user-roles-{Guid.NewGuid():N}.db")}", null);

    [Fact]
    [Trait("Category", "e2e")]
    public async Task Mysql()
    {
        var url = Environment.GetEnvironmentVariable("MYSQL_URL");
        if (string.IsNullOrEmpty(url)) return;
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
        var url = Environment.GetEnvironmentVariable("POSTGRES_URL");
        if (string.IsNullOrEmpty(url)) return;
        var schema = $"ds_{Guid.NewGuid():N}"[..18];
        await using (var admin = new NpgsqlConnection(url))
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
            await using var admin = new NpgsqlConnection(url);
            await admin.OpenAsync();
            await using var drop = admin.CreateCommand();
            drop.CommandText = $"DROP SCHEMA {schema} CASCADE";
            await drop.ExecuteNonQueryAsync();
        }
    }

    static async Task Run(string provider, string url, string? schema)
    {
        var sample = SampleDir();
        await File.WriteAllTextAsync(Path.Combine(sample, ".env"), $"""
            INSTALLED_PROVIDERS=sqlite,mysql,postgres
            DATASOURCE_URL={url}
            DATABASE_SCHEMA={schema}
            """);
        var env = new Dictionary<string, string>
        {
            ["DATASOURCE_URL"] = url,
            ["DATABASE_SCHEMA"] = schema ?? ""
        };
        Exec("npm", $"run setup -- --cwd {sample} --ecosystem csharp --yes", RepoRoot());
        Exec("dotnet", "build -v q --nologo", sample);
        Cli(sample, env, "add-role", "admin");
        Cli(sample, env, "add-group", "staff");
        Cli(sample, env, "add-user", "alice", "secret");
        Cli(sample, env, "add-user-role", "alice", "admin");
        Cli(sample, env, "add-user-group", "alice", "staff");
        var listed = Cli(sample, env, "list-users-roles-groups");
        using var doc = JsonDocument.Parse(listed);
        var user = doc.RootElement[0];
        Assert.Equal("alice", user.GetProperty("username").GetString());
        Assert.True(user.GetProperty("is_enabled").GetBoolean());
        Assert.Equal("admin", user.GetProperty("roles")[0].GetProperty("name").GetString());
        Assert.Equal("staff", user.GetProperty("groups")[0].GetProperty("name").GetString());
        Assert.False(listed.Contains("password", StringComparison.Ordinal));
        Assert.False(listed.Contains("ugly_name", StringComparison.Ordinal));
        Cli(sample, env, "disable-user", "alice");
        var disabled = Cli(sample, env, "list-users-roles-groups");
        Assert.False(JsonDocument.Parse(disabled).RootElement[0].GetProperty("is_enabled").GetBoolean());
    }

    static string Cli(string sample, IReadOnlyDictionary<string, string> env, params string[] args)
    {
        var dll = Path.Combine(sample, "bin/Debug/net10.0/UserRolesSimpleCli.dll");
        return Exec("dotnet", string.Join(" ", args.Prepend(Quote(dll)).Select(Quote)), sample, env);
    }

    static string Quote(string value) =>
        value.Contains(' ') ? $"\"{value}\"" : value;

    static string Exec(string file, string arguments, string cwd, IReadOnlyDictionary<string, string>? env = null)
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
        return process.ExitCode == 0 ? output : throw new InvalidOperationException($"{file} {arguments}\n{error}\n{output}");
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
}
