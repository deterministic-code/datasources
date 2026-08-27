namespace Datasources;

public class StubCommand : MiddlewareHost, ICommand
{
    private readonly string _provider;
    public StubCommand(string provider, IConnection connection)
    {
        _provider = provider;
        Connection = connection;
    }

    public IConnection Connection { get; }

    public Task<IReader> ExecuteReader(string? text = null, IReadOnlyList<Parameter>? parameters = null) =>
        Run<IReader>("executeReader", () => throw new ProviderNotImplementedException(_provider), text, parameters);

    public Task<Response> Execute(string? text = null, IReadOnlyList<Parameter>? parameters = null) =>
        Run<Response>("execute", () => throw new ProviderNotImplementedException(_provider), text, parameters);
}
