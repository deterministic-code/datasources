using System.Diagnostics;

namespace ContactsCrudEnrichedMediumRest.Tests;

public sealed class GenerateE2ETests
{
    [Fact]
    [Trait("Category", "e2e")]
    public void GeneratesAndRegistersContacts()
    {
        var repo = RepoRoot();
        var sample = SampleDir();
        var ts = Path.GetFullPath(Path.Combine(sample, "../../typescript/contacts-crud-enriched-medium-rest"));
        var tmp = Path.Combine(Path.GetTempPath(), $"contacts-gen-cs-{Guid.NewGuid():N}");
        Directory.CreateDirectory(tmp);
        try
        {
            var db = Path.Combine(tmp, "app.sqlite");
            var url = $"sqlite:///{db}";
            var env = new Dictionary<string, string> { ["PROVIDER"] = "sqlite", ["DATASOURCE_URL"] = url };
            var cli = Path.Combine(repo, "cli", "bin", "datasources.mjs");
            var spec = Path.Combine(repo, "samples", "contacts-crud-enriched-medium-rest-shared", "deterministic");
            Require(Exec(
                Which("node"),
                [cli, "generate", "all", "--dialect", "csharp", "--cwd", sample, "--input", spec, "--output", "generated", "--no-migrate"],
                ts,
                env));
            Require(Exec(Which("npx"), ["migrate-setup", "--provider", "sqlite", "--connection", db], ts));
            Require(Exec(
                Which("npx"),
                ["migrate-up", "--provider", "sqlite", "--connection", db, "--migrations-root", Path.Combine(sample, "sql")],
                ts));
            var run = Path.Combine(tmp, "run");
            Directory.CreateDirectory(run);
            File.Copy(Path.Combine(sample, "Program.cs"), Path.Combine(run, "Program.cs"));
            File.WriteAllText(Path.Combine(run, "Run.csproj"), $"""
                <Project Sdk="Microsoft.NET.Sdk">
                  <PropertyGroup>
                    <OutputType>Exe</OutputType>
                    <TargetFramework>net10.0</TargetFramework>
                    <ImplicitUsings>enable</ImplicitUsings>
                    <Nullable>enable</Nullable>
                    <TreatWarningsAsErrors>true</TreatWarningsAsErrors>
                    <NoWarn>CS8618</NoWarn>
                  </PropertyGroup>
                  <ItemGroup>
                    <Compile Include="{Path.Combine(sample, "generated", "**", "*.cs")}" />
                    <ProjectReference Include="{Path.Combine(repo, "csharp", "Datasources", "Datasources.csproj")}" />
                    <ProjectReference Include="{Path.Combine(repo, "csharp", "Providers", "Sqlite", "Datasources.Sqlite.csproj")}" />
                  </ItemGroup>
                </Project>
                """);
            Require(Exec(
                Which("dotnet"),
                ["run", "--project", Path.Combine(run, "Run.csproj")],
                run,
                new Dictionary<string, string> { ["MSBUILDDISABLENODEREUSE"] = "1", ["DATASOURCE_URL"] = url }));
        }
        finally
        {
            Directory.Delete(tmp, true);
        }
    }

    static void Require((int Exit, string Stdout, string Stderr) result)
    {
        if (result.Exit != 0)
            throw new InvalidOperationException($"{result.Stderr}\n{result.Stdout}");
    }

    static (int Exit, string Stdout, string Stderr) Exec(
        string file,
        IEnumerable<string> args,
        string cwd,
        IReadOnlyDictionary<string, string>? env = null)
    {
        var start = new ProcessStartInfo(file)
        {
            WorkingDirectory = cwd,
            UseShellExecute = false,
            RedirectStandardInput = true,
            RedirectStandardOutput = true,
            RedirectStandardError = true,
        };
        foreach (var arg in args)
            start.ArgumentList.Add(arg);
        if (env is not null)
            foreach (var pair in env)
                start.Environment[pair.Key] = pair.Value;
        var process = Process.Start(start) ?? throw new InvalidOperationException($"failed {file}");
        process.StandardInput.Close();
        var output = process.StandardOutput.ReadToEndAsync();
        var error = process.StandardError.ReadToEndAsync();
        Task.WaitAll(output, error);
        process.WaitForExit();
        return (process.ExitCode, output.Result, error.Result);
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
