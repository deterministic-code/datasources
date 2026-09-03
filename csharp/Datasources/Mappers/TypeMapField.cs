namespace Datasources;

public abstract record TypeMapField
{
    public sealed record Column(string Source) : TypeMapField;
    public sealed record Group(TypeMapGroup Map) : TypeMapField;

    public static implicit operator TypeMapField(string source) => new Column(source);
    public static implicit operator TypeMapField(TypeMapGroup map) => new Group(map);
}
