using Datasources;

namespace Datasources.Sqlite;

internal sealed class SqliteCreateTableStatementRenderer : CreateTableStatementTextRenderer
{
    protected override string Type(Column column) =>
        SqliteDriver.ColumnTypes.TryGetValue(column.Type, out var sql) ? sql : column.Type;
}
