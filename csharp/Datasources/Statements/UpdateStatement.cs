namespace Datasources;

public sealed record UpdateStatement : IUpdateStatement
{
    readonly string text;

    public UpdateStatement(string text, IWhere? where = null, IReadOnlyList<Parameter>? parameters = null)
    {
        this.text = text;
        Where = where;
        Parameters = parameters ?? [];
    }

    public UpdateStatement(string table, IReadOnlyList<Parameter> fields, IWhere? where = null)
    {
        Where = where;
        (text, Parameters) = Bind(
            $"UPDATE {table} SET {string.Join(", ", fields.Select(field => $"{field.Name} = ${field.Name}"))}",
            where,
            fields);
    }

    public IWhere? Where { get; }
    public IReadOnlyList<Parameter> Parameters { get; }
    public string GetText() => text;

    static (string Text, IReadOnlyList<Parameter> Parameters) Bind(
        string sql, IWhere? where, IReadOnlyList<Parameter> fields)
    {
        if (where is null) return (sql, fields);
        var text = $"{sql} WHERE {where.GetText()}";
        var split = text.LastIndexOf(" WHERE ", StringComparison.Ordinal);
        var head = split < 0 ? text : text[..split];
        var tail = split < 0 ? "" : text[split..];
        var bound = fields.ToList();
        var values = fields.GroupBy(field => field.Name).ToDictionary(group => group.Key, group => group.Last().Value);
        foreach (var parameter in where.Parameters)
            if (values.TryGetValue(parameter.Name, out var current) && !Equals(current, parameter.Value))
            {
                var alias = $"{parameter.Name}__where";
                tail = Rename(tail, parameter.Name, alias);
                bound.Add(new Parameter(alias, parameter.Value));
            }
            else
                bound.Add(parameter);
        return (head + tail, bound);
    }

    static string Rename(string text, string name, string alias)
    {
        var token = "$" + name;
        var next = "$" + alias;
        var index = 0;
        var result = new System.Text.StringBuilder();
        while (text.IndexOf(token, index, StringComparison.Ordinal) is var at && at >= 0)
        {
            var end = at + token.Length;
            var boundary = end >= text.Length || !char.IsAsciiLetterOrDigit(text[end]) && text[end] != '_';
            result.Append(text[index..at]).Append(boundary ? next : token);
            index = end;
        }
        return result.Append(text[index..]).ToString();
    }
}
