namespace Datasources;

public sealed class RepositoryFactory : IRepositoryFactory, ICRUDRepositoryFactory
{
    static readonly List<Action<RepositoryFactory>> binders = [];
    static readonly object gate = new();
    readonly Dictionary<Type, IRepository> readonlyByType = new();
    readonly Dictionary<Type, IRepository> crudByType = new();
    readonly Dictionary<string, Named> byName = new(StringComparer.Ordinal);

    public RepositoryFactory() => ApplyBound();

    public static void Bind(Action<RepositoryFactory> register)
    {
        lock (gate) binders.Add(register);
    }

    public static void ResetBinders()
    {
        lock (gate) binders.Clear();
    }

    public void Register(IRepository repository) => Put(repository, crud: true);

    void IRepositoryFactory.Register(IRepository repository) => Put(repository, crud: false);

    public void Register(
        string typeName,
        string pluralName,
        Func<IConnection, object> create,
        Func<IConnection, object> createReadonly)
    {
        var named = new Named(create, createReadonly);
        byName[typeName] = named;
        byName[pluralName] = named;
    }

    public ICRUDRepository<T> Create<T>() => Get<ICRUDRepository<T>>(crudByType, typeof(T));

    IReadonlyRepository<T> IRepositoryFactory.Create<T>() =>
        Get<IReadonlyRepository<T>>(readonlyByType, typeof(T));

    public TRepository Create<TRepository>(string name, IConnection connection) where TRepository : class
    {
        ApplyBound();
        return Lookup(name).Create(connection) as TRepository
            ?? throw new InvalidCastException($"Repository \"{name}\" is not {typeof(TRepository).Name}");
    }

    public TRepository CreateReadonly<TRepository>(string name, IConnection connection) where TRepository : class
    {
        ApplyBound();
        return Lookup(name).CreateReadonly(connection) as TRepository
            ?? throw new InvalidCastException($"Repository \"{name}\" is not {typeof(TRepository).Name}");
    }

    public RepositoryAccess Access(string name, IConnection connection)
    {
        ApplyBound();
        var named = Lookup(name);
        return new RepositoryAccess(named.Create, named.CreateReadonly, connection);
    }

    void ApplyBound()
    {
        Action<RepositoryFactory>[] snapshot;
        lock (gate) snapshot = [.. binders];
        foreach (var register in snapshot)
            register(this);
    }

    void Put(IRepository repository, bool crud)
    {
        readonlyByType[repository.Type] = repository;
        if (!crud)
            return;
        crudByType[repository.Type] = repository;
    }

    Named Lookup(string name) =>
        byName.TryGetValue(name, out var named)
            ? named
            : throw new RepositoryNotRegisteredException(name);

    static T Get<T>(Dictionary<Type, IRepository> map, Type type)
        where T : class =>
        map.TryGetValue(type, out var repository) && repository is T typed
            ? typed
            : throw new RepositoryNotRegisteredException(type.Name);

    sealed record Named(Func<IConnection, object> Create, Func<IConnection, object> CreateReadonly);
}
