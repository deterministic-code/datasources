using System.Collections.Concurrent;
using System.Text.RegularExpressions;

namespace Datasources.CodeGenerators;

public static class GenerateSql
{
    static readonly ConcurrentDictionary<string, IDriver> Drivers = new();

    public static void Register(string dialect, IDriver driver) =>
        Drivers[RequireName(dialect)] = driver.StatementRenderer is null
            ? throw new InvalidOperationException($"\"{dialect}\" is not a SQL dialect.")
            : driver;

    public static IDriver Driver(string dialect) =>
        Drivers.TryGetValue(RequireName(dialect), out var driver)
            ? driver
            : throw new ProviderNotInstalledException(dialect);

    public static string Require(string language)
    {
        Driver(language);
        return language;
    }

    public static string Render(string dialect, IStatement statement)
    {
        var sql = Driver(dialect).StatementRenderer!.Render(statement).TrimEnd();
        return sql == "" ? "" : sql.EndsWith(';') || sql.EndsWith('/') ? sql : $"{sql};";
    }

    static string RequireName(string dialect) =>
        ProviderCatalog.Default.ContainsKey(dialect)
            ? dialect
            : throw new InvalidOperationException($"Unknown SQL dialect \"{dialect}\".");

    public static IReadOnlyList<GenerateEntry> Generate(
        IReadOnlyDictionary<string, string> files,
        IReadOnlyDictionary<string, string> settings,
        IReadOnlyList<string> dialects) =>
        Of(files, settings, dialects);

    public static IReadOnlyList<GenerateEntry> Of(
        IReadOnlyDictionary<string, string> files,
        IReadOnlyDictionary<string, string> settings,
        IReadOnlyList<string> dialects) =>
        dialects.SelectMany(dialect => ForDialect(Require(dialect), files, settings)).ToList();

    public static IReadOnlyList<GenerateEntry> ForDialect(
        string dialect,
        IReadOnlyDictionary<string, string> files,
        IReadOnlyDictionary<string, string> settings)
    {
        var key = Require(dialect);
        var scoped = DatasourceYaml.For(files, key);
        var casing = PackCasing.Of(settings);
        var schema = SchemaFiles(key, scoped, settings, casing);
        settings.TryGetValue("paths.deterministic", out var dir);
        return
        [
            .. schema.Entries,
            .. CustomMigrations.Of(files, key, dir),
            .. GenerateProcedures.ForDialect(dialect, scoped, settings, schema.Count + 1),
        ];
    }

    static (IReadOnlyList<GenerateEntry> Entries, int Count) SchemaFiles(
        string key,
        IReadOnlyDictionary<string, string> files,
        IReadOnlyDictionary<string, string> settings,
        PackCasing casing)
    {
        if (VersionMigrations.DiffPaths(files).Count > 0)
        {
            var versions = VersionMigrations.Files(key, files, settings, casing)
                .Select(entry => new GenerateEntry($"{key}/migrations/{entry.Filename}", entry.Content))
                .ToList();
            var count = versions
                .Select(entry => Regex.Match(entry.Filename, @"/(\d{4})_"))
                .Where(match => match.Success)
                .Select(match => int.Parse(match.Groups[1].Value))
                .DefaultIfEmpty(0)
                .Max();
            return (versions, count);
        }
        var spec = SpecParser.Parse(files);
        var types = SqlSchema.From(spec);
        var initial = Initial(key, types, spec.DatasourceSeeds, casing, settings);
        return (
            [
                new($"{key}/migrations/{initial.Up.Filename}", initial.Up.Content),
                new($"{key}/migrations/{initial.Down.Filename}", initial.Down.Content),
            ],
            1);
    }

    static (GenerateEntry Up, GenerateEntry Down) Initial(
        string language,
        IReadOnlyList<SqlTable> types,
        IReadOnlyDictionary<string, IReadOnlyList<SeedRow>> seedsByTable,
        PackCasing casing,
        IReadOnlyDictionary<string, string> settings)
    {
        var dialect = Require(language);
        var mapping = new SqlMapping(casing, types);
        var live = SqlSchema.BuildLive(types, mapping);
        var seeds = SeedGenerator.Sections(dialect, live, seedsByTable, mapping);
        var separator = JoinBatches.Of(settings);
        return (
            new("0001_initial_up.sql", Finalize(UpSql(dialect, live, mapping, seeds, separator), casing, dialect)),
            new("0001_initial_down.sql", Finalize(DownSql(dialect, live, separator), casing, dialect)));
    }

    static IEnumerable<string> FlattenTable(string dialect, SqlTable table, SqlMapping mapping)
    {
        var indexes = table.Indexes.Select(idx =>
            Render(dialect, TableStatements.CreateIndex(mapping, table, idx)));
        var pk = SpecTables.PkName(table.AsType(), SqlSchema.OverlayOf(table));
        var trigger = SqlSchema.HasAuditColumns(table)
            ? Render(dialect, TableStatements.CreateTrigger(table, mapping, pk ?? "id"))
            : "";
        return
        [
            Render(dialect, TableStatements.CreateTable(table, mapping)),
            .. indexes,
            trigger,
        ];
    }

    static string UpSql(
        string dialect,
        IReadOnlyList<SqlTable> live,
        SqlMapping mapping,
        IReadOnlyList<string> seeds,
        string separator)
    {
        var parts = live.SelectMany(table => FlattenTable(dialect, table, mapping)).Concat(
            seeds.Count == 0 ? [] : [$"-- Seed data\n{JoinBatches.Join(seeds, separator)}"]);
        return $"-- Generated initial migration for {dialect}\n\n-- Create added tables\n{JoinBatches.Join(parts, separator)}";
    }

    static string DownSql(string dialect, IReadOnlyList<SqlTable> live, string separator) =>
        $"-- Rollback initial migration for {dialect}\n\n-- DROP tables (reverse dependency order)\n{JoinBatches.Join(live.Reverse().Select(table => Render(dialect, TableStatements.DropTable(table.TableName))), separator)}";

    static string Finalize(string text, PackCasing casing, string dialect)
    {
        var output = casing.ApplyKeywords(System.Text.RegularExpressions.Regex.Replace(text, "\n{3,}", "\n\n"), dialect);
        return output.EndsWith('\n') ? output : $"{output}\n";
    }
}
