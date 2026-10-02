using Datasources;

namespace Datasources.Mysql;

public sealed class MysqlDriver : IDriver
{
    static readonly StatementTextRenderer Statements =
        new(t => new MysqlStatementRenderer(t));

    public StatementTextRenderer? StatementRenderer => Statements;
    public IConnection Connection(IReadOnlyDictionary<string, string?>? options) => new MysqlConnection(options);
    public ICommand Command(IConnection connection) =>
        new MysqlCommand((MysqlConnection)connection, Statements);
    public ITransaction Transaction(IConnection connection, IsolationLevel isolation = IsolationLevel.ReadCommitted) =>
        new MysqlTransaction((MysqlConnection)connection, isolation);
    public IReadOnlyDictionary<string, TypeConverter<Value, Value>> RowTypeConverters => MysqlRowTypeConverters.All;

    internal static readonly IReadOnlyDictionary<string, string> ColumnTypes =
        new Dictionary<string, string>
        {
            ["int"] = "INT",
            ["uuid"] = "CHAR(36)",
            ["string"] = "TEXT",
            ["datetime"] = "TEXT",
            ["bool"] = "TINYINT",
            ["decimal"] = "DECIMAL(18,4)",
            ["float"] = "DOUBLE",
            ["binary"] = "BLOB",
        };

    public string ColumnType(string name) =>
        ColumnTypes.TryGetValue(name, out var sql) ? sql : throw new DatasourceException($"Unknown mysql type \"{name}\"");
}
