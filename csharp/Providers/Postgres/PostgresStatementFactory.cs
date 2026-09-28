namespace Datasources.Postgres;

internal sealed class PostgresStatementFactory
{
    public IStatement Transform(IStatement statement) => statement switch
    {
        CreateTableStatement { Table: not null } create =>
            new CreateTableStatement(new PostgresCreateTableStatementRenderer().Render(create)),
        InsertedIdStatement inserted => new SelectStatement(InsertedIdStatement.Select("postgres", inserted.Column)),
        _ => statement
    };
}
