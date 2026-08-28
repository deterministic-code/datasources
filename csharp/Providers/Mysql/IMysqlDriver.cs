namespace Datasources.Mysql;

public interface IMysqlDriver
{
    Task Open(string url);
    Task Close();
    Task<MysqlExecuteResult> Execute(string sql, IReadOnlyDictionary<string, object?> parameters);
    Task<IReadOnlyList<IReadOnlyDictionary<string, object?>>> Query(string sql, IReadOnlyDictionary<string, object?> parameters);
    Task Begin(IsolationLevel isolation = IsolationLevel.Serializable);
    Task Commit();
    Task Rollback();
}
