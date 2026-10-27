namespace Datasources.Mysql;

internal sealed class MysqlCreateTableStatementRenderer : CreateTableStatementTextRenderer
{
    protected override string Type(Column column) =>
        MysqlDriver.ColumnTypes.TryGetValue(column.Type, out var sql) ? sql : column.Type;

    protected override string Default(Column column) =>
        column.Default is null ? "" : $" DEFAULT {column.Default.Value}";

    protected override string NotNullSql(NotNull? value) =>
        value is null ? "" : " NOT NULL";

    protected override string Identity(Column column) => column.Identity ? " AUTO_INCREMENT" : "";
}
