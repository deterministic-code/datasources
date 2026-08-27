using Datasources;

namespace Datasources.Text;

public sealed class TextDatasource : IDatasourceFactory
{
    public static readonly TextDatasource Factory = new();
    public string Provider => "text";
    public IDatasource Create(object? options = null) => new StubDatasource("text");
}
