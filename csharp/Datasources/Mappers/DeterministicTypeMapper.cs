namespace Datasources;

public sealed class DeterministicTypeMapper : IMapper<string, string>
{
    static readonly Dictionary<string, string> Forward = new()
    {
        ["Boolean"] = "boolean",
        ["Byte"] = "unsignedsmallinteger",
        ["SByte"] = "smallinteger",
        ["Int16"] = "smallinteger",
        ["UInt16"] = "unsignedsmallinteger",
        ["Int32"] = "integer",
        ["UInt32"] = "unsignedinteger",
        ["Int64"] = "biginteger",
        ["UInt64"] = "unsignedbiginteger",
        ["Guid"] = "uuid",
        ["DateTime"] = "datetime",
        ["Decimal"] = "decimal",
        ["Byte[]"] = "binary",
    };
    static readonly Dictionary<string, string> Reverse = new()
    {
        ["boolean"] = "Boolean",
        ["smallinteger"] = "Int16",
        ["unsignedsmallinteger"] = "UInt16",
        ["integer"] = "Int32",
        ["unsignedinteger"] = "UInt32",
        ["biginteger"] = "Int64",
        ["unsignedbiginteger"] = "UInt64",
        ["uuid"] = "Guid",
        ["datetime"] = "DateTime",
        ["decimal"] = "Decimal",
        ["binary"] = "Byte[]",
    };

    public string Map(string value) => Forward.TryGetValue(value, out var spec) ? spec : "";

    public string ReverseMap(string value) => Reverse.TryGetValue(value, out var name) ? name : "";
}
