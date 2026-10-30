namespace Datasources.Oracle;

internal sealed class TypeMap : IMapping<string, string>
{
    readonly IReadOnlyDictionary<string, string> dictionary;
    readonly Dictionary<string, string> reverse = [];

    public TypeMap(IReadOnlyDictionary<string, string> dictionary)
    {
        this.dictionary = dictionary;
        foreach (var (specType, native) in dictionary)
            reverse.TryAdd(native, specType);
    }

    public string Convert(string from) =>
        dictionary.TryGetValue(from, out var native)
            ? native
            : throw new InvalidOperationException($"Unknown spec field type: {from}");

    public string ReverseConvert(string native) =>
        reverse.TryGetValue(native, out var spec)
            ? spec
            : throw new InvalidOperationException($"Unknown native field type: {native}");

    public string Convert(SpecField field)
    {
        if (field.Type == "string")
            return field.Size is null or "unlimited" ? "CLOB" : $"VARCHAR2({field.Size})";
        if (field.Type == "character")
            return $"CHAR({(field.Size is null ? 1 : System.Convert.ToInt32(field.Size))})";
        if (field.Type == "decimal")
        {
            if (field.Size is null) return "NUMBER";
            if (field.Size is int[] { Length: 2 } pair) return $"NUMBER({pair[0]}, {pair[1]})";
            if (field.Size is IReadOnlyList<int> { Count: 2 } list) return $"NUMBER({list[0]}, {list[1]})";
            throw new InvalidOperationException($"decimal field \"{field.Name}\" size must be [precision, scale] tuple; got {field.Size}");
        }
        if (field.Type == "binary")
            return field.Size is null or "unlimited" ? "BLOB" : $"RAW({field.Size})";
        return Convert(field.Type);
    }
}
