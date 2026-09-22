using Datasources;

namespace Datasources.Memory;

public sealed class MemoryProvider : IDriver
{
    public IConnection Connection(IReadOnlyDictionary<string, string?>? options) => new MemoryConnection(options);
    public ICommand Command(IConnection connection) => connection.CreateCommand();
    public ITransaction Transaction(IConnection connection, IsolationLevel isolation = IsolationLevel.Serializable) =>
        new MemoryTransaction((MemoryConnection)connection, isolation);
    public IReadOnlyDictionary<string, TypeConverter<Value, Value>> Types { get; } =
        new Dictionary<string, TypeConverter<Value, Value>>();
}
