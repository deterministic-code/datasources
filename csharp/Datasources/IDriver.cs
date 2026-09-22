namespace Datasources;

public interface IDriver
{
    IConnection Connection(IReadOnlyDictionary<string, string?>? options);
    ICommand Command(IConnection connection);
    ITransaction Transaction(IConnection connection, IsolationLevel isolation = IsolationLevel.Serializable);
    IReadOnlyDictionary<string, TypeConverter<Value, Value>> Types { get; }
}
