using Datasources;

namespace Datasources.Sqlite;

public sealed class SqliteProvider : IDriver
{
    public IConnection Connection(IReadOnlyDictionary<string, string?>? options) => new SqliteConnection(options);
    public ICommand Command(IConnection connection) => connection.CreateCommand();
    public ITransaction Transaction(IConnection connection, IsolationLevel isolation = IsolationLevel.Serializable) =>
        new SqliteTransaction((SqliteConnection)connection, isolation);
    public IReadOnlyDictionary<string, TypeConverter<Value, Value>> RowTypeConverters => SqliteRowTypeConverters.All;
}
