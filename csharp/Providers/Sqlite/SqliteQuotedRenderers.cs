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
    public string Render(ForeignKeyStatement statement)
    {
        if (statement.Kind == "drop") return "";
        if (statement.Create is null) throw new InvalidOperationException("SQLite cannot ADD CONSTRAINT");
        var tmp = $"{statement.Table}__new";
        var rebuilt = new CreateTableStatement(tmp, statement.Create.Columns, statement.Create.Constraints);
        var cols = string.Join(", ", statement.Create.Columns.Select(column => SqliteDriver.Quote(column.Name)));
        var from = SqliteDriver.Quote(statement.Table);
        var to = SqliteDriver.Quote(tmp);
        return string.Join(";\n",
            new SqliteCreateTableStatementRenderer().Render(rebuilt),
            $"INSERT INTO {to} ({cols}) SELECT {cols} FROM {from}",
            new SqliteDropTableStatementRenderer().Render(new DropTableStatement(statement.Table)),
            $"ALTER TABLE {to} RENAME TO {from}");
    }
}
