namespace Datasources.Sqlite;

public interface ISqliteDriver
{
    Task Open(string filename);
    Task Close();
    Task<SqliteExecuteResult> Execute(string sql, IReadOnlyDictionary<string, object?> parameters);
    Task<IReadOnlyList<IReadOnlyDictionary<string, object?>>> Query(string sql, IReadOnlyDictionary<string, object?> parameters);
    Task Begin();
    Task Commit();
    Task Rollback();
}
