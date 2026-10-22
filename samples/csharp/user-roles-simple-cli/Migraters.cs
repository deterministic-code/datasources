using System.Diagnostics;

namespace UserRolesSimpleCli;

public static class Migraters
{
    public static void Up(string provider, string connection, string migrationsRoot)
    {
        var repo = RepoRoot();
        var result = Exec(
            "node",
            [Path.Combine(repo, "scripts", "migraters.mjs"), "up", "--provider", provider, "--connection", connection, "--migrations-root", migrationsRoot],
            repo);
        if (result.Exit != 0) throw new InvalidOperationException($"{result.Stderr}\n{result.Stdout}");
    }

    public static string Create(string migrationsRoot)
    {
        var repo = RepoRoot();
        Directory.CreateDirectory(migrationsRoot);
        var result = Exec(
            "node",
            [
                "--import", "tsx", Path.Combine(repo, "cli", "src", "main.ts"),
                "migrations", "create",
                "--input", Path.Combine(repo, "samples", "user-roles-simple-cli-shared", "deterministic"),
                "--migrations-root", migrationsRoot,
            ],
            Path.Combine(repo, "cli"));
        if (result.Exit != 0) throw new InvalidOperationException($"{result.Stderr}\n{result.Stdout}");
        return migrationsRoot;
    }

    public static void CreateAndUp(string provider, string connection) =>
        Up(provider, connection, Create(Path.Combine(Path.GetTempPath(), $"user-roles-sql-{Guid.NewGuid():N}")));

    static (int Exit, string Stdout, string Stderr) Exec(string file, IEnumerable<string> args, string cwd)
    {
        var start = new ProcessStartInfo(file) { WorkingDirectory = cwd, RedirectStandardOutput = true, RedirectStandardError = true };
        foreach (var arg in args) start.ArgumentList.Add(arg);
        var process = Process.Start(start) ?? throw new InvalidOperationException(file);
        var output = process.StandardOutput.ReadToEndAsync();
        var error = process.StandardError.ReadToEndAsync();
        process.WaitForExit();
        return (process.ExitCode, output.Result, error.Result);
    }

    static string RepoRoot()
    {
        for (var dir = new DirectoryInfo(AppContext.BaseDirectory); dir is not null; dir = dir.Parent)
            if (File.Exists(Path.Combine(dir.FullName, "package.json")) &&
                File.ReadAllText(Path.Combine(dir.FullName, "package.json")).Contains("\"setup\""))
                return dir.FullName;
        throw new InvalidOperationException("repo");
    }
}
