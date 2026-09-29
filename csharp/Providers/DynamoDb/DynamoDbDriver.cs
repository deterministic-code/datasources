using Datasources;

namespace Datasources.DynamoDb;

public sealed class DynamoDbDriver : IDriver
{
    public IConnection Connection(IReadOnlyDictionary<string, string?>? options) => new DynamoDbConnection(options);
    public ICommand Command(IConnection connection) => connection.CreateCommand();
    public ITransaction Transaction(IConnection connection, IsolationLevel isolation = IsolationLevel.ReadCommitted) =>
        new StubTransaction(connection.Provider, connection);
    public IReadOnlyDictionary<string, TypeConverter<Value, Value>> RowTypeConverters { get; } =
        new Dictionary<string, TypeConverter<Value, Value>>();
}
