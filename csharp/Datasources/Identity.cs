namespace Datasources;

public static class Identity
{
    public static IReadOnlyList<string> Ids(Schema? schema) =>
        schema is null
            ? []
            : [.. schema.Columns.Where(pair => pair.Value.IsId).Select(pair => pair.Key)];
}
