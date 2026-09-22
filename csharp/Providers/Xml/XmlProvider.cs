using Datasources;

namespace Datasources.Xml;

public sealed class XmlProvider : IDriver
{
    public IConnection Connection(IReadOnlyDictionary<string, string?>? options) => new XmlConnection(options);
    public ICommand Command(IConnection connection) => connection.CreateCommand();
    public ITransaction Transaction(IConnection connection, IsolationLevel isolation = IsolationLevel.Serializable) =>
        new StubTransaction(connection.Provider, connection);
    public IReadOnlyDictionary<string, TypeConverter<Value, Value>> Types { get; } =
        new Dictionary<string, TypeConverter<Value, Value>>();
}
