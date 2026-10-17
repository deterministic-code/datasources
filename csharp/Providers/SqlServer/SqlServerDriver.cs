using System.Text.RegularExpressions;
using Datasources;

namespace Datasources.SqlServer;

public sealed class SqlServerDriver : IDriver
{
    static readonly StatementTextRenderer Statements =
        new(t => new SqlServerStatementRenderer(t));
    static readonly DeterministicTypeConverter Types = new(
        new Dictionary<string, string>
        {
            ["string"] = "NVARCHAR(MAX)",
            ["character"] = "NCHAR(1)",
            ["number"] = "INT",
            ["integer"] = "INT",
            ["unsignedinteger"] = "BIGINT",
            ["smallinteger"] = "SMALLINT",
            ["unsignedsmallinteger"] = "INT",
            ["biginteger"] = "BIGINT",
            ["unsignedbiginteger"] = "DECIMAL(20)",
            ["float"] = "FLOAT",
            ["decimal"] = "DECIMAL",
            ["boolean"] = "BIT",
            ["datetime"] = "DATETIME2",
            ["binary"] = "VARBINARY(MAX)",
            ["uuid"] = "UNIQUEIDENTIFIER",
            ["reference"] = "INT",
        },
        new SpecFieldRules
        {
            DialectName = "sqlserver",
            StringUnlimited = "NVARCHAR(MAX)",
            StringSized = n => $"NVARCHAR({n})",
            Character = "NCHAR",
            Decimal = new DecimalRule.Required("DECIMAL"),
            BinaryUnlimited = "VARBINARY(MAX)",
            BinarySized = n => $"VARBINARY({n})",
        });
    static readonly DefaultValueConverter Defaults = new(new DefaultValueOptions
    {
        NewId = () => "NEWID()",
        Now = () => "GETDATE()",
        UtcNow = () => "GETUTCDATE()",
        BooleanOn = "1",
        BooleanOff = "0",
        Hex = arg => $"0x{arg}",
        HexReverse = new Regex(@"^0x(.*)$"),
    });

    public string QuoteLeft => "[";
    public string QuoteRight => "]";
    public IMapping<string, string>? TypeConverter => Types;
    public DefaultValueConverter? DefaultValueConverter => Defaults;
    public StatementTextRenderer? StatementRenderer => Statements;
    public Information? Information { get; } = new(
        ["SQL Server driver."],
        [
            "Overrides CreateCreateStoredProcedureStatement to PROCEDURE Create{Plural}",
            "Overrides CreateReadStoredProcedureStatement to PROCEDURE Read{Plural}By{Properties}",
            "Overrides CreateUpdateStoredProcedureStatement to PROCEDURE Update{Plural}By{Properties}",
            "Overrides CreateDeleteStoredProcedureStatement to PROCEDURE Delete{Plural}By{Properties}",
            "Overrides CreateBulkCreateStoredProcedureStatement to PROCEDURE BulkCreate{Plural}",
            "Overrides CreateBulkUpdateStoredProcedureStatement to PROCEDURE BulkUpdate{Plural}",
            "Overrides CreateBulkDeleteStoredProcedureStatement to PROCEDURE BulkDelete{Plural}",
        ],
        []);
    public IConnection Connection(IReadOnlyDictionary<string, string?>? options) => new SqlServerConnection(options);
    public ICommand Command(IConnection connection) => connection.CreateCommand();
    public ITransaction Transaction(IConnection connection, IsolationLevel isolation = IsolationLevel.ReadCommitted) =>
        new StubTransaction(connection.Provider, connection);
    public IReadOnlyDictionary<string, TypeConverter<Value, Value>> RowTypeConverters { get; } =
        new Dictionary<string, TypeConverter<Value, Value>>();
}
