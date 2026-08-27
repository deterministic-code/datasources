namespace Datasources;

public class StubReader<T> : IReader<T>
{
    private readonly string _provider;
    public StubReader(string provider) => _provider = provider;
    public T? Current => default;
    public Task<bool> Next() => throw new ProviderNotImplementedException(_provider);
}

public sealed class StubReader : StubReader<object>, IReader
{
    public StubReader(string provider) : base(provider) { }
}
