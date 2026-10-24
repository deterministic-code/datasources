namespace Datasources;

public sealed class NestedSelectStatement
{
    public required SelectStatement Select { get; init; }
    public required TypeMap Map { get; init; }
}
