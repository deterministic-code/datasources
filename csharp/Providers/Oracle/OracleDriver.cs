using System.Text.RegularExpressions;
using Datasources;

namespace Datasources.Oracle;

public sealed class OracleDriver : IDriver
{
    static readonly StatementTextRenderer Statements =
        new(t => new OracleStatementRenderer(t));
    static readonly DeterministicTypeConverter Types = new(
        new Dictionary<string, string>
        {
            ["string"] = "CLOB",
            ["character"] = "CHAR(1)",
            ["integer"] = "NUMBER(10)",
            ["unsignedinteger"] = "NUMBER(10)",
            ["smallinteger"] = "NUMBER(5)",
            ["unsignedsmallinteger"] = "NUMBER(5)",
            ["biginteger"] = "NUMBER(19)",
            ["unsignedbiginteger"] = "NUMBER(20)",
            ["float"] = "BINARY_DOUBLE",
            ["decimal"] = "NUMBER",
            ["boolean"] = "NUMBER(1)",
            ["datetime"] = "TIMESTAMP",
            ["binary"] = "BLOB",
            ["uuid"] = "VARCHAR2(36)",
            ["reference"] = "NUMBER(10)",
        },
        new SpecFieldRules
        {
            DialectName = "oracle",
            StringUnlimited = "CLOB",
            StringSized = n => $"VARCHAR2({n})",
            Character = "CHAR",
            Decimal = new DecimalRule.Required("NUMBER"),
            BinaryUnlimited = "BLOB",
            BinarySized = n => $"RAW({n})",
        });
    static readonly DefaultValueConverter Defaults = new(new DefaultValueOptions
    {
        NewId = () => "SYS_GUID()",
        Now = () => "LOCALTIMESTAMP",
        UtcNow = () => "SYS_EXTRACT_UTC(SYSTIMESTAMP)",
        BooleanOn = "1",
        BooleanOff = "0",
        Hex = arg => $"HEXTORAW('{arg}')",
        HexReverse = new Regex(@"^HEXTORAW\('(.*)'\)$"),
    });

    public string QuoteLeft => "\"";
    public string QuoteRight => "\"";
    public IMapping<string, string>? TypeConverter => Types;
    public DefaultValueConverter? DefaultValueConverter => Defaults;
    public StatementTextRenderer? StatementRenderer => Statements;
    public Information? Information { get; } = new(
        ["Oracle driver."],
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
    public IConnection Connection(IReadOnlyDictionary<string, string?>? options) => new OracleConnection(options);
    public ICommand Command(IConnection connection) => connection.CreateCommand();
    public ITransaction Transaction(IConnection connection, IsolationLevel isolation = IsolationLevel.ReadCommitted) =>
        new StubTransaction(connection.Provider, connection);
    public IReadOnlyDictionary<string, TypeConverter<Value, Value>> RowTypeConverters { get; } =
        new Dictionary<string, TypeConverter<Value, Value>>();
}
