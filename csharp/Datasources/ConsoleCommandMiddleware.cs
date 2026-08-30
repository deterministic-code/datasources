namespace Datasources;

public sealed class ConsoleCommandMiddleware : ICommandMiddleware
{
    private readonly Action<string> _write;
    private readonly Func<DateTimeOffset> _now;

    public ConsoleCommandMiddleware(Action<string>? write = null, Func<DateTimeOffset>? now = null)
    {
        _write = write ?? Console.WriteLine;
        _now = now ?? (() => DateTimeOffset.UtcNow);
    }

    public Task<(string Query, IReadOnlyList<Parameter>? Parameters)?> BeforeQuery(
        string query,
        IReadOnlyList<Parameter>? parameters)
    {
        _write(Line("Start", query));
        return Task.FromResult<(string Query, IReadOnlyList<Parameter>? Parameters)?>(null);
    }

    public Task AfterQuery(
        string query,
        IReadOnlyList<Parameter>? parameters,
        IReadOnlyList<Row> results,
        double elapsedMs,
        Exception? error = null)
    {
        _write($"{Line("Finish", query)} {FormatElapsed(elapsedMs)}");
        return Task.CompletedTask;
    }

    public static string FormatElapsed(double elapsedMs) =>
        elapsedMs < 1 ? $"{Math.Round(elapsedMs * 1000)}μs" : $"{Math.Round(elapsedMs)}ms";

    private string Line(string phase, string query) =>
        $"[{_now().UtcDateTime:yyyy-MM-ddTHH:mm:ss.fff}Z]-[datasource][{phase}]-[{query}]";
}
