using Datasources;

namespace Datasources.SqlServer;

internal sealed class SqlServerAlterTableStatementRenderer : AlterTableStatementTextRenderer
{
    protected override string Quote(string name) => SqlServerDriver.Quote(name);

    protected override string Type(Column column) => SqlServerDriver.NativeType(column);

    public override string Render(AlterTableStatement statement) => statement.Action switch
    {
        AlterTableStatement.RenameColumn rename =>
            $"EXEC sp_rename '{statement.Table}.{rename.From}', '{rename.To}', 'COLUMN'",
        AlterTableStatement.RenameTable rename =>
            $"EXEC sp_rename '{statement.Table}', '{rename.To}'",
        _ => base.Render(statement)
    };

    protected override string ActionSql(AlterTableStatement.IAction action) => action switch
    {
        AlterTableStatement.AlterColumn alter =>
            $"ALTER COLUMN {Quote(alter.Column.Name)} {Type(alter.Column)}{NotNullSql(alter.Column.NotNull)}",
        _ => base.ActionSql(action)
    };
}
