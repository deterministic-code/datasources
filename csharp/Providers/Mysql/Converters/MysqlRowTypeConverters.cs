namespace Datasources.Mysql;

internal static class MysqlRowTypeConverters
{
    public static readonly IReadOnlyDictionary<string, TypeConverter<Value, Value>> All = Catalog();

    static IReadOnlyDictionary<string, TypeConverter<Value, Value>> Catalog()
    {
        var integer = new IntegerConverter();
        return new Dictionary<string, TypeConverter<Value, Value>>
        {
            ["boolean"] = new BoolConverter(),
            ["datetime"] = new DateTimeConverter(),
            ["uuid"] = new UuidConverter(),
            ["decimal"] = new DecimalConverter(),
            ["binary"] = new BinaryConverter(),
            ["integer"] = integer,
            ["unsignedinteger"] = integer,
            ["smallinteger"] = integer,
            ["unsignedsmallinteger"] = integer,
            ["biginteger"] = integer,
            ["unsignedbiginteger"] = integer,
        };
    }

    public static Value Reverse(Value value) => value switch
    {
        Value.Bool => All["boolean"].ReverseConvert(value),
        Value.DateTime => All["datetime"].ReverseConvert(value),
        Value.Guid => All["uuid"].ReverseConvert(value),
        Value.Decimal => All["decimal"].ReverseConvert(value),
        Value.Binary => All["binary"].ReverseConvert(value),
        _ => value
    };
}
