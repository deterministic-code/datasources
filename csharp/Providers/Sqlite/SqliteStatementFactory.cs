using Datasources;

namespace Datasources.Sqlite;

internal sealed class SqliteStatementFactory
{
    public IStatement Transform(IStatement statement) => statement switch
    {
        CreateTableStatement { Table: not null } create =>
            new CreateTableStatement(new SqliteCreateTableStatementRenderer().Render(create)),
        InsertedIdStatement inserted => new SelectStatement(InsertedIdStatement.Select("sqlite", inserted.Column)),
        _ => statement
    };
}
