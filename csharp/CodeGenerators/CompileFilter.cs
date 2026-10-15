using System.Text.RegularExpressions;

namespace Datasources.CodeGenerators;

public static class CompileFilter
{
    static readonly HashSet<string> Allowed =
    [
        "__name", "__tags", "__inherits", "includes", "true", "false", "null", "undefined",
        "type", "tag", "inherits",
    ];

    public static Func<SpecType, bool> Types(string? filter) =>
        Of(filter, "types.filter");

    public static Func<SpecType, bool> Of(string? filter, string context)
    {
        if (string.IsNullOrEmpty(filter)) return _ => true;
        var placeholders = new List<string>();
        var withStrings = Regex.Replace(filter, "\"[^\"]*\"", m =>
        {
            placeholders.Add(m.Value[1..^1]);
            return $"__STR{placeholders.Count - 1}__";
        });
        var rewritten = Rewrite(withStrings);
        foreach (Match ident in Regex.Matches(rewritten, @"[A-Za-z_][A-Za-z0-9_]*"))
        {
            if (ident.Value.StartsWith("__STR", StringComparison.Ordinal) || Allowed.Contains(ident.Value))
                continue;
            throw new InvalidOperationException(
                $"{context}: unknown identifier or syntax near \"{ident.Value}\". Supported: `type == \"name\"`, `tag == \"kind\"`, `inherits == \"parent\"`, logical && / ||, parens.");
        }
        var expr = restored(rewritten, placeholders);
        return type => Eval(expr, type.Name, type.Tags, type.Inherits ?? "");
    }

    static string Rewrite(string input)
    {
        var type = Regex.Replace(input, @"\btype\s*(==|!=)\s*(__STR\d+__)", m =>
            $"( __name {(m.Groups[1].Value == "==" ? "==" : "!=")} {m.Groups[2].Value} )");
        var tagEq = Regex.Replace(type, @"\btag\s*==\s*(__STR\d+__)", m => $"__tags.includes({m.Groups[1].Value})");
        var tagNe = Regex.Replace(tagEq, @"\btag\s*!=\s*(__STR\d+__)", m => $"!__tags.includes({m.Groups[1].Value})");
        return Regex.Replace(tagNe, @"\binherits\s*(==|!=)\s*(__STR\d+__)", m =>
            $"( __inherits {(m.Groups[1].Value == "==" ? "==" : "!=")} {m.Groups[2].Value} )");
    }

    static string restored(string rewritten, List<string> placeholders) =>
        Regex.Replace(rewritten, @"__STR(\d+)__", m => $"\"{placeholders[int.Parse(m.Groups[1].Value)]}\"");

    static bool Eval(string expr, string name, IReadOnlyList<string> tags, string inherits) =>
        Or(expr.Replace(" ", ""), name, tags, inherits);

    static bool Or(string expr, string name, IReadOnlyList<string> tags, string inherits)
    {
        var parts = Split(expr, "||");
        return parts.Count == 0 || parts.Any(part => And(part, name, tags, inherits));
    }

    static bool And(string expr, string name, IReadOnlyList<string> tags, string inherits)
    {
        var parts = Split(expr, "&&");
        return parts.All(part => Atom(part, name, tags, inherits));
    }

    static bool Atom(string expr, string name, IReadOnlyList<string> tags, string inherits)
    {
        if (expr.StartsWith('!') && !expr.StartsWith("!=", StringComparison.Ordinal))
            return !Atom(expr[1..], name, tags, inherits);
        if (expr.StartsWith('(') && expr.EndsWith(')'))
            return Or(expr[1..^1], name, tags, inherits);
        var include = Regex.Match(expr, @"^__tags\.includes\(""([^""]*)""\)$");
        if (include.Success) return tags.Contains(include.Groups[1].Value);
        var notInclude = Regex.Match(expr, @"^!__tags\.includes\(""([^""]*)""\)$");
        if (notInclude.Success) return !tags.Contains(notInclude.Groups[1].Value);
        var cmp = Regex.Match(expr, @"^\(__(name|inherits)(==|!=)""([^""]*)""\)$");
        if (!cmp.Success)
            throw new InvalidOperationException($"types.filter is not a valid expression: {expr}");
        var left = cmp.Groups[1].Value == "name" ? name : inherits;
        return cmp.Groups[2].Value == "==" ? left == cmp.Groups[3].Value : left != cmp.Groups[3].Value;
    }

    static List<string> Split(string expr, string op)
    {
        var parts = new List<string>();
        var depth = 0;
        var start = 0;
        for (var i = 0; i <= expr.Length - op.Length; i++)
        {
            var c = expr[i];
            if (c == '(') depth++;
            else if (c == ')') depth--;
            else if (depth == 0 && expr.AsSpan(i, op.Length).SequenceEqual(op))
            {
                parts.Add(expr[start..i]);
                i += op.Length - 1;
                start = i + 1;
            }
        }
        parts.Add(expr[start..]);
        return parts;
    }
}
