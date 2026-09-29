namespace Datasources.Mysql;

internal sealed class MysqlStatementFactory
{
    public IStatement Transform(IStatement statement) => statement switch
    {
        CreateTableStatement { Table: not null } create =>
            new CreateTableStatement(new MysqlCreateTableStatementRenderer().Render(create)),
        SelectInsertedIdStatement inserted => new SelectStatement($"SELECT LAST_INSERT_ID() AS {inserted.Column}"),
        _ => statement
    };
}
