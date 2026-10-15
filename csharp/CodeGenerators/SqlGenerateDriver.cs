namespace Datasources.CodeGenerators;

public sealed class SqlGenerateDriver
{
    public required IDriver Driver { get; init; }
    public required string QuoteLeft { get; init; }
    public required string QuoteRight { get; init; }
    public required DeterministicTypeConverter TypeConverter { get; init; }
    public required DefaultValueConverter DefaultValueConverter { get; init; }
    public required StatementTextRenderer StatementRenderer { get; init; }
}
