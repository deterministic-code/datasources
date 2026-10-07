namespace Datasources;

public sealed class RepositoryAccess(
    Func<IConnection, object> create,
    Func<IConnection, object> createReadonly,
    IConnection connection)
{
    public object Create() => create(connection);
    public object CreateReadonly() => createReadonly(connection);
}
