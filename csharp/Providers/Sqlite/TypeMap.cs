namespace Datasources.Sqlite;

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
            return field.Size is null or "unlimited" ? "TEXT" : $"VARCHAR({field.Size})";
        return Convert(field.Type);
    }
}
