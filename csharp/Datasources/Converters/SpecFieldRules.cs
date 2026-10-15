namespace Datasources;

public sealed class SpecFieldRules
{
    public string? DialectName { get; init; }
    public required string StringUnlimited { get; init; }
    public required Func<int, string> StringSized { get; init; }
    public string? Character { get; init; }
    public DecimalRule? Decimal { get; init; }
    public string? BinaryUnlimited { get; init; }
    public Func<int, string>? BinarySized { get; init; }
}
