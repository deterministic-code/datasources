using Datasources;

namespace Datasources.Postgres;

internal interface IPostgresDriver
{
    Task Open(string url);
    Task Close();
    Task<PostgresExecuteResult> Execute(string sql, Row parameters);
    Task<PostgresExecuteResult> Execute(string sql, IReadOnlyList<Parameter>? parameters);
    Task<IReadOnlyList<Row>> Query(string sql, Row parameters);
    Task Begin(IsolationLevel isolation = IsolationLevel.Serializable);
    Task Commit();
    Task Rollback();
}
