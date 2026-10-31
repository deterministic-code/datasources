namespace Datasources;

internal static class SchemaDefault
{
    public static Value Of(SchemaColumn column) => (column.Nullable, column.Type) switch
    {
        (true, _) => Value.Null.Instance,
        (_, "string") => "",
        (_, "boolean") => false,
        (_, var type) when type.EndsWith("integer", StringComparison.Ordinal) => 0L,
        _ => Value.Null.Instance
    };
}
