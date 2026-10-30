using Datasources;

namespace Datasources.Oracle;

internal sealed class OracleAlterTableStatementRenderer : AlterTableStatementTextRenderer
{
    protected override string Quote(string name) => OracleDriver.Quote(name);

    protected override string Type(Column column) => OracleDriver.NativeType(column);

    protected override string Default(Column column) =>
        column.Default is null
            ? ""
            : column.Default.Name is null
                ? $" DEFAULT {OracleDriver.NativeDefault(column)}"
                : $" CONSTRAINT {Quote(column.Default.Name)} DEFAULT {OracleDriver.NativeDefault(column)}";

    protected override string ActionSql(AlterTableStatement.IAction action) => action switch
    {
        AlterTableStatement.AddColumn add => $"ADD ({Column(add.Column)})",
        AlterTableStatement.AlterColumn alter => $"MODIFY ({Column(alter.Column)})",
        _ => base.ActionSql(action)
    };
}
