using Datasources;

namespace Datasources.Mysql;

public interface IMysqlDriver
{
    Task Open(string url);
    Task Close();
    Task<MysqlExecuteResult> Execute(string sql, IReadOnlyDictionary<string, Value> parameters);
    Task<IReadOnlyList<IReadOnlyDictionary<string, Value>>> Query(string sql, IReadOnlyDictionary<string, Value> parameters);
    Task Begin(IsolationLevel isolation = IsolationLevel.Serializable);
    Task Commit();
    Task Rollback();
}
