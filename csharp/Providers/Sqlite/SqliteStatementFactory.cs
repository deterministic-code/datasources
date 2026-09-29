using Datasources;

namespace Datasources.Sqlite;

internal sealed class SqliteStatementFactory
{
    public IStatement Transform(IStatement statement) => statement switch
    {
        CreateTableStatement { Table: not null } create =>
            new CreateTableStatement(new SqliteCreateTableStatementRenderer().Render(create)),
        SelectInsertedIdStatement inserted => new SelectStatement($"SELECT last_insert_rowid() AS {inserted.Column}"),
        _ => statement
    };
}
