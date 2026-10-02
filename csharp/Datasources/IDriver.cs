namespace Datasources;

public interface IDriver
{
    IConnection Connection(IReadOnlyDictionary<string, string?>? options);
    ICommand Command(IConnection connection);
    ITransaction Transaction(IConnection connection, IsolationLevel isolation = IsolationLevel.ReadCommitted);
    IReadOnlyDictionary<string, TypeConverter<Value, Value>> RowTypeConverters { get; }
    string ColumnType(string name) => name;
    StatementTextRenderer? StatementRenderer => null;
    Information? Information => null;
}
