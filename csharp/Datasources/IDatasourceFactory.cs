namespace Datasources;

public interface IDatasourceFactory
{
    string Provider { get; }
    IDatasource Create(object? options = null);
}
