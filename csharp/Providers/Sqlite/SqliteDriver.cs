using Datasources;

namespace Datasources.Sqlite;

public sealed class SqliteDriver : IDriver
{
    public IConnection Connection(IReadOnlyDictionary<string, string?>? options) => new SqliteConnection(options);
    public ICommand Command(IConnection connection) => connection.CreateCommand();
    public ITransaction Transaction(IConnection connection, IsolationLevel isolation = IsolationLevel.Serializable) =>
        new SqliteTransaction((SqliteConnection)connection, isolation);
    public IReadOnlyDictionary<string, TypeConverter<Value, Value>> RowTypeConverters => SqliteRowTypeConverters.All;

    public string ColumnType(string name) => name switch
    {
        "int" or "bool" => "INTEGER",
        "uuid" or "string" or "datetime" => "TEXT",
        "decimal" => "NUMERIC",
        "float" => "REAL",
        "binary" => "BLOB",
        _ => throw new DatasourceException($"Unknown sqlite type \"{name}\"")
    };
}
