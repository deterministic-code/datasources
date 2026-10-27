using System.Text.RegularExpressions;
using Datasources;

namespace Datasources.Mysql;

public sealed class MysqlDriver : IDriver
{
    static readonly StatementTextRenderer Statements =
        new(t => new MysqlStatementRenderer(t));
    static readonly DeterministicTypeConverter Types = new(
        new Dictionary<string, string>
        {
            ["string"] = "TEXT",
            ["character"] = "CHAR(1)",
            ["number"] = "INT",
            ["integer"] = "INT",
            ["unsignedinteger"] = "INT UNSIGNED",
            ["smallinteger"] = "SMALLINT",
            ["unsignedsmallinteger"] = "SMALLINT UNSIGNED",
            ["biginteger"] = "BIGINT",
            ["unsignedbiginteger"] = "BIGINT UNSIGNED",
            ["float"] = "DOUBLE",
            ["decimal"] = "DECIMAL(10, 2)",
            ["boolean"] = "TINYINT(1)",
            ["datetime"] = "DATETIME(3)",
            ["binary"] = "BLOB",
            ["uuid"] = "CHAR(36)",
            ["reference"] = "INT",
        },
        new SpecFieldRules
        {
            DialectName = "mysql",
            StringUnlimited = "TEXT",
            StringSized = n => n <= 16383 ? $"VARCHAR({n})" : "LONGTEXT",
            Character = "CHAR",
            Decimal = new DecimalRule.Optional("DECIMAL", "DECIMAL(10, 2)"),
            BinaryUnlimited = "BLOB",
            BinarySized = n => n <= 65535 ? $"VARBINARY({n})" : "LONGBLOB",
        });
    static readonly DefaultValueConverter Defaults = new(new DefaultValueOptions
    {
        NewId = () => "(UUID())",
        Now = () => "(NOW(3))",
        UtcNow = () => "(UTC_TIMESTAMP(3))",
        BooleanOn = "1",
        BooleanOff = "0",
        Hex = arg => $"(X'{arg}')",
        HexReverse = new Regex(@"^\(?X'(.*)'\)?$"),
    });

    public string QuoteLeft => "`";
    public string QuoteRight => "`";
    public IMapping<string, string>? TypeConverter => Types;
    public DefaultValueConverter? DefaultValueConverter => Defaults;
    public StatementTextRenderer? StatementRenderer => Statements;
    public Information? Information { get; } = new(
        [
            "MySQL / MariaDB server driver.",
            "Full connection, command, transaction, and repository support.",
        ],
        [
            "Overrides SelectInsertedIdStatement to SELECT LAST_INSERT_ID() AS {column}",
            "Overrides CreateTableStatement identity to AUTO_INCREMENT",
            "Overrides CreateTableStatement type mapping (int→INT, uuid→CHAR(36), string/datetime→TEXT, bool→TINYINT, decimal→DECIMAL(18,4), float→DOUBLE, binary→BLOB)",
            "Overrides CreateCreateStoredProcedureStatement to PROCEDURE Create{Plural}",
            "Overrides CreateReadStoredProcedureStatement to PROCEDURE Read{Plural}By{Properties}",
            "Overrides CreateUpdateStoredProcedureStatement to PROCEDURE Update{Plural}By{Properties}",
            "Overrides CreateDeleteStoredProcedureStatement to PROCEDURE Delete{Plural}By{Properties}",
            "Overrides CreateBulkCreateStoredProcedureStatement to PROCEDURE BulkCreate{Plural}",
            "Overrides CreateBulkUpdateStoredProcedureStatement to PROCEDURE BulkUpdate{Plural}",
            "Overrides CreateBulkDeleteStoredProcedureStatement to PROCEDURE BulkDelete{Plural}",
        ],
        []);
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
