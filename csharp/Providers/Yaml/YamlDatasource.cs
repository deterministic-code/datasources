using Datasources;

namespace Datasources.Yaml;

public static class YamlProvider
{
    public static readonly StubDatasource Factory = StubFactory.Create("yaml");

    static YamlProvider()
    {
        Datasource.Use(Factory);
    }
}
