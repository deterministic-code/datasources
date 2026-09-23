namespace Datasources.Postgres;

internal static class PostgresRowTypeConverters
{
    public static readonly IReadOnlyDictionary<string, TypeConverter<Value, Value>> All =
        new Dictionary<string, TypeConverter<Value, Value>>
        {
            ["bool"] = new BoolConverter(),
            ["datetime"] = new DateTimeConverter(),
            ["uuid"] = new UuidConverter(),
            ["decimal"] = new DecimalConverter(),
            ["binary"] = new BinaryConverter(),
        };
}
