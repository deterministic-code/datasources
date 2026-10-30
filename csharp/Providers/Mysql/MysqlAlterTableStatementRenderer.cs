using Datasources;

namespace Datasources.Mysql;

internal sealed class MysqlAlterTableStatementRenderer : AlterTableStatementTextRenderer
{
    protected override string Quote(string name) => MysqlDriver.Quote(name);

    protected override string Type(Column column) => MysqlDriver.NativeType(column);

    protected override string Default(Column column) =>
        column.Default is null ? "" : $" DEFAULT {MysqlDriver.NativeDefault(column)}";

    protected override string NotNullSql(NotNull? value) =>
        value is null ? "" : " NOT NULL";

    protected override string Identity(Column column) => column.Identity ? " AUTO_INCREMENT" : "";

    protected override string ActionSql(AlterTableStatement.IAction action) => action switch
    {
        AlterTableStatement.AlterColumn alter => $"MODIFY COLUMN {Column(alter.Column)}",
        _ => base.ActionSql(action)
    };
}
