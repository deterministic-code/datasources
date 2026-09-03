namespace Datasources;

public interface IEnriched<T>
{
    IEnriched<T> AddEnriched<TRelated>(
        string into,
        string from,
        IReadonlyRepository<TRelated> via,
        string by,
        string? select = null,
        bool many = false);
    Task Enrich(IList<T> items);
}
