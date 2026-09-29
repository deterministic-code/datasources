using Datasources;

namespace Datasources.Yaml;

public sealed class YamlDriver : IDriver
{
    public IConnection Connection(IReadOnlyDictionary<string, string?>? options) => new YamlConnection(options);
    public ICommand Command(IConnection connection) => connection.CreateCommand();
    public ITransaction Transaction(IConnection connection, IsolationLevel isolation = IsolationLevel.Serializable) =>
        new StubTransaction(connection.Provider, connection);
    public IReadOnlyDictionary<string, TypeConverter<Value, Value>> RowTypeConverters { get; } =
        new Dictionary<string, TypeConverter<Value, Value>>();
}
