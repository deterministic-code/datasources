namespace Datasources.Mysql;

internal sealed class MysqlStatementFactory
{
    public IStatement Transform(IStatement statement) => statement switch
    {
        CreateTableStatement { Table: not null } create =>
            new CreateTableStatement(new MysqlCreateTableStatementRenderer().Render(create)),
        InsertedIdStatement inserted => new SelectStatement(InsertedIdStatement.Select("mysql", inserted.Column)),
        _ => statement
    };
}
