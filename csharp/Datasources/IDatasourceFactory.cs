namespace Datasources;

public interface IDatasourceFactory
{
    IDatasource Create(IReadOnlyDictionary<string, string?>? options = null);
}
