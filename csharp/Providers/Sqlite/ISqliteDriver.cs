using Datasources;

namespace Datasources.Sqlite;

public interface ISqliteDriver
{
    Task Open(string filename);
    Task Close();
    Task<SqliteExecuteResult> Execute(string sql, Row parameters);
    Task<SqliteExecuteResult> Execute(string sql, IReadOnlyList<Parameter>? parameters);
    Task<IReadOnlyList<Row>> Query(string sql, Row parameters);
    Task Begin(IsolationLevel isolation = IsolationLevel.Serializable);
    Task Commit();
    Task Rollback();
}
