using Datasources;

namespace Datasources.Sqlite;

public sealed class SqliteDriver : IDriver
{
    static readonly StatementTextRenderer Statements =
        new(t => new SqliteStatementRenderer(t));

    public StatementTextRenderer? StatementRenderer => Statements;
    public IConnection Connection(IReadOnlyDictionary<string, string?>? options) => new SqliteConnection(options);
    public ICommand Command(IConnection connection) =>
        new SqliteCommand((SqliteConnection)connection, Statements);
    public ITransaction Transaction(IConnection connection, IsolationLevel isolation = IsolationLevel.ReadCommitted) =>
        new SqliteTransaction((SqliteConnection)connection, isolation);
    public IReadOnlyDictionary<string, TypeConverter<Value, Value>> RowTypeConverters => SqliteRowTypeConverters.All;

    internal static readonly IReadOnlyDictionary<string, string> ColumnTypes =
        new Dictionary<string, string>
        {
            ["int"] = "INTEGER",
            ["bool"] = "BOOLEAN",
            ["uuid"] = "TEXT",
            ["string"] = "TEXT",
            ["datetime"] = "TEXT",
            ["decimal"] = "NUMERIC",
            ["float"] = "REAL",
            ["binary"] = "BLOB",
        };

    public string ColumnType(string name) =>
        ColumnTypes.TryGetValue(name, out var sql) ? sql : throw new DatasourceException($"Unknown sqlite type \"{name}\"");
}
