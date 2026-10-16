using Datasources;

namespace Datasources.Mysql;

internal sealed class MysqlAlterTableStatementRenderer : AlterTableStatementTextRenderer
{
    protected override string Type(Column column) =>
        MysqlDriver.ColumnTypes.TryGetValue(column.Type, out var sql) ? sql : column.Type;

    protected override string Identity(Column column) => column.Identity ? " AUTO_INCREMENT" : "";

    protected override string ActionSql(AlterTableStatement.IAction action) => action switch
    {
        AlterTableStatement.AlterColumn alter => $"MODIFY COLUMN {Column(alter.Column)}",
        _ => base.ActionSql(action)
    };
}
