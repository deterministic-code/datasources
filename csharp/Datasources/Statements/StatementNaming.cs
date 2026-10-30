using System.Text.RegularExpressions;

namespace Datasources;

public sealed class StatementNaming
{
    static readonly Regex Snake = new(@"^[a-z_][a-z0-9_]*$", RegexOptions.Compiled);
    static readonly PluralizerConverter Pluralizer = new();

    public Casing TableNameCasing { get; init; } = Casing.Snake;
    public Casing FieldNameCasing { get; init; } = Casing.Snake;
    public Casing ConstraintNameCasing { get; init; } = Casing.Snake;
    public Casing TriggerNameCasing { get; init; } = Casing.Snake;
    public KeywordCasing KeywordCasing { get; init; } = KeywordCasing.Upper;
    public bool PluralizeTables { get; init; } = true;
    public string Objects { get; init; } = "preserve";

    public static StatementNaming Sql { get; } = new();

    public bool IsVerbatim(string mapping) => !Snake.IsMatch(mapping);

    public string PhysicalStem(string name) =>
        IsVerbatim(name) ? name : PluralizeTables ? LastToken(name) : name;

    public string Convert(string text, Casing casing)
    {
        var words = IdentifierWords(text);
        if (words.Count == 0) return text;
        return casing switch
        {
            Casing.Camel => string.Concat(words.Select((word, i) => i == 0 ? word : Cap(word))),
            Casing.Pascal => string.Concat(words.Select(Cap)),
            Casing.Snake => string.Join("_", words),
            Casing.Kebab => string.Join("-", words),
            _ => new ChangeCasingConverter(casing).Convert(text),
        };
    }

    public string Letter(string text) => Objects switch
    {
        "upper" => text.ToUpperInvariant(),
        "lower" => text.ToLowerInvariant(),
        _ => text,
    };

    public string Keyword(string text) =>
        KeywordCasing == KeywordCasing.Lower ? text.ToLowerInvariant() : text.ToUpperInvariant();

    static List<string> IdentifierWords(string name) =>
        Regex.Replace(
            Regex.Replace(
                Regex.Replace(name, @"([a-z0-9])([A-Z])", "$1 $2"),
                @"([A-Z]+)([A-Z][a-z])", "$1 $2"),
            @"[_\-.]+", " ")
        .Trim()
        .Split((char[]?)null, StringSplitOptions.RemoveEmptyEntries)
        .Select(word => word.ToLowerInvariant())
        .ToList();

    static string Cap(string word) =>
        word.Length == 0 ? word : char.ToUpperInvariant(word[0]) + word[1..];

    static string LastToken(string name)
    {
        var at = name.LastIndexOf('_');
        return at < 0
            ? Pluralizer.Convert(name)
            : name[..(at + 1)] + Pluralizer.Convert(name[(at + 1)..]);
    }
}
