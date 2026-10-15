namespace Datasources;

public static class SplitStatements
{
    public static IReadOnlyList<string> Split(string sql)
    {
        var parts = new List<string>();
        var start = 0;
        char? quote = null;
        string? dollar = null;
        var line = false;
        var block = false;
        var depth = 0;
        for (var i = 0; i < sql.Length; i++)
        {
            var next = i + 1 < sql.Length ? sql[i + 1] : (char?)null;
            if (line)
            {
                if (sql[i] == '\n') line = false;
                continue;
            }
            if (block)
            {
                if (sql[i] == '*' && next == '/') { block = false; i++; }
                continue;
            }
            if (dollar is not null)
            {
                if (sql.AsSpan(i).StartsWith(dollar, StringComparison.Ordinal))
                {
                    i += dollar.Length - 1;
                    dollar = null;
                }
                continue;
            }
            if (quote is { } mark)
            {
                if (sql[i] == mark)
                {
                    if (next == mark) i++;
                    else quote = null;
                }
                continue;
            }
            if (sql[i] == '-' && next == '-') { line = true; i++; continue; }
            if (sql[i] == '/' && next == '*') { block = true; i++; continue; }
            if (DollarTag(sql, i) is { } tag)
            {
                dollar = tag;
                i += tag.Length - 1;
                continue;
            }
            if (sql[i] is '\'' or '"' or '`') { quote = sql[i]; continue; }
            if (Word(sql, i, "BEGIN") && Routine(sql, start, i)) { depth++; i += 4; continue; }
            if (sql[i] != ';') continue;
            if (depth > 0 && Routine(sql, start, i))
            {
                if (EndsWithWord(sql[start..i], "END")) depth--;
                if (depth > 0) continue;
            }
            var part = sql[start..i].Trim();
            if (part.Length > 0) parts.Add(part);
            start = i + 1;
            depth = 0;
        }
        var tail = sql[start..].Trim();
        if (tail.Length > 0) parts.Add(tail);
        return parts;
    }

    public static IReadOnlyList<string> Parameters(string sql)
    {
        var names = new List<string>();
        var seen = new HashSet<string>();
        for (var i = 0; i < sql.Length; i++)
        {
            var prefix = sql[i];
            if (prefix is not ('$' or ':' or '@')) continue;
            var end = i + 1;
            while (end < sql.Length && Id(sql[end])) end++;
            if (end == i + 1) continue;
            if (prefix == '$' && end < sql.Length && sql[end] == '$')
            {
                i = end;
                continue;
            }
            var name = sql[(i + 1)..end];
            if (seen.Add(name)) names.Add(name);
            i = end - 1;
        }
        return names;
    }

    public static bool UsesParameter(string sql, string name) =>
        Parameters(sql).Contains(name.TrimStart('$', '@', ':'));

    static string? DollarTag(string sql, int i)
    {
        if (sql[i] != '$') return null;
        var end = i + 1;
        while (end < sql.Length && (char.IsLetterOrDigit(sql[end]) || sql[end] == '_')) end++;
        return end < sql.Length && sql[end] == '$' ? sql[i..(end + 1)] : null;
    }

    static bool Word(string sql, int i, string token) =>
        sql.AsSpan(i).StartsWith(token, StringComparison.Ordinal)
        && (i == 0 || !Id(sql[i - 1]))
        && (i + token.Length == sql.Length || !Id(sql[i + token.Length]));

    static bool Routine(string sql, int start, int i)
    {
        var head = sql[start..i].ToUpperInvariant();
        return head.Contains("CREATE PROCEDURE", StringComparison.Ordinal)
            || head.Contains("CREATE FUNCTION", StringComparison.Ordinal)
            || head.Contains("CREATE TRIGGER", StringComparison.Ordinal);
    }

    static bool EndsWithWord(string sql, string token)
    {
        var trimmed = sql.TrimEnd();
        return trimmed.EndsWith(token, StringComparison.OrdinalIgnoreCase)
            && (trimmed.Length == token.Length || !Id(trimmed[trimmed.Length - token.Length - 1]));
    }

    static bool Id(char c) => char.IsLetterOrDigit(c) || c == '_';
}
