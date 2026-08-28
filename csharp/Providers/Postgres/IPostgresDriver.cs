namespace Datasources.Postgres;

public interface IPostgresDriver
{
    Task Open(string url);
    Task Close();
    Task<PostgresExecuteResult> Execute(string sql, IReadOnlyDictionary<string, object?> parameters);
    Task<IReadOnlyList<IReadOnlyDictionary<string, object?>>> Query(string sql, IReadOnlyDictionary<string, object?> parameters);
    Task Begin(IsolationLevel isolation = IsolationLevel.Serializable);
    Task Commit();
    Task Rollback();
}
