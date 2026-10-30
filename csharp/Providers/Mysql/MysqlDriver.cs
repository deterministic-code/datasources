using System.Text.RegularExpressions;
using Datasources;

namespace Datasources.Mysql;

public sealed class MysqlDriver : IDriver
{
    static readonly StatementTextRenderer Statements =
        new(t => new MysqlStatementRenderer(t), applyKeywords: ApplyKeywords);
    static readonly TypeMap Types = new(
        new Dictionary<string, string>
        {
            ["string"] = "TEXT",
            ["character"] = "CHAR(1)",
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
        });
    static readonly Defaults DefaultsMap = new();

    internal string QuoteLeft => "`";
    internal string QuoteRight => "`";
    internal static string Quote(string name) => $"`{name}`";
    internal static string NativeType(Column column) =>
        column.Field is { } field
            ? Types.Convert(field)
            : ColumnTypes.TryGetValue(column.Type, out var sql) ? sql : column.Type;
    internal static string NativeDefault(Column column) =>
        column.Default is null
            ? ""
            : column.Field is { } field
                ? DefaultsMap.Convert(field.Type, column.Default.Value) ?? column.Default.Value
                : column.Default.Value;
    internal static string UtcNow() => DefaultsMap.Convert("datetime", "UtcNow") ?? "";
    internal static string NativeType(RoutineParameter parameter) =>
        parameter.Type == "payload"
            ? ProcedurePayloadType
            : parameter.Field is { } field ? Types.Convert(field) : NativeSpec(parameter.Type);
    internal static string NativeSpec(string spec) =>
        spec == "payload" ? ProcedurePayloadType : Types.Convert(spec);
    public IMapping<string, string>? TypeConverter => Types;
    internal Defaults DefaultValueConverter => DefaultsMap;
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

    internal const string ProcedurePayloadType = "JSON";

    internal static string ApplyKeywords(string sql, Func<string, string> keyword) =>
        SqlKeywords.Apply(sql, keyword);

    internal sealed class TypeMap : IMapping<string, string>
    {
        readonly IReadOnlyDictionary<string, string> dictionary;
        readonly Dictionary<string, string> reverse = [];

        public TypeMap(IReadOnlyDictionary<string, string> dictionary)
        {
            this.dictionary = dictionary;
            foreach (var (specType, native) in dictionary)
                reverse.TryAdd(native, specType);
        }

        public string Convert(string from) =>
            dictionary.TryGetValue(from, out var native)
                ? native
                : throw new InvalidOperationException($"Unknown spec field type: {from}");

        public string ReverseConvert(string native) =>
            reverse.TryGetValue(native, out var spec)
                ? spec
                : throw new InvalidOperationException($"Unknown native field type: {native}");

        public string Convert(SpecField field)
        {
            if (field.Type == "string")
                return field.Size is null or "unlimited"
                    ? "TEXT"
                    : System.Convert.ToInt32(field.Size) <= 16383
                        ? $"VARCHAR({field.Size})"
                        : "LONGTEXT";
            if (field.Type == "character")
                return $"CHAR({(field.Size is null ? 1 : System.Convert.ToInt32(field.Size))})";
            if (field.Type == "decimal")
            {
                if (field.Size is null) return "DECIMAL(10, 2)";
                if (field.Size is int[] { Length: 2 } pair) return $"DECIMAL({pair[0]}, {pair[1]})";
                if (field.Size is IReadOnlyList<int> { Count: 2 } list) return $"DECIMAL({list[0]}, {list[1]})";
                throw new InvalidOperationException(
                    $"decimal field \"{field.Name}\" size must be [precision, scale] tuple; got {field.Size}");
            }
            if (field.Type == "binary")
                return field.Size is null or "unlimited"
                    ? "BLOB"
                    : System.Convert.ToInt32(field.Size) <= 65535
                        ? $"VARBINARY({field.Size})"
                        : "LONGBLOB";
            return Convert(field.Type);
        }
    }

    internal sealed class Defaults
    {
        const string EmptyUuid = "00000000-0000-0000-0000-000000000000";
        static readonly Regex CallForm = new(@"^([A-Za-z]+)\('(.*)'\)$", RegexOptions.Compiled);
        static readonly string[] NumericTypes =
        [
            "integer", "unsignedinteger", "smallinteger", "unsignedsmallinteger",
            "biginteger", "unsignedbiginteger", "float", "decimal",
        ];
        static readonly HashSet<string> NumericLiteralTypes = [.. NumericTypes, "reference"];
        static string SqlStringLiteral(object? value) =>
            $"'{System.Convert.ToString(value)?.Replace("'", "''", StringComparison.Ordinal)}'";
        static string Quote(string arg) => $"({SqlStringLiteral(arg)})";
        readonly IReadOnlyDictionary<string, IReadOnlyDictionary<string, Entry>> dictionary = DictionaryOf();

        public string? Convert(string specType, string text)
        {
            var parsed = Parse(specType, text);
            if (parsed.Token == "None") return null;
            if (!dictionary.TryGetValue(specType, out var entries))
                throw new InvalidOperationException($"Unknown spec field type: {specType}");
            if (!entries.TryGetValue(parsed.Token, out var entry)) return null;
            if (!entry.Spec.IsMatch(text))
                throw new InvalidOperationException(
                    $"default_value '{text}' is not a valid {specType} {parsed.Token} default");
            var expr = entry.Native(parsed.Arg ?? "");
            return expr == "" ? null : expr;
        }

        public string ReverseConvert(string specType, string native)
        {
            if (!dictionary.TryGetValue(specType, out var entries))
                throw new InvalidOperationException($"Unknown spec field type: {specType}");
            foreach (var (name, entry) in entries)
            {
                var match = entry.Reverse.Match(native);
                if (!match.Success) continue;
                var arg = match.Groups.Cast<Group>().Skip(1).FirstOrDefault(g => g.Success)?.Value ?? "";
                return entry.SpecText is null ? name : entry.SpecText(arg);
            }
            throw new InvalidOperationException($"Native default '{native}' is not a valid {specType} default");
        }

        static (string Token, string? Arg) Parse(string specType, object? value)
        {
            if (value is null) return ("None", null);
            var v = System.Convert.ToString(value) ?? "";
            var call = CallForm.Match(v);
            if (call.Success) return (call.Groups[1].Value == "uuid" ? "Uuid" : call.Groups[1].Value, call.Groups[2].Value);
            if (specType == "boolean")
                return ("Boolean", value is bool flag ? flag.ToString().ToLowerInvariant() : v == "true" ? "true" : "false");
            if (NumericLiteralTypes.Contains(specType)) return ("Numeric", v);
            if (specType == "datetime")
                return v is "Now" or "UtcNow" ? (v, null) : ("DateTime", v);
            if (specType == "uuid")
                return v is "NewId" or "Empty" ? (v, null) : ("Uuid", v);
            if (specType == "binary") return ("Hex", v);
            return ("String", v);
        }

        static IReadOnlyDictionary<string, IReadOnlyDictionary<string, Entry>> DictionaryOf()
        {
            const string newId = "(UUID())";
            const string now = "(NOW(3))";
            const string utcNow = "(UTC_TIMESTAMP(3))";
            var quoteReverse = new Regex(@"^(?:\('(.*)'\)|'(.*)')$");
            var quoted = new Dictionary<string, Entry>
            {
                ["String"] = new(new Regex(@"^.*$"), quoteReverse, Quote),
                ["Numeric"] = new(new Regex(@"^.*$"), quoteReverse, Quote),
            };
            var dictionary = new Dictionary<string, IReadOnlyDictionary<string, Entry>>
            {
                ["string"] = quoted,
                ["character"] = new Dictionary<string, Entry>
                {
                    ["String"] = new(new Regex(@"^.+$"), quoteReverse, Quote),
                    ["Numeric"] = new(new Regex(@"^.+$"), quoteReverse, Quote),
                },
                ["boolean"] = new Dictionary<string, Entry>
                {
                    ["Boolean"] = new(
                        new Regex(@"^(true|false)$"),
                        new Regex("^(1|0)$"),
                        arg => arg == "true" ? "1" : "0"),
                },
                ["datetime"] = new Dictionary<string, Entry>
                {
                    ["Now"] = new(new Regex(@"^Now$"), Exact(now), _ => now),
                    ["UtcNow"] = new(new Regex(@"^UtcNow$"), Exact(utcNow), _ => utcNow),
                    ["DateTime"] = new(
                        new Regex(@"^DateTime\('[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}(Z|[+-][0-9]{2}:[0-9]{2})'\)$"),
                        new Regex(@"^'(.*)'$"),
                        SqlStringLiteral,
                        arg => $"DateTime('{arg}')"),
                },
                ["binary"] = new Dictionary<string, Entry>
                {
                    ["Hex"] = new(
                        new Regex(@"^Hex\('([0-9a-fA-F]{2})*'\)$"),
                        new Regex(@"^(?:\(X'(.*)'\)|X'(.*)')$"),
                        arg => $"(X'{arg}')",
                        arg => $"Hex('{arg}')"),
                },
                ["uuid"] = new Dictionary<string, Entry>
                {
                    ["NewId"] = new(new Regex(@"^NewId$"), Exact(newId), _ => newId),
                    ["Empty"] = new(
                        new Regex(@"^Empty$"),
                        Exact(SqlStringLiteral(EmptyUuid)),
                        _ => SqlStringLiteral(EmptyUuid)),
                    ["Uuid"] = new(
                        new Regex(@"^uuid\('[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}'\)$"),
                        new Regex(@"^'(.*)'$"),
                        SqlStringLiteral,
                        arg => $"uuid('{arg}')"),
                },
                ["reference"] = new Dictionary<string, Entry>(),
            };
            var numeric = new Regex(@"^-?[0-9]+(\.[0-9]+)?$");
            var integer = new Regex(@"^-?[0-9]+$");
            var unsigned = new Regex(@"^[0-9]+$");
            foreach (var type in NumericTypes)
            {
                var spec = type is "float" or "decimal" ? numeric : type.StartsWith("unsigned", StringComparison.Ordinal) ? unsigned : integer;
                dictionary[type] = new Dictionary<string, Entry>
                {
                    ["Numeric"] = new(spec, spec, arg => arg),
                    ["String"] = new(spec, spec, arg => arg),
                };
            }
            return dictionary;
        }

        static Regex Exact(string text) => new($"^{Regex.Escape(text)}$");

        internal sealed record Entry(
            Regex Spec,
            Regex Reverse,
            Func<string, string> Native,
            Func<string, string>? SpecText = null);
    }
}
