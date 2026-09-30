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

    public Task<IReader<Row>> ExecuteReader(Native text, IReadOnlyList<Parameter>? parameters = null) =>
        Run<IReader<Row>>(text.Text, parameters, (_, _) => throw new ProviderNotImplementedException(_provider));

    public Task<Response> Execute(Native text, IReadOnlyList<Parameter>? parameters = null) =>
        Run<Response>(text.Text, parameters, (_, _) => throw new ProviderNotImplementedException(_provider));
}
