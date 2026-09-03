namespace Datasources;

public sealed class RepositoryFactory : IRepositoryFactory, ICRUDRepositoryFactory
{
    readonly Dictionary<string, Func<IConnection?, object>> readonlyByName = new(StringComparer.Ordinal);
    readonly Dictionary<Type, Func<IConnection?, object>> readonlyByType = new();
    readonly Dictionary<string, Func<IConnection?, object>> crudByName = new(StringComparer.Ordinal);
    readonly Dictionary<Type, Func<IConnection?, object>> crudByType = new();

    public void Register<T>(Func<IConnection?, ICRUDRepository<T>> factory) =>
        Put(typeof(T), typeof(T).Name, connection => factory(connection), crud: true);

    void IRepositoryFactory.Register<T>(Func<IConnection?, IReadonlyRepository<T>> factory) =>
        Put(typeof(T), typeof(T).Name, connection => factory(connection), crud: false);

    public void Register(string entityTypeName, Func<IConnection?, object> factory) =>
        Put(null, entityTypeName, factory, crud: true);

    void IRepositoryFactory.Register(string entityTypeName, Func<IConnection?, object> factory) =>
        Put(null, entityTypeName, factory, crud: false);

    public void Register(Type type, Func<IConnection?, object> factory) =>
        Put(type, type.Name, factory, crud: true);

    void IRepositoryFactory.Register(Type type, Func<IConnection?, object> factory) =>
        Put(type, type.Name, factory, crud: false);

    public ICRUDRepository<T> Create<T>(IConnection? connection = null) =>
        Get<ICRUDRepository<T>>(crudByType, typeof(T), connection);

    IReadonlyRepository<T> IRepositoryFactory.Create<T>(IConnection? connection) =>
        Get<IReadonlyRepository<T>>(readonlyByType, typeof(T), connection);

    public object Create(string entityTypeName, IConnection? connection = null) =>
        Invoke(Lookup(readonlyByName, entityTypeName), connection);

    object ICRUDRepositoryFactory.Create(string entityTypeName, IConnection? connection) =>
        Invoke(Lookup(crudByName, entityTypeName), connection);

    public object Create(Type type, IConnection? connection = null) =>
        Invoke(Lookup(readonlyByType, type), connection);

    object ICRUDRepositoryFactory.Create(Type type, IConnection? connection) =>
        Invoke(Lookup(crudByType, type), connection);

    void Put(Type? type, string name, Func<IConnection?, object> factory, bool crud)
    {
        readonlyByName[name] = factory;
        if (type is not null)
            readonlyByType[type] = factory;
        if (!crud)
            return;
        crudByName[name] = factory;
        if (type is not null)
            crudByType[type] = factory;
    }

    static T Get<T>(Dictionary<Type, Func<IConnection?, object>> byType, Type type, IConnection? connection)
        where T : class =>
        Lookup(byType, type)(connection) as T
            ?? throw new RepositoryNotRegisteredException(type.Name);

    static Func<IConnection?, object> Lookup<TKey>(Dictionary<TKey, Func<IConnection?, object>> map, TKey key)
        where TKey : notnull =>
        map.TryGetValue(key, out var factory)
            ? factory
            : throw new RepositoryNotRegisteredException(key is Type type ? type.Name : $"{key}");

    static object Invoke(Func<IConnection?, object> factory, IConnection? connection) =>
        factory(connection);
}
