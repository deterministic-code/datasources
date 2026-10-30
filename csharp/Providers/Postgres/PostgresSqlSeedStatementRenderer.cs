using System.Globalization;
using Datasources;

namespace Datasources.Postgres;

internal sealed class PostgresSqlSeedStatementRenderer
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
        var insert = $"INSERT INTO {table} ({cols}) VALUES ({values})";
        if (!statement.Identity || statement.IdColumn is not { } id)
            return insert;
        return $"{insert};\nSELECT setval(pg_get_serial_sequence('{statement.Table}', '{id}'), (SELECT MAX({Quote(id)}) FROM {table}))";
    }

    static string Quote(string ident) => $"\"{ident}\"";

    static string Literal(SqlSeedColumn column)
    {
        if (column.Value is null)
            return column.Field.DefaultValue is { } def ? DefaultSql(column.Field.Type, def) : "NULL";
        return column.Field.Type switch
        {
            "boolean" => Truthy(column.Value) ? "TRUE" : "FALSE",
            "integer" or "unsignedinteger" or "smallinteger" or "unsignedsmallinteger"
                or "biginteger" or "unsignedbiginteger" or "reference" or "float" or "decimal" =>
                Convert.ToString(column.Value, CultureInfo.InvariantCulture) ?? "NULL",
            "binary" => $"'\\x{column.Value}'",
            _ => QuoteString(column.Value),
        };
    }

    static string DefaultSql(string type, object def)
    {
        var text = Convert.ToString(def) ?? "";
        return (type, text) switch
        {
            ("datetime", "UtcNow") => "(NOW() AT TIME ZONE 'UTC')",
            ("datetime", "Now") => "LOCALTIMESTAMP",
            ("uuid", "NewId") => "gen_random_uuid()",
            ("boolean", _) => text is "true" or "True" ? "TRUE" : "FALSE",
            _ => QuoteString(text),
        };
    }

    static bool Truthy(object value) => value is true or "true" or "True" or 1 or 1L;

    static string QuoteString(object value) =>
        $"'{Convert.ToString(value, CultureInfo.InvariantCulture)?.Replace("'", "''", StringComparison.Ordinal)}'";
}
