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

    public static Value Reverse(Value value) => value switch
    {
        Value.Bool => All["bool"].ReverseConvert(value),
        Value.DateTime => All["datetime"].ReverseConvert(value),
        Value.Guid => All["uuid"].ReverseConvert(value),
        Value.Decimal => All["decimal"].ReverseConvert(value),
        Value.Binary => All["binary"].ReverseConvert(value),
        Value.Text(var text) when BinaryConverter.IsBase64(text) => All["binary"].ReverseConvert(value),
        _ => value
    };
}
