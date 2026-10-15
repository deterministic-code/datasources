namespace Datasources;

public abstract record DecimalRule
{
    public sealed record Optional(string Type, string Fallback) : DecimalRule;
    public sealed record Required(string Type) : DecimalRule;
}
