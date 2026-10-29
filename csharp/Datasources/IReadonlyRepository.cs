namespace Datasources;

public interface IReadonlyRepository<T> : IRepository
{
    bool IsSchemaRequired { get; }
    Task<T?> ReadSingle(
        IStatement? query = null,
        IReadOnlyList<Parameter>? parameters = null,
        ReadonlyRepositoryMappingOptions<T>? options = null);
    Task<T?> ReadSingle(IWhere where, ReadonlyRepositoryMappingOptions<T>? options = null);
    IAsyncEnumerable<T> Read(
        IStatement? query = null,
        IReadOnlyList<Parameter>? parameters = null,
        ReadonlyRepositoryMappingOptions<T>? options = null);
    IAsyncEnumerable<T> Read(IWhere where, ReadonlyRepositoryMappingOptions<T>? options = null);
}
