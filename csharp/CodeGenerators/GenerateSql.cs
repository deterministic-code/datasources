using System.Text;
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
        var casing = PackCasing.Of(settings);
        var schema = SchemaFiles(key, files, settings, casing);
        settings.TryGetValue("paths.deterministic", out var dir);
        return
        [
            .. schema.Entries,
            .. CustomMigrations.Of(files, key, dir),
            .. GenerateProcedures.ForDialect(dialect, files, settings, schema.Count + 1),
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
        var initial = Initial(key, types, spec.DatasourceSeeds, casing);
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
        PackCasing casing)
    {
        var dialect = SqlDialect.Require(language);
        var mapping = new SqlMapping(casing, types);
        var live = SqlSchema.BuildLive(types, mapping);
        var seeds = SeedGenerator.Sections(dialect, live, seedsByTable, mapping);
        return (
            new("0001_initial_up.sql", Finalize(UpSql(dialect, live, mapping, seeds), casing)),
            new("0001_initial_down.sql", Finalize(DownSql(dialect, live), casing)));
    }

    static string UpSql(string dialect, IReadOnlyList<SqlTable> live, SqlMapping mapping, IReadOnlyList<string> seeds)
    {
        var body = new StringBuilder();
        body.Append($"-- Generated initial migration for {dialect}\n\n");
        body.Append("-- Create added tables\n");
        foreach (var table in live)
        {
            var indexes = table.Indexes
                .Select(idx => SqlDialect.RenderSql(dialect, TableStatements.CreateIndex(dialect, mapping, table, idx)))
                .ToList();
            var pk = SpecTables.PkName(table.AsType(), SqlSchema.OverlayOf(table));
            var trigger = SqlSchema.HasAuditColumns(table)
                ? SqlDialect.RenderSql(dialect, TableStatements.CreateTrigger(dialect, table, mapping, pk ?? "id"))
                : "";
            body.Append(SqlDialect.RenderSql(dialect, TableStatements.CreateTable(dialect, table, mapping)));
            body.Append('\n');
            if (indexes.Count > 0 && indexes.Any(sql => sql != ""))
            {
                body.Append(string.Join("\n", indexes.Where(sql => sql != "")));
                body.Append('\n');
            }
            if (trigger != "")
            {
                body.Append(trigger);
                body.Append('\n');
            }
            body.Append('\n');
        }
        if (seeds.Count > 0)
        {
            body.Append("-- Seed data\n");
            foreach (var block in seeds) body.Append(block).Append(block.EndsWith('\n') ? "" : "\n");
        }
        return body.ToString();
    }

    static string DownSql(string dialect, IReadOnlyList<SqlTable> live)
    {
        var body = new StringBuilder();
        body.Append($"-- Rollback initial migration for {dialect}\n\n");
        body.Append("-- DROP tables (reverse dependency order)\n");
        foreach (var table in live.Reverse())
        {
            body.Append(SqlDialect.RenderSql(dialect, TableStatements.DropTable(dialect, table.TableName)));
            body.Append('\n');
        }
        return body.ToString();
    }

    static string Finalize(string text, PackCasing casing)
    {
        var output = casing.ApplyKeywords(System.Text.RegularExpressions.Regex.Replace(text, "\n{3,}", "\n\n"));
        return output.EndsWith('\n') ? output : $"{output}\n";
    }
}
