namespace Datasources.CodeGenerators;

public sealed class CasingStrategy
{
    static readonly string[] Formats = ["Camel", "Pascal", "Snake", "Kebab", "Auto"];
    static readonly Dictionary<string, string> ParseCase = new(StringComparer.OrdinalIgnoreCase)
    {
        ["camel"] = "Camel",
        ["pascal"] = "Pascal",
        ["snake"] = "Snake",
        ["kebab"] = "Kebab",
        ["auto"] = "Auto",
    };

    readonly string fileNames;
    readonly string types;
    readonly string fields;
    readonly string directories;

    CasingStrategy(string fileNames, string types, string fields, string directories) =>
        (this.fileNames, this.types, this.fields, this.directories) = (fileNames, types, fields, directories);

    public string ConvertFileName(string text) => ToCase(text, fileNames);
    public string ConvertTypes(string text) => ToCase(text, types);
    public string ConvertFields(string text) => ToCase(text, fields);
    public string ConvertDirectories(string text) => ToCase(text, directories);

    public static CasingStrategy Sql(IReadOnlyDictionary<string, string> settings, string prefix)
    {
        const string files = "Snake", type = "Snake", field = "Snake", dirs = "Snake";
        return new(
            Leaf(Parse(Get(settings, $"{prefix}.file_names"), $"{prefix}.file_names"), files),
            Leaf(Parse(Get(settings, $"{prefix}.types"), $"{prefix}.types"), type),
            Leaf(Parse(Get(settings, $"{prefix}.fields"), $"{prefix}.fields"), field),
            Leaf(Parse(Get(settings, $"{prefix}.directories"), $"{prefix}.directories"), dirs));
    }

    static string Leaf(string overrideFormat, string fallback) => overrideFormat == "Auto" ? fallback : overrideFormat;

    static string Parse(string? raw, string path)
    {
        if (string.IsNullOrEmpty(raw)) return "Auto";
        return ParseCase.TryGetValue(raw, out var parsed)
            ? parsed
            : throw new InvalidOperationException($"{path} must be one of [{string.Join(", ", Formats)}] (got \"{raw}\").");
    }

    static string? Get(IReadOnlyDictionary<string, string> settings, string key) =>
        settings.TryGetValue(key, out var value) ? value : null;

    static string ToCase(string name, string format)
    {
        var words = Words(name);
        if (words.Count == 0) return name;
        return format switch
        {
            "Camel" => string.Concat(words.Select((w, i) => i == 0 ? w : Cap(w))),
            "Pascal" => string.Concat(words.Select(Cap)),
            "Snake" => string.Join("_", words),
            "Kebab" => string.Join("-", words),
            _ => name,
        };
    }

    static List<string> Words(string name) =>
        System.Text.RegularExpressions.Regex.Replace(
            System.Text.RegularExpressions.Regex.Replace(
                System.Text.RegularExpressions.Regex.Replace(name, @"([a-z0-9])([A-Z])", "$1 $2"),
                @"([A-Z]+)([A-Z][a-z])", "$1 $2"),
            @"[_\-.]+", " ")
        .Trim()
        .Split((char[]?)null, StringSplitOptions.RemoveEmptyEntries)
        .Select(w => w.ToLowerInvariant())
        .ToList();

    static string Cap(string word) => word.Length == 0 ? word : char.ToUpperInvariant(word[0]) + word[1..];
}
