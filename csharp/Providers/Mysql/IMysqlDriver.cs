using Datasources;

namespace Datasources.Mysql;

public interface IMysqlDriver
{
    Task Open(string url);
    Task Close();
    Task<MysqlExecuteResult> Execute(string sql, Row parameters);
    Task<IReadOnlyList<Row>> Query(string sql, Row parameters);
    Task Begin(IsolationLevel isolation = IsolationLevel.Serializable);
    Task Commit();
    Task Rollback();
}
