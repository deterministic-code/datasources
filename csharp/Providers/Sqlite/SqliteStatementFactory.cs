using Datasources;

namespace Datasources.Sqlite;

internal sealed class SqliteStatementFactory : CreateTableStatementRenderer
{
    protected override string Type(Column column) => column.Type switch
    {
        "int" or "uuid" or "string" or "datetime" or "bool" or "decimal" or "float" or "binary"
            => new SqliteDriver().ColumnType(column.Type),
        _ => column.Type
    };

    public IStatement Transform(IStatement statement) => statement switch
    {
        CreateTableStatement { Table: not null } create => new CreateTableStatement(Render(create)),
        InsertedIdStatement inserted => new SelectStatement($"SELECT last_insert_rowid() AS {inserted.Column}"),
        _ => statement
    };
}
