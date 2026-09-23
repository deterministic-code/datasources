using Datasources;

namespace Datasources.Postgres;

public sealed class PostgresDriver : IDriver
{
    public IConnection Connection(IReadOnlyDictionary<string, string?>? options) => new PostgresConnection(options);
    public ICommand Command(IConnection connection) => connection.CreateCommand();
    public ITransaction Transaction(IConnection connection, IsolationLevel isolation = IsolationLevel.Serializable) =>
        new PostgresTransaction((PostgresConnection)connection, isolation);
    public IReadOnlyDictionary<string, TypeConverter<Value, Value>> RowTypeConverters => PostgresRowTypeConverters.All;
}
