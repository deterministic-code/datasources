namespace Datasources;

public sealed class Schema
{
    public Schema(IReadOnlyDictionary<string, SchemaColumn> columns) => Columns = columns;
    public IReadOnlyDictionary<string, SchemaColumn> Columns { get; }
    public IReadOnlyDictionary<string, string> Types =>
        Columns.ToDictionary(pair => pair.Key, pair => pair.Value.Type, StringComparer.Ordinal);
}
