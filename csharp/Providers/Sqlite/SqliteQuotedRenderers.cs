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
        return new SqliteRebuildTableStatementRenderer().Render(new RebuildTableStatement(
            statement.Table,
            statement.Create,
            [.. statement.Create.Columns.Select(column => new RebuildColumn(column.Name, column.Name))]));
    }
}

internal sealed class SqliteRebuildTableStatementRenderer
{
    public string Render(RebuildTableStatement statement)
    {
        var dest = statement.Create.Table;
        var tmp = $"{dest}__new";
        var rebuilt = new CreateTableStatement(tmp, statement.Create.Columns, statement.Create.Constraints);
        var names = string.Join(", ", statement.Columns.Select(column => SqliteDriver.Quote(column.Name)));
        var values = string.Join(", ", statement.Columns.Select(Select));
        var from = SqliteDriver.Quote(statement.Table);
        var to = SqliteDriver.Quote(dest);
        var tmpName = SqliteDriver.Quote(tmp);
        List<string> parts =
        [
            new SqliteCreateTableStatementRenderer().Render(rebuilt),
            $"INSERT INTO {tmpName} ({names}) SELECT {values} FROM {from}",
            new SqliteDropTableStatementRenderer().Render(new DropTableStatement(statement.Table)),
            $"ALTER TABLE {tmpName} RENAME TO {to}",
        ];
        parts.AddRange(statement.Recreate.Select(Recreate).Where(sql => sql != ""));
        return string.Join(";\n", parts.Where(sql => sql != ""));
    }

    static string Select(RebuildColumn column) =>
        column.Source is { } source
            ? column.Cast is { } cast
                ? $"CAST({SqliteDriver.Quote(source)} AS {cast})"
                : SqliteDriver.Quote(source)
            : column.Literal ?? "NULL";

    static string Recreate(IStatement statement) => statement switch
    {
        CreateIndexStatement index => new SqliteCreateIndexStatementRenderer().Render(index),
        CreateTriggerStatement trigger => new SqliteCreateTriggerStatementRenderer().Render(trigger),
        _ => throw new InvalidOperationException($"{statement.GetType().Name} cannot be recreated during a table rebuild"),
    };
}
