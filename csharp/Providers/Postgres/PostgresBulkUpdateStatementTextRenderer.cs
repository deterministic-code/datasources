using System.Globalization;
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
        Value.Text(var text) when IsoDate(text) => "datetime",
        Value.Text(var text) when BinaryConverter.IsBase64(text) => "binary",
        Value.Text => "string",
        Value.DateTime => "datetime",
        Value.Guid => "uuid",
        Value.Decimal => "decimal",
        Value.Binary => "binary",
        _ => null
    };

    static bool IsoDate(string text) =>
        text.Contains('T', StringComparison.Ordinal) && DateTime.TryParse(
            text, CultureInfo.InvariantCulture,
            DateTimeStyles.AdjustToUniversal | DateTimeStyles.AssumeUniversal, out _);
}
