namespace Datasources;

public sealed class BatchCommandIBulkWriteStrategy<T> : IBulkWriteStrategy<T> where T : new()
{
    public CRUDRepository<T> Repo { get; set; } = null!;
    public ICommand Command { get; set; } = null!;
    public CRUDRepositoryOptions<T> Options { get; set; } = null!;

    public void Attach(ICommand command, CRUDRepositoryOptions<T> options) =>
        (Command, Options) = (command, options);

    public Task<Response> Create(IReadOnlyList<T> items) =>
        Then(Send([.. items.Select(Insert)]), items, WriteMode.Create);

    public Task<Response> Update(IReadOnlyList<T> items, IWhere? byWhere) =>
        Then(Send([.. items.Select(item => UpdateCommand(item, byWhere))]), items, WriteMode.Update);

    public Task<Response> Delete(IReadOnlyList<T> items, IWhere? byWhere) =>
        Then(Send([.. items.Select(item => DeleteCommand(item, byWhere))]), items, WriteMode.Delete);

    async Task<Response> Then(Task<Response> write, IReadOnlyList<T> items, WriteMode mode)
    {
        var response = await write;
        await BulkWriteEnrichment.After(Repo, Options, items, mode);
        return response;
    }

    Task<Response> Send(IReadOnlyList<BulkCommand> commands)
    {
        if (commands.Count == 0)
            return Task.FromResult(new Response(Value.Null.Instance, 0));
        var renamed = new BulkCommand[commands.Count];
        for (var index = 0; index < commands.Count; index++)
            renamed[index] = Rename(commands[index], index);
        return Command.Execute(
            new Native(string.Join("; ", renamed.Select(command => command.Text))),
            renamed.SelectMany(command => command.Parameters).ToArray());
    }

    BulkCommand Insert(T item)
    {
        var record = Reverse(item);
        var statements = Statements();
        var statement = statements.Insert(Fields(record, col => !Ids.Contains(col) || Assigned(record, col)));
        return new(DefaultStatementTextRenderer.Instance.Render(statement), statement.Fields);
    }

    BulkCommand UpdateCommand(T item, IWhere? byWhere)
    {
        var record = Reverse(item);
        var fields = Fields(record, col => !Ids.Contains(col));
        if (fields.Count == 0)
            throw new DatasourceException("Update requires at least one non-Id field");
        var statement = Statements().Update(fields, byWhere ?? IdWhere(record), (name, value) => Cell(name, value, byWhere is null));
        return new(DefaultStatementTextRenderer.Instance.Render(statement), statement.Parameters);
    }

    BulkCommand DeleteCommand(T item, IWhere? byWhere)
    {
        var record = Reverse(item);
        var statement = Statements().Delete(byWhere ?? IdWhere(record), (name, value) => Cell(name, value, byWhere is null));
        var parameters = statement.Where?.Parameters is { Count: > 0 } listed
            ? listed
            : DefaultStatementTextRenderer.Instance.Render(statement).Contains('$') ? Fields(record, _ => true) : [];
        return new(DefaultStatementTextRenderer.Instance.Render(statement), parameters);
    }

    CRUDRepositoryStatements Statements()
    {
        var builder = new CRUDRepositoryBuilder<T>();
        builder.Parts(Command.Connection, Options);
        return builder.CRUDStatements ?? throw new DatasourceException("Statements required");
    }

    IReadOnlyList<string> Ids => Options.Ids;

    Row Reverse(T item)
    {
        var objects = Options.ObjectConverter ?? new MetaDataRowTypeConverter<T> { ShouldCoerceId = true };
        var row = objects.ReverseConvert(item);
        return Options.RowConverter?.ReverseConvert(row) ?? row;
    }

    IReadOnlyList<Parameter> Fields(Row record, Func<string, bool> include) =>
        [.. record.Where(pair => include(pair.Key) && pair.Value is not Value.Null)
            .OrderBy(pair => pair.Key)
            .Select(pair => new Parameter(pair.Key, Cell(pair.Key, pair.Value, fieldsApplied: true)))];

    Value Cell(string property, Value value, bool fieldsApplied) =>
        fieldsApplied ? value
        : Options.ObjectConverter is MetaDataRowTypeConverter<T> typed
            && typed.Fields.TryGetValue(property, out var field)
            ? field.ReverseConvert(value)
            : value;

    static bool Assigned(Row record, string col) =>
        record.TryGetValue(col, out var value) && value switch
        {
            Value.Null => false,
            Value.Text(var text) => text.Length > 0,
            Value.Int(0) => false,
            _ => true
        };

    IWhere IdWhere(Row record) =>
        new Where(Ids.Select(col =>
            record.TryGetValue(col, out var value)
                ? (IExpression)new Operator.Equals(col, value)
                : throw new DatasourceException($"Missing Id value \"{col}\"")
        ).DefaultIfEmpty(new Operator.Equals(new Literal(1), new Literal(1)))
         .Aggregate((left, right) => new Operator.And(left, right)));

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
