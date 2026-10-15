namespace Datasources.CodeGenerators;

public static class FieldType
{
    static readonly HashSet<string> Primitives =
    [
        "string", "character", "number", "integer", "unsignedinteger",
        "biginteger", "unsignedbiginteger", "smallinteger", "unsignedsmallinteger",
        "float", "decimal", "boolean", "datetime", "binary", "uuid", "reference",
    ];

    public static (string Kind, string Base, bool IsArray, bool IsMap) Parse(string raw)
    {
        var isArray = raw.EndsWith("[]", StringComparison.Ordinal);
        var isMap = raw.EndsWith("{}", StringComparison.Ordinal);
        var baseName = isArray || isMap ? raw[..^2] : raw;
        return (Primitives.Contains(baseName) ? "primitive" : "type", baseName, isArray, isMap);
    }
}
