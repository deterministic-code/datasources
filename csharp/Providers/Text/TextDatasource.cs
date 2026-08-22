using Datasources;

namespace Datasources.Text;

public static class TextProvider
{
    public static readonly StubDatasource Factory = StubFactory.Create("text");

    static TextProvider()
    {
        Datasource.Use(Factory);
    }
}
