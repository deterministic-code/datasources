using Datasources;
using Datasources.Memory;

public partial class RegisterCreateTests
{
    const string Provider = "memory";
    static IConnection Connection(IReadOnlyDictionary<string, string?>? options) => new MemoryConnection(options);
    static object Repository(IConnection connection) => MemoryConnection.Repository(connection);
}
