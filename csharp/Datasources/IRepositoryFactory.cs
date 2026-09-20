namespace Datasources;

public interface IRepositoryFactory
{
    void Register(IRepository repository);
    IReadonlyRepository<T> Create<T>();
}
