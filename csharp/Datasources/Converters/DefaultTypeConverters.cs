namespace Datasources;

public static class DefaultTypeConverters
{
    static readonly Dictionary<string, IReadOnlyDictionary<string, TypeConverter<Value, Value>>> ByProvider =
        new(StringComparer.OrdinalIgnoreCase);

    public static void Use(string provider, IReadOnlyDictionary<string, TypeConverter<Value, Value>> types) =>
        ByProvider[provider] = types;

    public static IReadOnlyDictionary<string, TypeConverter<Value, Value>> For(string provider) =>
        ByProvider.TryGetValue(provider, out var types) ? types : Empty;

    static readonly IReadOnlyDictionary<string, TypeConverter<Value, Value>> Empty =
        new Dictionary<string, TypeConverter<Value, Value>>();
}
