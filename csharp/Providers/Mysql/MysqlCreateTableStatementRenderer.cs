namespace Datasources.Mysql;

internal sealed class MysqlCreateTableStatementRenderer : CreateTableStatementRenderer
{
    protected override string Type(Column column) =>
        MysqlDriver.ColumnTypes.TryGetValue(column.Type, out var sql) ? sql : column.Type;

    protected override string Identity(Column column) => column.Identity ? " AUTO_INCREMENT" : "";
}
