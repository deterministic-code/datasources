using Datasources;

namespace Datasources.Oracle;

internal sealed class OracleAlterTableStatementRenderer : AlterTableStatementTextRenderer
{
    protected override string ActionSql(AlterTableStatement.IAction action) => action switch
    {
        AlterTableStatement.AddColumn add => $"ADD ({Column(add.Column)})",
        AlterTableStatement.AlterColumn alter => $"MODIFY ({Column(alter.Column)})",
        _ => base.ActionSql(action)
    };
}
