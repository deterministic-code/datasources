namespace Datasources;

public sealed class RepositoryFactory : IRepositoryFactory, ICRUDRepositoryFactory
{
    readonly Dictionary<Type, IRepository> readonlyByType = new();
    readonly Dictionary<Type, IRepository> crudByType = new();

    public void Register(IRepository repository) => Put(repository, crud: true);

    void IRepositoryFactory.Register(IRepository repository) => Put(repository, crud: false);

    public ICRUDRepository<T> Create<T>() => Get<ICRUDRepository<T>>(crudByType, typeof(T));

    IReadonlyRepository<T> IRepositoryFactory.Create<T>() =>
        Get<IReadonlyRepository<T>>(readonlyByType, typeof(T));

    void Put(IRepository repository, bool crud)
    {
        readonlyByType[repository.Type] = repository;
        if (!crud)
            return;
        crudByType[repository.Type] = repository;
    }

    static T Get<T>(Dictionary<Type, IRepository> map, Type type)
        where T : class =>
        map.TryGetValue(type, out var repository) && repository is T typed
            ? typed
            : throw new RepositoryNotRegisteredException(type.Name);
}
