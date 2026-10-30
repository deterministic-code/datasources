using System.Globalization;
using Datasources;

namespace Datasources.Mysql;

internal sealed class MysqlSqlSeedStatementRenderer
{
    public string Render(SqlSeedStatement statement)
    {
        var table = Quote(statement.Table);
        if (statement.Kind == "delete")
        {
            var key = statement.Columns[0];
            return $"DELETE FROM {table} WHERE {Quote(key.Column)} = {Literal(key)}";
        }
        var cols = string.Join(", ", statement.Columns.Select(c => Quote(c.Column)));
        var values = string.Join(", ", statement.Columns.Select(Literal));
        return $"INSERT INTO {table} ({cols}) VALUES ({values})";
    }

    static string Quote(string ident) => $"`{ident}`";

    static string Literal(SqlSeedColumn column)
    {
        if (column.Value is null)
            return column.Field.DefaultValue is { } def ? DefaultSql(column.Field.Type, def) : "NULL";
        return column.Field.Type switch
        {
            "boolean" => Truthy(column.Value) ? "1" : "0",
            "integer" or "unsignedinteger" or "smallinteger" or "unsignedsmallinteger"
                or "biginteger" or "unsignedbiginteger" or "reference" or "float" or "decimal" =>
                Convert.ToString(column.Value, CultureInfo.InvariantCulture) ?? "NULL",
            "binary" => $"(X'{column.Value}')",
            _ => QuoteString(column.Value),
        };
    }

    static string DefaultSql(string type, object def)
    {
        var text = Convert.ToString(def) ?? "";
        return (type, text) switch
        {
            ("datetime", "UtcNow") => "(UTC_TIMESTAMP(3))",
            ("datetime", "Now") => "(NOW(3))",
            ("uuid", "NewId") => "(UUID())",
            ("boolean", _) => text is "true" or "True" ? "1" : "0",
            _ => QuoteString(text),
        };
    }

    static bool Truthy(object value) => value is true or "true" or "True" or 1 or 1L;

    static string QuoteString(object value) =>
        $"'{Convert.ToString(value, CultureInfo.InvariantCulture)?.Replace("'", "''", StringComparison.Ordinal)}'";
}
