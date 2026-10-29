namespace Datasources;

public sealed class DeterministicTypeMapper : IMapper<string, string>
{
    static readonly Dictionary<string, string> Forward = new()
    {
        ["Boolean"] = "boolean",
        ["Byte"] = "integer",
        ["SByte"] = "integer",
        ["Int16"] = "integer",
        ["UInt16"] = "integer",
        ["Int32"] = "integer",
        ["UInt32"] = "integer",
        ["Int64"] = "integer",
        ["UInt64"] = "integer",
        ["Guid"] = "uuid",
        ["DateTime"] = "datetime",
        ["Decimal"] = "decimal",
        ["Byte[]"] = "binary",
    };
    static readonly Dictionary<string, string> Reverse = new()
    {
        ["boolean"] = "Boolean",
        ["integer"] = "Int32",
        ["uuid"] = "Guid",
        ["datetime"] = "DateTime",
        ["decimal"] = "Decimal",
        ["binary"] = "Byte[]",
    };

    public string Map(string value) => Forward.TryGetValue(value, out var spec) ? spec : "";

    public string ReverseMap(string value) => Reverse.TryGetValue(value, out var name) ? name : "";
}
