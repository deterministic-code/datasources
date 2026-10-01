namespace Datasources;

public sealed record UpdateStatement : IUpdateStatement
{
    readonly string? text;
    public string? Table { get; }
    public IReadOnlyList<Parameter> Fields { get; } = [];
    public IWhere? Where { get; }

    public UpdateStatement(string text, IWhere? where = null) => (this.text, Where) = (text, where);

    public UpdateStatement(string table, IReadOnlyList<Parameter> fields, IWhere? where = null) =>
        (Table, Fields, Where) = (table, fields, where);

    public string GetText() => text ?? Compose(Sql(), Where, Fields).Text;

    public IReadOnlyList<Parameter> Parameters =>
        text is null ? Compose(Sql(), Where, Fields).Parameters : [];

    public static IReadOnlyList<Parameter> ParameterList(IReadOnlyList<Parameter> fields, IWhere? where = null) =>
        Compose("", where, fields).Parameters;

    public static (string Text, IReadOnlyList<Parameter> Parameters) Compose(
        string sql, IWhere? where, IReadOnlyList<Parameter> fields)
    {
        if (where is null) return (sql, fields);
        var text = $"{sql} WHERE {where.GetText()}";
        var split = text.LastIndexOf(" WHERE ", StringComparison.Ordinal);
        var head = split < 0 ? text : text[..split];
        var tail = split < 0 ? "" : text[split..];
        var parameters = fields.ToList();
        var values = fields.GroupBy(field => field.Name).ToDictionary(group => group.Key, group => group.Last().Value);
        foreach (var parameter in where.Parameters)
            if (values.TryGetValue(parameter.Name, out var current) && !Equals(current, parameter.Value))
            {
                var alias = $"{parameter.Name}__where";
                tail = Rename(tail, parameter.Name, alias);
                parameters.Add(new Parameter(alias, parameter.Value));
            }
            else
                parameters.Add(parameter);
        return (head + tail, parameters);
    }

    string Sql() => $"UPDATE {Table} SET {Set(Fields)}";

    static string Set(IReadOnlyList<Parameter> fields) =>
        string.Join(", ", fields.Select(field => $"{field.Name} = ${field.Name}"));

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
