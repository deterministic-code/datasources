using System.Diagnostics;
using Microsoft.Data.Sqlite;

namespace ContactsCrudEnrichedMediumRest.Tests;

public sealed class MigrationsE2ETests
{
    [Fact]
    [Trait("Category", "e2e")]
    public void DropsExtraFieldsAndTablesAcrossThreeVersions()
    {
        var repo = RepoRoot();
        var tmp = Path.Combine(Path.GetTempPath(), $"contacts-mig-{Guid.NewGuid():N}");
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
                    Path.Combine(repo, "samples", "contacts-crud-enriched-medium-rest-shared", "deterministic"),
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
            ExecSql(connection, PatchSeeds(File.ReadAllText(files[0])));
            Assert.Contains("tags", Tables(connection));
            Assert.Contains("note_pads", Tables(connection));
            Assert.Contains("middle_name", Columns(connection, "contacts"));
            Assert.Contains("line3", Columns(connection, "addresses"));
            ExecSql(connection, File.ReadAllText(files[1]));
            Assert.DoesNotContain("tags", Tables(connection));
            Assert.DoesNotContain("middle_name", Columns(connection, "contacts"));
            Assert.Contains("line3", Columns(connection, "addresses"));
            ExecSql(connection, File.ReadAllText(files[2]));
            Assert.DoesNotContain("note_pads", Tables(connection));
            Assert.DoesNotContain("line3", Columns(connection, "addresses"));
            Assert.Contains("contacts", Tables(connection));
        }
        finally
        {
            Directory.Delete(tmp, true);
        }
    }

    static string PatchSeeds(string sql) =>
        System.Text.RegularExpressions.Regex.Replace(
            sql,
            @"INSERT INTO (""[^""]+"") \(([^)]+)\) VALUES \((.+)\);",
            match => match.Groups[2].Value.Contains("version", StringComparison.Ordinal)
                ? match.Value
                : $"INSERT INTO {match.Groups[1].Value} ({match.Groups[2].Value}, \"version\") VALUES ({match.Groups[3].Value}, X'00');");

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
            if (File.Exists(Path.Combine(dir.FullName, "ContactsCrudEnrichedMediumRest.Tests.csproj")))
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
