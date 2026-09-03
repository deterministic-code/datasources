namespace Datasources;

public interface ICRUDRepositoryFactory
{
    void Register<T>(Func<IConnection?, ICRUDRepository<T>> factory);
    void Register(string entityTypeName, Func<IConnection?, object> factory);
    void Register(Type type, Func<IConnection?, object> factory);
    ICRUDRepository<T> Create<T>(IConnection? connection = null);
    object Create(string entityTypeName, IConnection? connection = null);
    object Create(Type type, IConnection? connection = null);
}
