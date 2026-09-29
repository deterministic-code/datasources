using Datasources;

namespace Datasources.SqlServer;

public sealed class SqlServerDriver : IDriver
{
    public IConnection Connection(IReadOnlyDictionary<string, string?>? options) => new SqlServerConnection(options);
    public ICommand Command(IConnection connection) => connection.CreateCommand();
    public ITransaction Transaction(IConnection connection, IsolationLevel isolation = IsolationLevel.Serializable) =>
        new StubTransaction(connection.Provider, connection);
    public IReadOnlyDictionary<string, TypeConverter<Value, Value>> RowTypeConverters { get; } =
        new Dictionary<string, TypeConverter<Value, Value>>();
}
