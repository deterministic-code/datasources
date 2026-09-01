namespace Datasources;

public class StubCommand : CommandMiddlewareHost, ICommand
{
    private readonly string _provider;
    public StubCommand(string provider, IConnection connection)
    {
        _provider = provider;
        Connection = connection;
    }

    public IConnection Connection { get; }

    public Task<IReader<Row>> ExecuteReader(string text, IReadOnlyList<Parameter>? parameters = null) =>
        Run<IReader<Row>>(text, parameters, (_, _) => throw new ProviderNotImplementedException(_provider));

    public Task<Response> Execute(string text, IReadOnlyList<Parameter>? parameters = null) =>
        Run<Response>(text, parameters, (_, _) => throw new ProviderNotImplementedException(_provider));
}
