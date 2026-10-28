namespace Datasources.CodeGenerators;

public static class JoinBatches
{
    public const string Default = "GO";

    public static string Of(IReadOnlyDictionary<string, string> settings) =>
        settings.TryGetValue("datasource.batch_separator", out var value) ? Require(value) : Default;

    public static string Join(IEnumerable<string> parts, string separator)
    {
        var chunks = parts.Where(part => part != "").ToList();
        return chunks.Count == 0
            ? ""
            : string.Join(separator.Trim() == "" ? "\n" : $"\n{separator}\n", chunks);
    }

    static string Require(string value) =>
        value.Contains('\n') || value.Contains('\r')
            ? throw new InvalidOperationException("datasource.batch_separator must be a single line")
            : value;
}
