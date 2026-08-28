namespace Datasources;

public sealed class RepositoryOptions<T> where T : new()
{
    public required string DatasourceName { get; init; }
    public required IConnection Connection { get; init; }
    public IReadOnlyList<string> Ids { get; init; } = ["id"];
    public bool AssignIdentityOnAdd { get; init; } = true;
    public IReadOnlyDictionary<string, string>? Schema { get; init; }
    public IReadOnlyDictionary<string, IFieldConverter>? Converters { get; init; }
    public IReadOnlyDictionary<string, string>? Names { get; init; }
    public TypeMapping<T>? Mapping { get; init; }
}
