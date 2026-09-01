namespace Datasources;

public sealed class ConsoleCommandMiddleware : ICommandMiddleware
{
    readonly Action<string> write;
    readonly Func<DateTimeOffset> now;
    readonly IReadOnlyDictionary<string, string> options;

    public ConsoleCommandMiddleware(
        Action<string>? write = null,
        Func<DateTimeOffset>? now = null,
        IReadOnlyDictionary<string, string>? options = null)
    {
        this.write = write ?? Console.WriteLine;
        this.now = now ?? (() => DateTimeOffset.UtcNow);
        this.options = options ?? new Dictionary<string, string>();
    }

    public Task<(string Query, IReadOnlyList<Parameter>? Parameters)?> BeforeQuery(
        string query,
        IReadOnlyList<Parameter>? parameters)
    {
        write(Line("Start", query));
        if (Verbose()) write(Line("Request", ParamsOf(parameters)));
        return Task.FromResult<(string Query, IReadOnlyList<Parameter>? Parameters)?>(null);
    }

    public Task AfterQuery(
        string query,
        IReadOnlyList<Parameter>? parameters,
        IReadOnlyList<Row> results,
        double elapsedMs,
        Exception? error = null)
    {
        var elapsed = FormatElapsed(elapsedMs);
        var bag = new Dictionary<string, string>(options) { ["elapsedMs"] = elapsed };
        write($"{Line("Finish", query)} {bag["elapsedMs"]}");
        if (Verbose()) write(Line("Response", RowsOf(results)));
        return Task.CompletedTask;
    }

    public static string FormatElapsed(double elapsedMs) =>
        elapsedMs < 1 ? $"{Math.Round(elapsedMs * 1000)}μs" : $"{Math.Round(elapsedMs)}ms";

    bool Verbose() =>
        options.TryGetValue("isVerbose", out var flag) && flag is "true" or "1";

    string Line(string phase, string query) =>
        $"[{now().UtcDateTime:yyyy-MM-ddTHH:mm:ss.fff}Z]-[datasource][{phase}]-[{query}]";

    static string ParamsOf(IReadOnlyList<Parameter>? parameters) =>
        $"{{{string.Join(", ", (parameters ?? []).Select(param => $"{param.Name}: {Cell(param.Value)}"))}}}";

    static string RowsOf(IReadOnlyList<Row> results) =>
        string.Join(", ", results.Select(row => $"{{{string.Join(", ", row.Select(pair => $"{pair.Key}: {Cell(pair.Value)}"))}}}"));

    static string Cell(Value value) => value switch
    {
        Value.Null => "null",
        Value.DateTime => value.IsoDateTime(),
        Value.Binary(var bytes) => Convert.ToBase64String(bytes),
        Value.Text(var text) => text,
        _ => $"{value.ToObject()}"
    };
}
