namespace Datasources;

public sealed class MockCommand : MiddlewareHost, ICommand
{
    public List<(string? Text, IReadOnlyList<Parameter>? Parameters)> ExecuteCalls { get; } = new();
    public List<(string? Text, IReadOnlyList<Parameter>? Parameters)> ExecuteReaderCalls { get; } = new();
    public Exception? ExecuteError { get; set; }
    public Exception? ExecuteReaderError { get; set; }
    public Response Response { get; set; } = new(null, 0);
    public IReader Reader { get; set; } = new MockReader();
    public IConnection Connection { get; }

    public MockCommand(IConnection connection) => Connection = connection;

    public Task<IReader> ExecuteReader(string? text = null, IReadOnlyList<Parameter>? parameters = null) =>
        Run("executeReader", () =>
        {
            ExecuteReaderCalls.Add((text, parameters));
            if (ExecuteReaderError is not null) throw ExecuteReaderError;
            return Task.FromResult(Reader);
        }, text, parameters);

    public Task<Response> Execute(string? text = null, IReadOnlyList<Parameter>? parameters = null) =>
        Run("execute", () =>
        {
            ExecuteCalls.Add((text, parameters));
            if (ExecuteError is not null) throw ExecuteError;
            return Task.FromResult(Response);
        }, text, parameters);
}
