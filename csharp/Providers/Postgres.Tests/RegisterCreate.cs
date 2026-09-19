using Datasources;
using Datasources.Postgres;

public partial class RegisterCreateTests
{
    const string Provider = "postgres";
    static IConnection Connection(IReadOnlyDictionary<string, string?>? options) => new PostgresConnection(options);
    static object Repository(IConnection connection) => PostgresConnection.Repository(connection);
}
