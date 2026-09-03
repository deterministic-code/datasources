namespace Datasources;

public sealed class RepositoryNotRegisteredException : DatasourceException
{
    public string EntityTypeName { get; }

    public RepositoryNotRegisteredException(string entityTypeName)
        : base($"Repository \"{entityTypeName}\" is not registered") =>
        EntityTypeName = entityTypeName;
}
