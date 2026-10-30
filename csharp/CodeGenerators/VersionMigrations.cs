using System.Text.RegularExpressions;
using Deterministic.Generators.Common;

namespace Datasources.CodeGenerators;

public static class VersionMigrations
{
    static readonly Regex VersionDiff = new(@"^version-\d{8}T\d{6}Z-\d{3}/diff\.yaml$", RegexOptions.Compiled);

    const string DefaultDatasourceYaml = """
        version: 1.0.0
        includes:
          - types:
              filter: tag == "datasource_type" || tag == "view_type"
        types: []
        """;

    public static IReadOnlyList<string> DiffPaths(IReadOnlyDictionary<string, string> files) =>
        files.Keys.Where(path => VersionDiff.IsMatch(path)).OrderBy(path => path, StringComparer.Ordinal).ToList();

    public static IReadOnlyList<GenerateEntry> Files(
        string dialect,
        IReadOnlyDictionary<string, string> files,
        IReadOnlyDictionary<string, string> settings,
        PackCasing casing)
    {
        var paths = DiffPaths(files);
        if (paths.Count == 0) return [];

        var diffs = new List<YamlValue?>();
        var snapshots = new List<VersionSnapshot>();
        foreach (var path in paths)
        {
            var text = files[path];
            if (text == "") continue;
            diffs.Add(ParseDiff(text, path));
            snapshots.Add(VersionSnapshot.Combine(diffs));
        }
        if (snapshots.Count == 0) return [];

        var last = snapshots[^1];
        if (!Matches(last, SpecParser.TypesYaml, files.GetValueOrDefault(SpecParser.TypesYaml), null))
            throw new InvalidOperationException("types.yaml or datasource.yaml differs from the frozen version diffs; run datasources version first");
        if (!Has(files, $"datasource.{dialect}.yaml"))
        {
            if (!Has(files, SpecParser.DatasourceYaml))
                throw new InvalidOperationException($"{dialect} requires datasource.{dialect}.yaml or datasource.yaml");
            if (!Matches(last, SpecParser.DatasourceYaml, files.GetValueOrDefault(SpecParser.DatasourceYaml), DefaultDatasourceYaml))
                throw new InvalidOperationException("types.yaml or datasource.yaml differs from the frozen version diffs; run datasources version first");
        }

        var output = new List<GenerateEntry>();
        var previous = Compile(Empty(), settings);
        var number = 0;
        foreach (var snapshot in snapshots)
        {
            var current = Compile(snapshot, settings);
            var up = SchemaDiff.Of(dialect, previous.Tables, previous.Naming, current.Tables, current.Naming)
                .Concat(ProcedureDiff(dialect, settings, previous.Tables, current.Tables, casing)).ToList();
            var down = SchemaDiff.Of(dialect, current.Tables, current.Naming, previous.Tables, previous.Naming)
                .Concat(ProcedureDiff(dialect, settings, current.Tables, previous.Tables, casing)).ToList();
            var added = SeedGenerator.Added(previous.Seeds, current.Seeds);
            var removed = SeedGenerator.Added(current.Seeds, previous.Seeds);
            var seedUp = SeedGenerator.Sections(dialect, current.Tables, added, current.Naming);
            var seedDown = SeedGenerator.Deletes(dialect, previous.Tables, added, previous.Naming)
                .Concat(SeedGenerator.Sections(dialect, previous.Tables, removed, previous.Naming)).ToList();
            if (up.Count == 0 && down.Count == 0 && seedUp.Count == 0 && seedDown.Count == 0)
            {
                previous = current;
                continue;
            }
            number++;
            var separator = JoinBatches.Of(settings);
            var upSql = $"-- Generated migration for {dialect}\n{Render(dialect, up, separator)}";
            if (seedUp.Count > 0)
                upSql += "\n\n-- Seed data\n" + JoinBatches.Join(seedUp, separator);
            var downSql = $"-- Rollback migration for {dialect}\n{Render(dialect, down, separator)}";
            if (seedDown.Count > 0)
                downSql += "\n\n-- Seed data\n" + JoinBatches.Join(seedDown, separator);
            output.Add(new($"{Pad(number)}_up.sql", GenerateSql.Finalize(upSql, dialect, casing.Naming)));
            output.Add(new($"{Pad(number)}_down.sql", GenerateSql.Finalize(downSql, dialect, casing.Naming)));
            previous = current;
        }
        return output;
    }

    static IEnumerable<IStatement> ProcedureDiff(
        string dialect,
        IReadOnlyDictionary<string, string> settings,
        IReadOnlyList<SqlTable> before,
        IReadOnlyList<SqlTable> after,
        PackCasing casing)
    {
        if (!SqlSchema.UseStoredProcedures(settings)) return [];
        var previous = before.ToDictionary(table => table.Name, StringComparer.Ordinal);
        var groups = ByGroups.From(after);
        return after
            .Where(table => previous.TryGetValue(table.Name, out var prior)
                && SqlSchema.HasAuditColumns(table)
                && FieldsOf(prior) != FieldsOf(table))
            .SelectMany(table => GenerateProcedures.ForTable(dialect, table, casing, groups.GetValueOrDefault(table.Name) ?? []));
    }

    static string FieldsOf(SqlTable table) =>
        string.Join("|", table.Fields.Select(field => $"{field.Name}:{field.Type}"));

    static VersionSnapshot Empty() => VersionSnapshot.Combine([]);

    static bool Matches(VersionSnapshot snapshot, string path, string? liveText, string? whenMissing)
    {
        var has = snapshot.Documents.ContainsKey(path);
        var live = string.IsNullOrEmpty(liveText) ? whenMissing : liveText;
        if (string.IsNullOrEmpty(live)) return !has;
        var snapshotYaml = has ? snapshot.Yaml(path) : whenMissing is null ? null : Fold(path, whenMissing);
        return snapshotYaml is not null && snapshotYaml == Fold(path, live);
    }

    static bool Has(IReadOnlyDictionary<string, string> files, string name) =>
        files.TryGetValue(name, out var text) && text != "";

    static string Fold(string path, string text)
    {
        var (root, errors) = YamlDocuments.Parse(text);
        if (errors.Count > 0) throw new InvalidOperationException($"{path}: {errors[0].Message}");
        var envelope = new YamlMap();
        envelope.Entries.Add(new YamlEntry { Key = "added", Value = new YamlScalar { Value = true } });
        if (root is not null) envelope.Entries.Add(new YamlEntry { Key = "content", Value = root });
        var doc = new YamlMap();
        doc.Entries.Add(new YamlEntry { Key = path, Value = envelope });
        return VersionSnapshot.Combine([doc]).Yaml(path);
    }

    static YamlValue ParseDiff(string text, string source)
    {
        var (root, errors) = YamlDocuments.Parse(text);
        if (errors.Count > 0) throw new InvalidOperationException($"{source}: {errors[0].Message}");
        return root is YamlMap ? root : throw new InvalidOperationException($"{source}: expected a mapping");
    }

    static (IReadOnlyList<SqlTable> Tables, StatementNaming Naming, IReadOnlyDictionary<string, IReadOnlyList<SeedRow>> Seeds) Compile(
        VersionSnapshot snapshot,
        IReadOnlyDictionary<string, string> settings)
    {
        var files = snapshot.Documents.Keys.ToDictionary(path => path, path => snapshot.Yaml(path), StringComparer.Ordinal);
        files.TryAdd(SpecParser.DatasourceYaml, DefaultDatasourceYaml);
        var spec = SpecParser.Parse(files);
        var types = SqlSchema.From(spec);
        var naming = PackCasing.Of(settings).Naming;
        return (SqlSchema.BuildLive(types, naming), naming, spec.DatasourceSeeds);
    }

    static string Render(string dialect, IEnumerable<IStatement> statements, string separator) =>
        JoinBatches.Join(statements.Select(statement => GenerateSql.Render(dialect, statement)), separator);

    static string Pad(int n) => n.ToString("0000");

}
