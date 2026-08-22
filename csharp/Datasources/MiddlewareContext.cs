namespace Datasources;

public sealed class MiddlewareContext
{
    public required string Operation { get; init; }
    public required object Host { get; init; }
    public string? Text { get; init; }
    public IReadOnlyList<Parameter>? Parameters { get; init; }
    public Exception? Error { get; set; }
}
