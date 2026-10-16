using System.Text.RegularExpressions;

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

        var before = new Dictionary<string, object?>(StringComparer.Ordinal);
        var snapshots = new List<Dictionary<string, object?>>();
        foreach (var path in paths)
        {
            var text = files[path];
            if (text == "") continue;
            before = VersionDelta.ApplyDocument(before, VersionDelta.AsDocument(VersionDelta.Parse(text), path));
            snapshots.Add(new Dictionary<string, object?>(before, StringComparer.Ordinal));
        }
        if (snapshots.Count == 0) return [];

        var last = snapshots[^1];
        var liveTypes = VersionDelta.Parse(files.GetValueOrDefault(SpecParser.TypesYaml) ?? "");
        var liveDatasource = VersionDelta.Parse(files.GetValueOrDefault(SpecParser.DatasourceYaml) ?? DefaultDatasourceYaml);
        var snapshotDatasource = last.GetValueOrDefault("datasource.yaml") ?? VersionDelta.Parse(DefaultDatasourceYaml);
        if (!VersionDelta.Equal(last.GetValueOrDefault("types.yaml"), liveTypes)
            || !VersionDelta.Equal(snapshotDatasource, liveDatasource))
            throw new InvalidOperationException("types.yaml or datasource.yaml differs from the frozen version diffs; run datasources version first");

        var output = new List<GenerateEntry>();
        var previous = Compile(Empty(), settings);
        var number = 0;
        for (var index = 0; index < snapshots.Count; index++)
        {
            var current = Compile(snapshots[index], settings);
            var up = SchemaDiff.Of(dialect, previous.Tables, previous.Mapping, current.Tables, current.Mapping);
            var down = SchemaDiff.Of(dialect, current.Tables, current.Mapping, previous.Tables, previous.Mapping);
            if (up.Count == 0 && down.Count == 0)
            {
                previous = current;
                continue;
            }
            number++;
            var seeds = index == 0 && up.OfType<CreateTableStatement>().Any()
                ? SeedGenerator.Sections(dialect, current.Tables, current.Seeds, current.Mapping)
                : [];
            var upSql = $"-- Generated migration for {dialect}\n{Render(dialect, up)}";
            if (seeds.Count > 0)
                upSql += "\n\n-- Seed data\n" + string.Join("", seeds.Select(block => block.EndsWith('\n') ? block : block + "\n"));
            var downSql = $"-- Rollback migration for {dialect}\n{Render(dialect, down)}";
            output.Add(new($"{Pad(number)}_up.sql", Finalize(upSql, casing)));
            output.Add(new($"{Pad(number)}_down.sql", Finalize(downSql, casing)));
            previous = current;
        }
        return output;
    }

    static Dictionary<string, object?> Empty() => [];

    static (IReadOnlyList<SqlTable> Tables, SqlMapping Mapping, IReadOnlyDictionary<string, IReadOnlyList<SeedRow>> Seeds) Compile(
        Dictionary<string, object?> snapshot,
        IReadOnlyDictionary<string, string> settings)
    {
        var files = snapshot.ToDictionary(pair => pair.Key, pair => VersionDelta.Yaml(pair.Value), StringComparer.Ordinal);
        files.TryAdd(SpecParser.DatasourceYaml, DefaultDatasourceYaml);
        var spec = SpecParser.Parse(files);
        var types = SqlSchema.From(spec);
        var mapping = new SqlMapping(PackCasing.Of(settings), types);
        return (SqlSchema.BuildLive(types, mapping), mapping, spec.DatasourceSeeds);
    }

    static string Render(string dialect, IEnumerable<IStatement> statements) =>
        string.Join("\n", statements.Select(statement => SqlDialect.RenderSql(dialect, statement)).Where(sql => sql != ""));

    static string Pad(int n) => n.ToString("0000");

    static string Finalize(string text, PackCasing casing)
    {
        var output = casing.ApplyKeywords(System.Text.RegularExpressions.Regex.Replace(text, "\n{3,}", "\n\n"));
        return output.EndsWith('\n') ? output : $"{output}\n";
    }
}
