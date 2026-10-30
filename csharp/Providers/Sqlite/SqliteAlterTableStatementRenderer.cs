using Datasources;

namespace Datasources.Sqlite;

internal sealed class SqliteAlterTableStatementRenderer : AlterTableStatementTextRenderer
{
    protected override string Quote(string name) => SqliteDriver.Quote(name);

    protected override string Type(Column column) => SqliteDriver.NativeType(column);

    protected override string Default(Column column) =>
        column.Default is null
            ? ""
            : column.Default.Name is null
                ? $" DEFAULT {SqliteDriver.NativeDefault(column)}"
                : $" CONSTRAINT {Quote(column.Default.Name)} DEFAULT {SqliteDriver.NativeDefault(column)}";

    protected override string ActionSql(AlterTableStatement.IAction action) => action switch
    {
        AlterTableStatement.AlterColumn => throw new InvalidOperationException("SQLite cannot ALTER COLUMN"),
        AlterTableStatement.AddConstraint => throw new InvalidOperationException("SQLite cannot ADD CONSTRAINT"),
        AlterTableStatement.DropConstraint => throw new InvalidOperationException("SQLite cannot DROP CONSTRAINT"),
        _ => base.ActionSql(action)
    };
}
