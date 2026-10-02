namespace Datasources;

public class UpdateStatementTextRenderer(StatementTextRenderer text)
{
    public virtual string Render(UpdateStatement statement) =>
        Compose($"UPDATE {statement.Table} SET {Set(statement.Fields)}", statement.Where, statement.Fields).Text;

    public (string Text, IReadOnlyList<Parameter> Parameters) Compose(
        string sql, IWhere? where, IReadOnlyList<Parameter> fields)
    {
        if (where is null) return (sql, fields);
        var rendered = $"{sql} WHERE {text.Render(where)}";
        var split = rendered.LastIndexOf(" WHERE ", StringComparison.Ordinal);
        var head = split < 0 ? rendered : rendered[..split];
        var tail = split < 0 ? "" : rendered[split..];
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

    static string Set(IReadOnlyList<Parameter> fields) =>
        string.Join(", ", fields.Select(field => $"{field.Name} = ${field.Name}"));

    static string Rename(string sql, string name, string alias)
    {
        var token = "$" + name;
        var next = "$" + alias;
        var index = 0;
        var result = new System.Text.StringBuilder();
        while (sql.IndexOf(token, index, StringComparison.Ordinal) is var at && at >= 0)
        {
            var end = at + token.Length;
            var boundary = end >= sql.Length || !char.IsAsciiLetterOrDigit(sql[end]) && sql[end] != '_';
            result.Append(sql[index..at]).Append(boundary ? next : token);
            index = end;
        }
        return result.Append(sql[index..]).ToString();
    }
}
