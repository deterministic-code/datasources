using System.Text.RegularExpressions;
using Datasources;

namespace Datasources.SqlServer;

public sealed class SqlServerDriver : IDriver
{
    static readonly StatementTextRenderer Statements =
        new(t => new SqlServerStatementRenderer(t), applyKeywords: ApplyKeywords);
    static readonly TypeMap Types = new(
        new Dictionary<string, string>
        {
            ["string"] = "NVARCHAR(MAX)",
            ["character"] = "NCHAR(1)",
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

    internal string QuoteLeft => "[";
    internal string QuoteRight => "]";
    internal static string Quote(string name) => $"[{name}]";
    internal static string NativeType(Column column) =>
        column.Field is { } field ? Types.Convert(field) : column.Type;
    internal static string NativeDefault(Column column) =>
        column.Default is null
            ? ""
            : column.Field is { } field
                ? Defaults.Convert(field.Type, column.Default.Value) ?? column.Default.Value
                : column.Default.Value;
    internal static string UtcNow() => Defaults.Convert("datetime", "UtcNow") ?? "";
    internal static string NativeType(RoutineParameter parameter) =>
        parameter.Type == "payload"
            ? ProcedurePayloadType
            : parameter.Field is { } field ? Types.Convert(field) : NativeSpec(parameter.Type);
    internal static string NativeSpec(string spec) =>
        spec == "payload" ? ProcedurePayloadType : Types.Convert(spec);
    public IMapping<string, string>? TypeConverter => Types;
    internal DefaultValueConverter DefaultValueConverter => Defaults;
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

    internal const string ProcedurePayloadType = "NVARCHAR(MAX)";

    internal static string ApplyKeywords(string sql, Func<string, string> keyword) =>
        SqlKeywords.Apply(sql, keyword);
}
