using System.Text.RegularExpressions;

namespace Datasources.CodeGenerators;

public static class GenerateSql
{
    public static IReadOnlyList<GenerateEntry> Generate(
        IReadOnlyDictionary<string, string> files,
        IReadOnlyDictionary<string, string> settings,
        IReadOnlyList<string> dialects) =>
        Of(files, settings, dialects);

    public static IReadOnlyList<GenerateEntry> Of(
        IReadOnlyDictionary<string, string> files,
        IReadOnlyDictionary<string, string> settings,
        IReadOnlyList<string> dialects) =>
        dialects.SelectMany(dialect => ForDialect(SqlDialect.Require(dialect), files, settings)).ToList();

    public static IReadOnlyList<GenerateEntry> ForDialect(
        string dialect,
        IReadOnlyDictionary<string, string> files,
        IReadOnlyDictionary<string, string> settings)
    {
        var key = SqlDialect.Require(dialect);
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
        var dialect = SqlDialect.Require(language);
        var mapping = new SqlMapping(casing, types);
        var live = SqlSchema.BuildLive(types, mapping);
        var seeds = SeedGenerator.Sections(dialect, live, seedsByTable, mapping);
        var separator = JoinBatches.Of(settings);
        return (
            new("0001_initial_up.sql", Finalize(UpSql(dialect, live, mapping, seeds, separator), casing)),
            new("0001_initial_down.sql", Finalize(DownSql(dialect, live, separator), casing)));
    }

    static IEnumerable<string> FlattenTable(string dialect, SqlTable table, SqlMapping mapping)
    {
        var indexes = table.Indexes.Select(idx =>
            SqlDialect.RenderSql(dialect, TableStatements.CreateIndex(dialect, mapping, table, idx)));
        var pk = SpecTables.PkName(table.AsType(), SqlSchema.OverlayOf(table));
        var trigger = SqlSchema.HasAuditColumns(table)
            ? SqlDialect.RenderSql(dialect, TableStatements.CreateTrigger(dialect, table, mapping, pk ?? "id"))
            : "";
        return
        [
            SqlDialect.RenderSql(dialect, TableStatements.CreateTable(dialect, table, mapping)),
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
        $"-- Rollback initial migration for {dialect}\n\n-- DROP tables (reverse dependency order)\n{JoinBatches.Join(live.Reverse().Select(table => SqlDialect.RenderSql(dialect, TableStatements.DropTable(dialect, table.TableName))), separator)}";

    static string Finalize(string text, PackCasing casing)
    {
        var output = casing.ApplyKeywords(System.Text.RegularExpressions.Regex.Replace(text, "\n{3,}", "\n\n"));
        return output.EndsWith('\n') ? output : $"{output}\n";
    }
}
