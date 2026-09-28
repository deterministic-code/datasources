namespace Datasources;

public sealed class BatchCommands<T> : IBulkWriteStrategy<T>
{
    public Task<Response> Create(IReadOnlyList<T> items, IBulkWrite<T> write) =>
        Send(write, write.Inserts(items));

    public Task<Response> Update(IReadOnlyList<T> items, IWhere? byWhere, IBulkWrite<T> write) =>
        Send(write, write.Updates(items, byWhere));

    public Task<Response> Delete(IReadOnlyList<T> items, IWhere? byWhere, IBulkWrite<T> write) =>
        Send(write, write.Deletes(items, byWhere));

    static Task<Response> Send(IBulkWrite<T> write, IReadOnlyList<BulkCommand> commands)
    {
        if (commands.Count == 0)
            return Task.FromResult(new Response(Value.Null.Instance, 0));
        var renamed = new BulkCommand[commands.Count];
        for (var index = 0; index < commands.Count; index++)
            renamed[index] = Rename(commands[index], index);
        return write.Command.Execute(
            string.Join("; ", renamed.Select(command => command.Text)),
            renamed.SelectMany(command => command.Parameters).ToArray());
    }

    static BulkCommand Rename(BulkCommand command, int index)
    {
        var text = command.Text;
        var parameters = command.Parameters.ToArray();
        foreach (var name in parameters.Select(parameter => parameter.Name).Distinct().OrderByDescending(name => name.Length))
        {
            var alias = $"{name}__{index}";
            text = Rename(text, name, alias);
            for (var at = 0; at < parameters.Length; at++)
                if (parameters[at].Name == name)
                    parameters[at] = parameters[at] with { Name = alias };
        }
        return new BulkCommand(text, parameters);
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
