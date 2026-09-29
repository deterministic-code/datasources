namespace Datasources.Postgres;

internal sealed class PostgresStatementFactory
{
    public IStatement Transform(IStatement statement) => statement switch
    {
        CreateTableStatement { Table: not null } create =>
            new CreateTableStatement(new PostgresCreateTableStatementRenderer().Render(create)),
        SelectInsertedIdStatement inserted => new SelectStatement($"SELECT lastval() AS {inserted.Column}"),
        _ => statement
    };
}
