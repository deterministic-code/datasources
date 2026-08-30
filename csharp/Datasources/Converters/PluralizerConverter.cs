namespace Datasources;

public sealed class PluralizerConverter : TypeConverter<string, string>
{
    static readonly HashSet<string> Uncountable = ["sheep", "fish", "series", "species", "deer", "news"];
    static readonly Dictionary<string, string> Irregular = new()
    {
        ["person"] = "people",
        ["child"] = "children",
        ["man"] = "men",
        ["woman"] = "women",
        ["mouse"] = "mice",
        ["goose"] = "geese",
        ["tooth"] = "teeth",
        ["foot"] = "feet",
        ["ox"] = "oxen",
    };
    static readonly Dictionary<string, string> IrregularSingular =
        Irregular.ToDictionary(pair => pair.Value, pair => pair.Key);

    public override string Convert(string from) => Apply(from, Plural);
    public override string ReverseConvert(string to) => Apply(to, Singular);

    static string Apply(string word, Func<string, string> transform) =>
        word.Length == 0 ? word : Restore(word, transform(word.ToLowerInvariant()));

    static string Restore(string original, string result) =>
        char.IsUpper(original[0]) ? char.ToUpperInvariant(result[0]) + result[1..] : result;

    static string Plural(string word) => word switch
    {
        _ when Uncountable.Contains(word) || IrregularSingular.ContainsKey(word) => word,
        _ when Irregular.TryGetValue(word, out var plural) => plural,
        _ when Yies(word) || SibilantEs(word) => word,
        _ when Sibilant(word) => word + "es",
        _ when ConsonantY(word) => word[..^1] + "ies",
        _ when word.EndsWith('s') => word,
        _ => word + "s"
    };

    static string Singular(string word) => word switch
    {
        _ when Uncountable.Contains(word) || Irregular.ContainsKey(word) => word,
        _ when IrregularSingular.TryGetValue(word, out var singular) => singular,
        _ when Yies(word) => word[..^3] + "y",
        _ when SibilantEs(word) => word[..^2],
        _ when word.EndsWith('s') && !Sibilant(word) => word[..^1],
        _ => word
    };

    static bool Yies(string word) =>
        word.Length >= 4 && word.EndsWith("ies") && Consonant(word[^4]);

    static bool ConsonantY(string word) =>
        word.Length >= 2 && word.EndsWith('y') && Consonant(word[^2]);

    static bool SibilantEs(string word) =>
        word.Length > 2 && word.EndsWith("es") && Sibilant(word[..^2]);

    static bool Sibilant(string word) =>
        word.EndsWith("ch") || word.EndsWith("sh") || word.EndsWith('x') || word.EndsWith('z')
        || word.EndsWith("ss")
        || (word.Length >= 2 && word.EndsWith('s') && !word.EndsWith("es") && Vowel(word[^2]));

    static bool Vowel(char c) => "aeiou".Contains(c);
    static bool Consonant(char c) => char.IsLetter(c) && !Vowel(c);
}
