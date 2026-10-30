namespace Datasources.Mysql;

internal sealed class MysqlCreateTableStatementRenderer : CreateTableStatementTextRenderer
{
    protected override string Quote(string name) => MysqlDriver.Quote(name);

    protected override string Type(Column column) => MysqlDriver.NativeType(column);

    protected override string Default(Column column) =>
        column.Default is null ? "" : $" DEFAULT {MysqlDriver.NativeDefault(column)}";

    protected override string NotNullSql(NotNull? value) =>
        value is null ? "" : " NOT NULL";

    protected override string Identity(Column column) => column.Identity ? " AUTO_INCREMENT" : "";
}
