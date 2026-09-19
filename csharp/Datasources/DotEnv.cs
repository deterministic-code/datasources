namespace Datasources;

static class DotEnv
{
    public static IReadOnlyDictionary<string, string?> Load(string path = ".env") =>
        File.Exists(path) ? Parse(File.ReadAllText(path)) : new Dictionary<string, string?>();

    public static IReadOnlyDictionary<string, string?> Parse(string text)
    {
        var options = new Dictionary<string, string?>();
        foreach (var raw in text.Split(['\r', '\n'], StringSplitOptions.RemoveEmptyEntries))
        {
            var line = raw.TrimStart();
            if (line.Length == 0 || line.StartsWith('#') || !line.Contains('=')) continue;
            var i = line.IndexOf('=');
            var key = line[..i].Trim();
            if (key.Length > 0)
                options[key] = line[(i + 1)..];
        }
        return options;
    }
}
