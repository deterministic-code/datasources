using Datasources;

namespace Datasources.Postgres;

internal sealed class PostgresBulkUpdateStatementTextRenderer : BulkUpdateStatementTextRenderer
{
    protected override string Cell(string column, int index, Value value) =>
        Logical(value) is { } type && PostgresDriver.ColumnTypes.TryGetValue(type, out var sql)
            ? $"${column}__{index}::{sql} AS {column}"
            : $"${column}__{index} AS {column}";

    static string? Logical(Value value) => value switch
    {
        Value.Bool => "bool",
        Value.Int => "int",
        Value.Float => "float",
        Value.Text => "string",
        Value.DateTime => "datetime",
        Value.Guid => "uuid",
        Value.Decimal => "decimal",
        Value.Binary => "binary",
        _ => null
    };
}
