namespace Datasources;

public sealed class RepositoryBuilderOptions<T> where T : new()
{
    public required IConnection Connection { get; init; }
    public IReadOnlyList<string> Ids { get; init; } = ["id"];
    public bool AssignIdentityOnAdd { get; init; } = true;
    public IReadOnlyDictionary<string, IFieldConverter>? Converters { get; init; }
    public TypeMapping<T>? Mapping { get; init; }
}
