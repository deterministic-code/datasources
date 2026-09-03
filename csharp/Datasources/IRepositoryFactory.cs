namespace Datasources;

public interface IRepositoryFactory
{
    void Register<T>(Func<IConnection?, IReadonlyRepository<T>> factory);
    void Register(string entityTypeName, Func<IConnection?, object> factory);
    void Register(Type type, Func<IConnection?, object> factory);
    IReadonlyRepository<T> Create<T>(IConnection? connection = null);
    object Create(string entityTypeName, IConnection? connection = null);
    object Create(Type type, IConnection? connection = null);
}
