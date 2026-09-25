namespace Datasources;

public sealed class ExpectConsoleMiddleware : ICommandMiddleware
{
    readonly ConsoleCommandMiddleware inner;
    readonly IReadOnlyList<string> expected;
    int index;

    public ExpectConsoleMiddleware(IReadOnlyList<string> expected, Func<DateTimeOffset> now)
    {
        this.expected = expected;
        inner = new ConsoleCommandMiddleware(Check, now, new Dictionary<string, string> { ["isVerbose"] = "true" });
    }

    public Task<(string Text, IReadOnlyList<Parameter>? Parameters)?> BeforeCommand(
        string text,
        IReadOnlyList<Parameter>? parameters) =>
        inner.BeforeCommand(text, parameters);

    public Task AfterCommand(
        string text,
        IReadOnlyList<Parameter>? parameters,
        IReadOnlyList<Row> results,
        double elapsedMs,
        Exception? error = null) =>
        inner.AfterCommand(text, parameters, results, elapsedMs, error);

    public void Complete()
    {
        if (index != expected.Count)
            throw new InvalidOperationException($"console mismatch at {index}\nexpected: {expected[index]}\nactual:   <missing>");
    }

    public static string Signature(string line)
    {
        var duration = line.LastIndexOf("] ", StringComparison.Ordinal);
        return duration >= 0 && (line.EndsWith("ms", StringComparison.Ordinal) || line.EndsWith("μs", StringComparison.Ordinal))
            ? line[..(duration + 1)]
            : line;
    }

    void Check(string line)
    {
        var actual = Signature(line);
        if (index >= expected.Count || actual != expected[index])
            throw new InvalidOperationException($"console mismatch at {index}\nexpected: {(index < expected.Count ? expected[index] : "<missing>")}\nactual:   {actual}");
        index++;
    }
}
