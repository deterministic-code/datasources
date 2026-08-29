using Datasources;

namespace Datasources.Sqlite;

public interface ISqliteDriver
{
    Task Open(string filename);
    Task Close();
    Task<SqliteExecuteResult> Execute(string sql, IReadOnlyDictionary<string, Value> parameters);
    Task<IReadOnlyList<IReadOnlyDictionary<string, Value>>> Query(string sql, IReadOnlyDictionary<string, Value> parameters);
    Task Begin(IsolationLevel isolation = IsolationLevel.Serializable);
    Task Commit();
    Task Rollback();
}
