namespace Datasources;

public sealed class RepositoryBuilderOptions<T> where T : new()
{
    public required IConnection Connection { get; init; }
    public IReadOnlyList<string> Ids { get; init; } = ["id"];
    public bool AssignIdentityOnAdd { get; init; } = true;
    public TypeConverter<T>? Mapping { get; init; }
}
