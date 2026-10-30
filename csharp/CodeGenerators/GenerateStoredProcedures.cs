namespace Datasources.CodeGenerators;

public static class GenerateStoredProcedures
{
    public static IReadOnlyList<GenerateEntry> Generate(
        IReadOnlyDictionary<string, string> files,
        IReadOnlyDictionary<string, string> settings,
        IReadOnlyList<string> dialects)
    {
        var entries = new List<GenerateEntry>();
        foreach (var dialect in dialects)
            entries.AddRange(GenerateProcedures.ForDialect(GenerateSql.Require(dialect), files, settings));
        return entries;
    }
}
