namespace Datasources.Mysql;

internal static class MysqlRowTypeConverters
{
    public static readonly IReadOnlyDictionary<string, TypeConverter<Value, Value>> All =
        new Dictionary<string, TypeConverter<Value, Value>>
        {
            ["boolean"] = new BoolConverter(),
            ["datetime"] = new DateTimeConverter(),
            ["uuid"] = new UuidConverter(),
            ["decimal"] = new DecimalConverter(),
            ["binary"] = new BinaryConverter(),
            ["integer"] = new IntegerConverter(),
            ["number"] = new IntegerConverter(),
            ["unsignedinteger"] = new IntegerConverter(),
            ["smallinteger"] = new IntegerConverter(),
            ["unsignedsmallinteger"] = new IntegerConverter(),
            ["biginteger"] = new IntegerConverter(),
            ["unsignedbiginteger"] = new IntegerConverter(),
        };

    public static Value Reverse(Value value) => value switch
    {
        Value.Bool => All["boolean"].ReverseConvert(value),
        Value.DateTime => All["datetime"].ReverseConvert(value),
        Value.Text(var text) when text.Contains('T') && text.Contains('-') => All["datetime"].ReverseConvert(value),
        Value.Guid => All["uuid"].ReverseConvert(value),
        Value.Decimal => All["decimal"].ReverseConvert(value),
        Value.Binary => All["binary"].ReverseConvert(value),
        _ => value
    };
}
