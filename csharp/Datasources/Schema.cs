namespace Datasources;

public sealed class Schema
{
    public Schema(IReadOnlyDictionary<string, SchemaColumn> columns) => Columns = columns;
    public IReadOnlyDictionary<string, SchemaColumn> Columns { get; }
    public IReadOnlyList<string> Ids =>
        [.. Columns.Where(pair => pair.Value.IsId).Select(pair => pair.Key)];
    public IReadOnlyDictionary<string, string> Types =>
        Columns.ToDictionary(pair => pair.Key, pair => pair.Value.Type, StringComparer.Ordinal);
    public string DictionaryKey =>
        Columns.ContainsKey("Key") ? "Key" : Columns.ContainsKey("key") ? "key" : "Key";
    public string DictionaryValue =>
        Columns.ContainsKey("Value") ? "Value" : Columns.ContainsKey("value") ? "value" : "Value";
    public IReadOnlyList<string> DictionaryValueColumns(string relatedKey, string mapKey) =>
        [.. Columns
            .Where(pair => !pair.Value.IsId && pair.Key != relatedKey && pair.Key != mapKey)
            .Select(pair => pair.Key)];
}
