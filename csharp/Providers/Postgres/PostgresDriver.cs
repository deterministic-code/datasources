using Datasources;

namespace Datasources.Postgres;

public sealed class PostgresDriver : IDriver
{
    public IConnection Connection(IReadOnlyDictionary<string, string?>? options) => new PostgresConnection(options);
    public ICommand Command(IConnection connection) => connection.CreateCommand();
    public ITransaction Transaction(IConnection connection, IsolationLevel isolation = IsolationLevel.Serializable) =>
        new PostgresTransaction((PostgresConnection)connection, isolation);
    public IReadOnlyDictionary<string, TypeConverter<Value, Value>> RowTypeConverters => PostgresRowTypeConverters.All;

    internal static readonly IReadOnlyDictionary<string, string> ColumnTypes =
        new Dictionary<string, string>
        {
            ["int"] = "INTEGER",
            ["uuid"] = "UUID",
            ["string"] = "TEXT",
            ["bool"] = "BOOLEAN",
            ["decimal"] = "NUMERIC",
            ["float"] = "DOUBLE PRECISION",
            ["datetime"] = "TIMESTAMPTZ",
            ["binary"] = "BYTEA",
        };

    public string ColumnType(string name) =>
        ColumnTypes.TryGetValue(name, out var sql) ? sql : throw new DatasourceException($"Unknown postgres type \"{name}\"");
}
