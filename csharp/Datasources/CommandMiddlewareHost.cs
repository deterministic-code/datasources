using System.Diagnostics;

namespace Datasources;

public class CommandMiddlewareHost
{
    private readonly List<ICommandMiddleware> _middlewares = new();

    public ICommand AddMiddleware(ICommandMiddleware middleware)
    {
        _middlewares.Add(middleware);
        return (ICommand)this;
    }

    public virtual Task<IReader<Row>> ExecuteReader(string text, IReadOnlyList<Parameter>? parameters = null) =>
        string.IsNullOrWhiteSpace(text)
            ? ((ICommand)this).ExecuteReader(new Native(text), parameters)
            : ((ICommand)this).ExecuteReader(StatementParser.Parse(text), parameters);

    public virtual Task<Response> Execute(string text, IReadOnlyList<Parameter>? parameters = null) =>
        string.IsNullOrWhiteSpace(text)
            ? ((ICommand)this).Execute(new Native(text), parameters)
            : ((ICommand)this).Execute(StatementParser.Parse(text), parameters);

    public async Task<T> Run<T>(
        string text,
        IReadOnlyList<Parameter>? parameters,
        Func<string, IReadOnlyList<Parameter>?, Task<(T Value, IReadOnlyList<Row> Rows)>> action)
    {
        var q = text.Trim();
        var p = parameters;
        foreach (var middleware in _middlewares)
        {
            var rewritten = await middleware.BeforeCommand(q, p);
            if (rewritten is { } next)
            {
                q = next.Text;
                p = next.Parameters;
            }
        }

        var clock = Stopwatch.StartNew();
        try
        {
            var (value, rows) = await action(q, p);
            clock.Stop();
            foreach (var middleware in _middlewares)
                await middleware.AfterCommand(q, p, Logged(value, rows), clock.Elapsed.TotalMilliseconds);
            return value;
        }
        catch (Exception ex)
        {
            clock.Stop();
            foreach (var middleware in _middlewares)
                await middleware.AfterCommand(q, p, [], clock.Elapsed.TotalMilliseconds, ex);
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
