using Datasources;

namespace Datasources.Mysql;

public sealed class MysqlProvider : IDriver
{
    public IConnection Connection(IReadOnlyDictionary<string, string?>? options) => new MysqlConnection(options);
    public ICommand Command(IConnection connection) => connection.CreateCommand();
    public ITransaction Transaction(IConnection connection, IsolationLevel isolation = IsolationLevel.Serializable) =>
        new MysqlTransaction((MysqlConnection)connection, isolation);
    public IReadOnlyDictionary<string, TypeConverter<Value, Value>> RowTypeConverters => MysqlRowTypeConverters.All;
}
