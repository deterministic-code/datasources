namespace Datasources;

public interface IUnitOfWork
{
    Task Begin();
    Task Commit();
    Task Abort();
}
