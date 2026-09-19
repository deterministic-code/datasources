using Datasources;
using Datasources.Mysql;

public partial class RegisterCreateTests
{
    const string Provider = "mysql";
    static IConnection Connection(IReadOnlyDictionary<string, string?>? options) => new MysqlConnection(options);
    static object Repository(IConnection connection) => MysqlConnection.Repository(connection);
}
