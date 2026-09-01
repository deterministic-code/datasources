using System.Diagnostics;

namespace Datasources;

public class CommandMiddlewareHost
{
    private readonly List<ICommandMiddleware> _middlewares = new();

    public ICommand Use(ICommandMiddleware middleware)
    {
        _middlewares.Add(middleware);
        return (ICommand)this;
    }

    public async Task<T> Run<T>(
        string query,
        IReadOnlyList<Parameter>? parameters,
        Func<string, IReadOnlyList<Parameter>?, Task<(T Value, IReadOnlyList<Row> Rows)>> action)
    {
        var q = query.Trim();
        var p = parameters;
        foreach (var middleware in _middlewares)
        {
            var rewritten = await middleware.BeforeQuery(q, p);
            if (rewritten is { } next)
            {
                q = next.Query;
                p = next.Parameters;
            }
        }

        var clock = Stopwatch.StartNew();
        try
        {
            var (value, rows) = await action(q, p);
            clock.Stop();
            foreach (var middleware in _middlewares)
                await middleware.AfterQuery(q, p, Logged(value, rows), clock.Elapsed.TotalMilliseconds);
            return value;
        }
        catch (Exception ex)
        {
            clock.Stop();
            foreach (var middleware in _middlewares)
                await middleware.AfterQuery(q, p, [], clock.Elapsed.TotalMilliseconds, ex);
            throw;
        }
    }

    static IReadOnlyList<Row> Logged<T>(T value, IReadOnlyList<Row> rows) =>
        rows.Count > 0 || value is not Response response
            ? rows
            : [new Row
            {
                ["result"] = response.Result,
                ["rowsAffected"] = response.RowsAffected,
            }];
}
