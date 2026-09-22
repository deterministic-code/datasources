namespace Datasources;

static class DotEnv
{
    public static IReadOnlyDictionary<string, string?> Load(string path = ".env") =>
        File.Exists(path) ? Parse(File.ReadAllText(path)) : new Dictionary<string, string?>();

    public static IReadOnlyDictionary<string, string?> Parse(string text) =>
        text.Split(['\r', '\n'], StringSplitOptions.RemoveEmptyEntries)
            .Select(raw => raw.TrimStart().Split('=', 2))
            .Where(parts => parts is [var key, _] && !key.StartsWith('#') && key.Trim() is { Length: > 0 })
            .GroupBy(parts => parts[0].Trim(), parts => parts[1])
            .ToDictionary(g => g.Key, g => (string?)g.Last());
}
