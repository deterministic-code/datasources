using Datasources;
using Datasources.SqlSpecs;

namespace UserRolesSimpleCli;

public static class Schema
{
    public static string Ident(string provider, string name) => Quote(provider, name);

    public static string Qualified(string provider, string name) =>
        string.Join(".", name.Split('.').Select(part => Quote(provider, part)));

    static string Quote(string provider, string id) => provider == "mysql"
        ? $"`{id.Replace("`", "``")}`"
        : $"\"{id.Replace("\"", "\"\"")}\"";

    public static Value Flag(string provider, bool value) => provider == "sqlite" ? value ? 1 : 0 : value;

    public static IReadOnlyList<CreateTableStatement> Tables(string provider, IDriver driver) =>
        SchemaSpec.Tables(
            "user-roles-cli",
            provider,
            name => Ident(provider, name),
            canonical => provider == "sqlite" ? driver.ColumnType(canonical) : canonical);

    public static IReadOnlyList<string> MysqlProcedures()
    {
        using var stream = typeof(Schema).Assembly.GetManifestResourceStream("mysql-procedures.sql")!;
        using var reader = new StreamReader(stream);
        return reader.ReadToEnd().Split("---", StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries);
    }
}
