using Datasources;
using Datasources.Json;

public partial class RegisterCreateTests
{
    const string Provider = "json";
    static IConnection Connection(IReadOnlyDictionary<string, string?>? options) => new JsonConnection(options);
    static object Repository(IConnection connection) => JsonConnection.Repository(connection);
}
