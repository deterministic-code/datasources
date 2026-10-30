using System.Text.RegularExpressions;

namespace Datasources.SqlServer;

internal sealed class DefaultValueConverter
{
    internal const string EmptyUuid = "00000000-0000-0000-0000-000000000000";
    static readonly Regex CallForm = new(@"^([A-Za-z]+)\('(.*)'\)$", RegexOptions.Compiled);
    static readonly Regex QuotedReverse = new(@"^'(.*)'$", RegexOptions.Compiled);
    static readonly string[] NumericTypes =
    [
        "integer", "unsignedinteger", "smallinteger", "unsignedsmallinteger",
        "biginteger", "unsignedbiginteger", "float", "decimal",
    ];
    static readonly HashSet<string> NumericLiteralTypes = [.. NumericTypes, "reference"];

    internal static string SqlStringLiteral(object? value) =>
        $"'{System.Convert.ToString(value)?.Replace("'", "''", StringComparison.Ordinal)}'";

    readonly IReadOnlyDictionary<string, IReadOnlyDictionary<string, DefaultEntry>> dictionary;

    public DefaultValueConverter(DefaultValueOptions options) =>
        dictionary = DictionaryOf(options);

    internal string? Convert(string specType, string text)
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

    internal string ReverseConvert(string specType, string native)
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

    internal static (string Token, string? Arg) Parse(string specType, object? value)
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

    static IReadOnlyDictionary<string, IReadOnlyDictionary<string, DefaultEntry>> DictionaryOf(
        DefaultValueOptions opts)
    {
        var newId = opts.NewId();
        var now = opts.Now();
        var utcNow = opts.UtcNow();
        var quote = opts.Quote ?? SqlStringLiteral;
        var quoteReverse = opts.QuoteReverse ?? QuotedReverse;
        var quotedString = new Dictionary<string, DefaultEntry>
        {
            ["String"] = Entry(new Regex(@"^.*$"), quoteReverse, quote),
            ["Numeric"] = Entry(new Regex(@"^.*$"), quoteReverse, quote),
        };
        var dictionary = new Dictionary<string, IReadOnlyDictionary<string, DefaultEntry>>
        {
            ["string"] = quotedString,
            ["character"] = new Dictionary<string, DefaultEntry>
            {
                ["String"] = Entry(new Regex(@"^.+$"), quoteReverse, quote),
                ["Numeric"] = Entry(new Regex(@"^.+$"), quoteReverse, quote),
            },
            ["boolean"] = new Dictionary<string, DefaultEntry>
            {
                ["Boolean"] = Entry(
                    new Regex(@"^(true|false)$"),
                    new Regex($"^({Regex.Escape(opts.BooleanOn)}|{Regex.Escape(opts.BooleanOff)})$"),
                    arg => arg == "true" ? opts.BooleanOn : opts.BooleanOff),
            },
            ["datetime"] = new Dictionary<string, DefaultEntry>
            {
                ["Now"] = Entry(new Regex(@"^Now$"), Exact(now), _ => now),
                ["UtcNow"] = Entry(new Regex(@"^UtcNow$"), Exact(utcNow), _ => utcNow),
                ["DateTime"] = Entry(
                    new Regex(@"^DateTime\('[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}(Z|[+-][0-9]{2}:[0-9]{2})'\)$"),
                    QuotedReverse,
                    SqlStringLiteral,
                    arg => $"DateTime('{arg}')"),
            },
            ["binary"] = new Dictionary<string, DefaultEntry>
            {
                ["Hex"] = Entry(
                    new Regex(@"^Hex\('([0-9a-fA-F]{2})*'\)$"),
                    opts.HexReverse,
                    opts.Hex,
                    arg => $"Hex('{arg}')"),
            },
            ["uuid"] = new Dictionary<string, DefaultEntry>
            {
                ["NewId"] = Entry(new Regex(@"^NewId$"), Exact(newId), _ => newId),
                ["Empty"] = Entry(
                    new Regex(@"^Empty$"),
                    Exact(SqlStringLiteral(EmptyUuid)),
                    _ => SqlStringLiteral(EmptyUuid)),
                ["Uuid"] = Entry(
                    new Regex(@"^uuid\('[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}'\)$"),
                    QuotedReverse,
                    SqlStringLiteral,
                    arg => $"uuid('{arg}')"),
            },
            ["reference"] = new Dictionary<string, DefaultEntry>(),
        };
        var numeric = new Regex(@"^-?[0-9]+(\.[0-9]+)?$");
        var integer = new Regex(@"^-?[0-9]+$");
        var unsigned = new Regex(@"^[0-9]+$");
        foreach (var type in NumericTypes)
        {
            var spec = type is "float" or "decimal" ? numeric : type.StartsWith("unsigned", StringComparison.Ordinal) ? unsigned : integer;
            dictionary[type] = new Dictionary<string, DefaultEntry>
            {
                ["Numeric"] = Entry(spec, spec, arg => arg),
                ["String"] = Entry(spec, spec, arg => arg),
            };
        }
        return dictionary;
    }

    static Regex Exact(string text) => new($"^{Regex.Escape(text)}$");

    static DefaultEntry Entry(
        Regex spec,
        Regex reverse,
        Func<string, string> native,
        Func<string, string>? specText = null) =>
        new(spec, reverse, native, specText);

    public sealed record DefaultEntry(
        Regex Spec,
        Regex Reverse,
        Func<string, string> Native,
        Func<string, string>? SpecText);
}
