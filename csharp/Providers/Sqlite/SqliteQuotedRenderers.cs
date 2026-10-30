using Datasources;

namespace Datasources.Sqlite;

internal sealed class SqliteDropTableStatementRenderer : DropTableStatementTextRenderer
{
    protected override string Quote(string name) => SqliteDriver.Quote(name);
}

internal sealed class SqliteCreateIndexStatementRenderer : CreateIndexStatementTextRenderer
{
    protected override string Quote(string name) => SqliteDriver.Quote(name);
}

internal sealed class SqliteDropIndexStatementRenderer : DropIndexStatementTextRenderer
{
    protected override string Quote(string name) => SqliteDriver.Quote(name);
}

internal sealed class SqliteDropTriggerStatementRenderer : DropTriggerStatementTextRenderer
{
    protected override string Quote(string name) => SqliteDriver.Quote(name);
}

internal sealed class SqliteForeignKeyStatementRenderer
{
    public string Render(ForeignKeyStatement statement) =>
        throw new InvalidOperationException(
            statement.Kind == "drop" ? "SQLite cannot DROP CONSTRAINT" : "SQLite cannot ADD CONSTRAINT");
}
