namespace Datasources;

public sealed class NestedSelectBuilderOptions
{
    public IWhere? Where { get; init; }
    public IReadOnlyList<string> StopAt { get; init; } = [];
}
