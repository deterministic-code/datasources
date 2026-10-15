namespace Datasources;

public sealed class DefaultValueOptions
{
    public required Func<string> NewId { get; init; }
    public required Func<string> Now { get; init; }
    public required Func<string> UtcNow { get; init; }
    public required string BooleanOn { get; init; }
    public required string BooleanOff { get; init; }
    public required Func<string, string> Hex { get; init; }
    public required System.Text.RegularExpressions.Regex HexReverse { get; init; }
}
