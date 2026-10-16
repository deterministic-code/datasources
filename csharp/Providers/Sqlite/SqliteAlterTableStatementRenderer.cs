using Datasources;

namespace Datasources.Sqlite;

public sealed class SqliteAlterTableStatementRenderer : AlterTableStatementTextRenderer
{
    protected override string Type(Column column) =>
        SqliteDriver.ColumnTypes.TryGetValue(column.Type, out var sql) ? sql : column.Type;

    protected override string ActionSql(AlterTableStatement.IAction action) => action switch
    {
        AlterTableStatement.AlterColumn => throw new InvalidOperationException("SQLite cannot ALTER COLUMN"),
        AlterTableStatement.AddConstraint => throw new InvalidOperationException("SQLite cannot ADD CONSTRAINT"),
        AlterTableStatement.DropConstraint => throw new InvalidOperationException("SQLite cannot DROP CONSTRAINT"),
        _ => base.ActionSql(action)
    };
}
