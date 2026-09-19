using Datasources;
using Datasources.Sqlite;

public partial class RegisterCreateTests
{
    const string Provider = "sqlite";
    static IConnection Connection(IReadOnlyDictionary<string, string?>? options) => new SqliteConnection(options);
    static object Repository(IConnection connection) => SqliteConnection.Repository(connection);
}
