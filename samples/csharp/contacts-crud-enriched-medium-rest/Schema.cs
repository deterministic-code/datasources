using Datasources;
using Datasources.SqlSpecs;

namespace ContactsCrudEnrichedMediumRest;

public static class Schema
{
    public static string Ident(string provider, string name) => provider == "mysql"
        ? $"`{name.Replace("`", "``")}`"
        : $"\"{name.Replace("\"", "\"\"")}\"";

    public static IReadOnlyList<CreateTableStatement> Tables(string provider, IDriver driver) =>
        SchemaSpec.Tables(
            "contacts-rest",
            provider,
            name => Ident(provider, name),
            canonical => provider == "sqlite" ? driver.ColumnType(canonical) : canonical);
}
