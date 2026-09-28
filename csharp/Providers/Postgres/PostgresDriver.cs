using Datasources;

namespace Datasources.Postgres;

public sealed class PostgresDriver : IDriver
{
    public IConnection Connection(IReadOnlyDictionary<string, string?>? options) => new PostgresConnection(options);
    public ICommand Command(IConnection connection) => connection.CreateCommand();
    public ITransaction Transaction(IConnection connection, IsolationLevel isolation = IsolationLevel.Serializable) =>
        new PostgresTransaction((PostgresConnection)connection, isolation);
    public IReadOnlyDictionary<string, TypeConverter<Value, Value>> RowTypeConverters => PostgresRowTypeConverters.All;

    public string ColumnType(string name) => name switch
    {
        "int" => "INTEGER",
        "uuid" => "UUID",
        "string" => "TEXT",
        "bool" => "BOOLEAN",
        "decimal" => "NUMERIC",
        "float" => "DOUBLE PRECISION",
        "datetime" => "TIMESTAMPTZ",
        "binary" => "BYTEA",
        _ => throw new DatasourceException($"Unknown postgres type \"{name}\"")
    };
}
