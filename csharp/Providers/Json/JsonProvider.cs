using Datasources;

namespace Datasources.Json;

public sealed class JsonProvider : IDriver
{
    public IConnection Connection(IReadOnlyDictionary<string, string?>? options) => new JsonConnection(options);
    public ICommand Command(IConnection connection) => connection.CreateCommand();
    public ITransaction Transaction(IConnection connection, IsolationLevel isolation = IsolationLevel.Serializable) =>
        new JsonTransaction((JsonConnection)connection, isolation);
    public IReadOnlyDictionary<string, TypeConverter<Value, Value>> RowTypeConverters { get; } =
        new Dictionary<string, TypeConverter<Value, Value>>();
}
