namespace Datasources;

public sealed class MockDriver : IDriver
{
    public IConnection Connection(IReadOnlyDictionary<string, string?>? options) => new MockConnection();
    public ICommand Command(IConnection connection) => new MockCommand(connection);
    public ITransaction Transaction(IConnection connection, IsolationLevel isolation = IsolationLevel.Serializable) =>
        new MockTransaction(connection) { Isolation = isolation };
    public IReadOnlyDictionary<string, TypeConverter<Value, Value>> Types { get; } =
        new Dictionary<string, TypeConverter<Value, Value>>();
}
