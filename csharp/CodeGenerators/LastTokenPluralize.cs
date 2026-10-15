using System.Text.RegularExpressions;

namespace Datasources.CodeGenerators;

public static class LastTokenPluralize
{
    static readonly Dictionary<string, string> Irregular = new(StringComparer.OrdinalIgnoreCase)
    {
        ["person"] = "people",
        ["child"] = "children",
        ["man"] = "men",
        ["woman"] = "women",
        ["mouse"] = "mice",
        ["goose"] = "geese",
        ["foot"] = "feet",
        ["tooth"] = "teeth",
    };

    static readonly HashSet<string> Uncountable = new(StringComparer.OrdinalIgnoreCase)
    {
        "settings", "preferences", "news", "information", "series", "species", "equipment",
    };

    static readonly Regex Last = new(@"[^_]+$", RegexOptions.Compiled);

    public static string Of(string name) =>
        string.IsNullOrEmpty(name) ? name : Last.Replace(name, m => Token(m.Value));

    static string Token(string token)
    {
        if (Uncountable.Contains(token) || Irregular.ContainsValue(token)) return token;
        if (Irregular.TryGetValue(token, out var irregular)) return Match(token, irregular);
        if (token.Length > 1 && token.EndsWith('y') && !"aeiou".Contains(char.ToLowerInvariant(token[^2])))
            return token[..^1] + "ies";
        if (Ends(token, "s", "x", "z", "ch", "sh")) return token + "es";
        return token + "s";
    }

    static bool Ends(string token, params string[] suffixes) =>
        suffixes.Any(s => token.EndsWith(s, StringComparison.OrdinalIgnoreCase));

    static string Match(string source, string plural) =>
        char.IsUpper(source[0]) ? char.ToUpperInvariant(plural[0]) + plural[1..] : plural;
}
