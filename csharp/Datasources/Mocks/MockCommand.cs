namespace Datasources;

public sealed class MockCommand : CommandMiddlewareHost, ICommand
{
    public List<(string Text, IReadOnlyList<Parameter>? Parameters)> ExecuteCalls { get; } = new();
    public List<(string Text, IReadOnlyList<Parameter>? Parameters)> ExecuteReaderCalls { get; } = new();
    public Exception? ExecuteError { get; set; }
    public Exception? ExecuteReaderError { get; set; }
    public Response Response { get; set; } = new(Value.Null.Instance, 0);
    public IReader<Row> Reader { get; set; } = new MockReader<Row>();
    public IConnection Connection { get; }

    public MockCommand(IConnection connection) => Connection = connection;

    public Task<IReader<Row>> ExecuteReader(IStatement statement, IReadOnlyList<Parameter>? parameters = null) =>
        statement is SelectInsertedIdStatement inserted
            ? ExecuteReader(new Native($"{inserted.Table}.{inserted.Column}"), parameters)
            : ExecuteReader(new Native(statement.GetText()), parameters);

    public Task<IReader<Row>> ExecuteReader(Native text, IReadOnlyList<Parameter>? parameters = null) =>
        Run(text.Text, parameters, (query, paramsList) =>
        {
            ExecuteReaderCalls.Add((query, paramsList));
            if (ExecuteReaderError is not null) throw ExecuteReaderError;
            return Task.FromResult((Reader, (IReadOnlyList<Row>)[]));
        });

    public Task<Response> Execute(Native text, IReadOnlyList<Parameter>? parameters = null) =>
        Run(text.Text, parameters, (query, paramsList) =>
        {
            ExecuteCalls.Add((query, paramsList));
            if (ExecuteError is not null) throw ExecuteError;
            return Task.FromResult((Response, (IReadOnlyList<Row>)[]));
        });
}
