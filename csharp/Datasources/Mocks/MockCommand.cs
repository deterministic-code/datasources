namespace Datasources;

public sealed class MockCommand : CommandMiddlewareHost, ICommand
{
    public List<(string? Text, IReadOnlyList<Parameter>? Parameters)> ExecuteCalls { get; } = new();
    public List<(string? Text, IReadOnlyList<Parameter>? Parameters)> ExecuteReaderCalls { get; } = new();
    public Exception? ExecuteError { get; set; }
    public Exception? ExecuteReaderError { get; set; }
    public Response Response { get; set; } = new(Value.Null.Instance, 0);
    public IReader Reader { get; set; } = new MockReader();
    public IConnection Connection { get; }

    public MockCommand(IConnection connection) => Connection = connection;

    public Task<IReader> ExecuteReader(string? text = null, IReadOnlyList<Parameter>? parameters = null) =>
        Run(text, parameters, (query, paramsList) =>
        {
            ExecuteReaderCalls.Add((query, paramsList));
            if (ExecuteReaderError is not null) throw ExecuteReaderError;
            return Task.FromResult((Reader, (IReadOnlyList<IReadOnlyDictionary<string, Value>>)[]));
        });

    public Task<Response> Execute(string? text = null, IReadOnlyList<Parameter>? parameters = null) =>
        Run(text, parameters, (query, paramsList) =>
        {
            ExecuteCalls.Add((query, paramsList));
            if (ExecuteError is not null) throw ExecuteError;
            return Task.FromResult((Response, (IReadOnlyList<IReadOnlyDictionary<string, Value>>)[]));
        });
}
