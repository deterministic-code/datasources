using Datasources;

namespace Datasources.Postgres;

public interface IPostgresDriver
{
    Task Open(string url);
    Task Close();
    Task<PostgresExecuteResult> Execute(string sql, IReadOnlyDictionary<string, Value> parameters);
    Task<IReadOnlyList<IReadOnlyDictionary<string, Value>>> Query(string sql, IReadOnlyDictionary<string, Value> parameters);
    Task Begin(IsolationLevel isolation = IsolationLevel.Serializable);
    Task Commit();
    Task Rollback();
}
