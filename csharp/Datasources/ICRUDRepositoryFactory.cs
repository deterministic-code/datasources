namespace Datasources;

public interface ICRUDRepositoryFactory
{
    void Register(IRepository repository);
    ICRUDRepository<T> Create<T>();
}
