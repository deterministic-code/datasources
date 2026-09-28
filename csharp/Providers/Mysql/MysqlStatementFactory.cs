namespace Datasources.Mysql;

internal sealed class MysqlStatementFactory : CreateTableStatementRenderer
{
    protected override string Type(Column column) => column.Type switch
    {
        "int" or "uuid" or "string" or "datetime" or "bool" or "decimal" or "float" or "binary"
            => new MysqlDriver().ColumnType(column.Type),
        _ => column.Type
    };

    protected override string Identity(Column column) => column.Identity ? " AUTO_INCREMENT" : "";

    public IStatement Transform(IStatement statement) => statement switch
    {
        CreateTableStatement { Table: not null } create => new CreateTableStatement(Render(create)),
        _ => statement
    };
}
