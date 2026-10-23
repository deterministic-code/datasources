namespace Datasources.CodeGenerators;

public static class DatasourceYaml
{
    public static IReadOnlyDictionary<string, string> For(
        IReadOnlyDictionary<string, string> files,
        string dialect) =>
        files.TryGetValue($"datasource.{dialect}.yaml", out var yaml)
            ? new Dictionary<string, string>(files, StringComparer.Ordinal) { [SpecParser.DatasourceYaml] = yaml }
            : files;
}
