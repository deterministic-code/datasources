namespace Datasources;

public abstract record MappedValue
{
    public sealed record Cell(Value Value) : MappedValue;
    public sealed record Many(List<Dictionary<string, MappedValue>> Items) : MappedValue;
}
