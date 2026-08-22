namespace Datasources;

public sealed class StubReader : IReader
{
    private readonly string _provider;
    public StubReader(string provider) => _provider = provider;
    public object? Current => null;
    public Task<bool> Next() => throw new ProviderNotImplementedException(_provider);
}
