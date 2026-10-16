using System.Diagnostics;
using Microsoft.Data.Sqlite;

namespace UserRolesSimpleCli.Tests;

public sealed class MigrationsE2ETests
{
    [Fact]
    [Trait("Category", "e2e")]
    public void DropsExtraFieldsAndTablesAcrossThreeVersions()
    {
        var repo = RepoRoot();
        var tmp = Path.Combine(Path.GetTempPath(), $"user-roles-mig-{Guid.NewGuid():N}");
        Directory.CreateDirectory(tmp);
        try
        {
            Require(Exec(
                Which("node"),
                [
                    "--import",
                    "tsx",
                    Path.Combine(repo, "cli", "src", "main.ts"),
                    "migrations",
                    "create",
                    "--input",
                    Path.Combine(repo, "samples", "user-roles-simple-cli-shared", "deterministic"),
                    "--migrations-root",
                    Path.Combine(tmp, "sql"),
                ],
                Path.Combine(repo, "cli")));
            var dir = Path.Combine(tmp, "sql", "sqlite", "migrations");
            var files = Directory.GetFiles(dir)
                .Select(Path.GetFileName)
                .Where(name => name is not null && System.Text.RegularExpressions.Regex.IsMatch(name, @"^\d{4}_up\.sql$"))
                .OrderBy(name => name, StringComparer.Ordinal)
                .Select(name => Path.Combine(dir, name!))
                .ToList();
            Assert.Equal(["0001_up.sql", "0002_up.sql", "0003_up.sql"], files.Select(Path.GetFileName));
            using var connection = new SqliteConnection("Data Source=:memory:");
            connection.Open();
            ExecSql(connection, "PRAGMA foreign_keys = ON");
            ExecSql(connection, File.ReadAllText(files[0]));
            Assert.Contains("sessions", Tables(connection));
            Assert.Contains("audit_logs", Tables(connection));
            Assert.Contains("nickname", Columns(connection, "users"));
            Assert.Contains("description", Columns(connection, "roles"));
            ExecSql(connection, File.ReadAllText(files[1]));
            Assert.DoesNotContain("sessions", Tables(connection));
            Assert.DoesNotContain("nickname", Columns(connection, "users"));
            Assert.Contains("description", Columns(connection, "roles"));
            ExecSql(connection, File.ReadAllText(files[2]));
            Assert.DoesNotContain("audit_logs", Tables(connection));
            Assert.DoesNotContain("description", Columns(connection, "roles"));
            Assert.Contains("users", Tables(connection));
            Assert.Contains("roles", Tables(connection));
            Assert.Contains("groups", Tables(connection));
        }
        finally
        {
            Directory.Delete(tmp, true);
        }
    }

    static void ExecSql(SqliteConnection connection, string sql)
    {
        using var command = connection.CreateCommand();
        command.CommandText = sql;
        command.ExecuteNonQuery();
    }

    static List<string> Tables(SqliteConnection connection)
    {
        using var command = connection.CreateCommand();
        command.CommandText = "SELECT name FROM sqlite_master WHERE type = 'table' ORDER BY name";
        using var reader = command.ExecuteReader();
        var names = new List<string>();
        while (reader.Read()) names.Add(reader.GetString(0));
        return names;
    }

    static List<string> Columns(SqliteConnection connection, string table)
    {
        using var command = connection.CreateCommand();
        command.CommandText = $"PRAGMA table_info(\"{table}\")";
        using var reader = command.ExecuteReader();
        var names = new List<string>();
        while (reader.Read()) names.Add(reader.GetString(1));
        return names;
    }

    static void Require((int Exit, string Stdout, string Stderr) result)
    {
        if (result.Exit != 0)
            throw new InvalidOperationException($"{result.Stderr}\n{result.Stdout}");
    }

    static (int Exit, string Stdout, string Stderr) Exec(string file, IEnumerable<string> args, string cwd)
    {
        var start = new ProcessStartInfo(file)
        {
            WorkingDirectory = cwd,
            UseShellExecute = false,
            RedirectStandardOutput = true,
            RedirectStandardError = true,
        };
        foreach (var arg in args) start.ArgumentList.Add(arg);
        var process = Process.Start(start) ?? throw new InvalidOperationException($"failed {file}");
        var output = process.StandardOutput.ReadToEnd();
        var error = process.StandardError.ReadToEnd();
        process.WaitForExit();
        return (process.ExitCode, output, error);
    }

    static string Which(string name)
    {
        foreach (var dir in (Environment.GetEnvironmentVariable("PATH") ?? "").Split(Path.PathSeparator))
        {
            var candidate = Path.Combine(dir, name);
            if (File.Exists(candidate)) return candidate;
        }
        return name;
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
}
