namespace Datasources.Sqlite;

internal interface ISqliteCommands
{
    Task<SqliteExecuteResult> Execute(string sql, IReadOnlyList<Parameter>? parameters);
    Task<IReader<Row>> Open(string sql, Row parameters);
}
