using System.Text.RegularExpressions;
using Datasources;

namespace Datasources.Sqlite;

public sealed class SqliteDriver : IDriver
{
    static readonly StatementTextRenderer Statements =
        new(t => new SqliteStatementRenderer(t));
    static readonly DeterministicTypeConverter Types = new(
        new Dictionary<string, string>
        {
            ["string"] = "TEXT",
            ["character"] = "TEXT",
            ["integer"] = "INTEGER",
            ["unsignedinteger"] = "INTEGER",
            ["smallinteger"] = "INTEGER",
            ["unsignedsmallinteger"] = "INTEGER",
            ["biginteger"] = "INTEGER",
            ["unsignedbiginteger"] = "INTEGER",
            ["float"] = "REAL",
            ["decimal"] = "NUMERIC",
            ["boolean"] = "BOOLEAN",
            ["datetime"] = "TEXT",
            ["binary"] = "BLOB",
            ["uuid"] = "TEXT",
            ["reference"] = "INTEGER",
        },
        new SpecFieldRules
        {
            DialectName = "sqlite",
            StringUnlimited = "TEXT",
            StringSized = n => $"VARCHAR({n})",
        });
    static readonly DefaultValueConverter Defaults = new(new DefaultValueOptions
    {
        NewId = () =>
            "(lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))), 2) || '-' || substr('89ab', abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))), 2) || '-' || lower(hex(randomblob(6))))",
        Now = () => "(strftime('%Y-%m-%dT%H:%M:%f', 'now', 'localtime'))",
        UtcNow = () => "(strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))",
        BooleanOn = "1",
        BooleanOff = "0",
        Hex = arg => $"X'{arg}'",
        HexReverse = new Regex(@"^X'(.*)'$"),
    });

    public string QuoteLeft => "\"";
    public string QuoteRight => "\"";
    public IMapping<string, string>? TypeConverter => Types;
    public DefaultValueConverter? DefaultValueConverter => Defaults;
    public StatementTextRenderer? StatementRenderer => Statements;
    public Information? Information { get; } = new(
        [
            "Embedded SQL database for local development and tests.",
            "Full connection, command, transaction, and repository support.",
        ],
        [
            "Overrides SelectInsertedIdStatement to SELECT last_insert_rowid() AS {column}",
            "Overrides CreateTableStatement type mapping (int→INTEGER, bool→BOOLEAN, uuid/string/datetime→TEXT, decimal→NUMERIC, float→REAL, binary→BLOB)",
        ],
        []);
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
