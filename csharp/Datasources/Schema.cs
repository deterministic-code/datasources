namespace Datasources;

public sealed class Schema
{
    public Schema(IReadOnlyDictionary<string, string> columns) => Columns = columns;
    public IReadOnlyDictionary<string, string> Columns { get; }
}
