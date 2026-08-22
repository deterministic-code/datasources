namespace Datasources;

public static class StubFactory
{
    public static StubDatasource Create(string provider) => new(provider);
}
