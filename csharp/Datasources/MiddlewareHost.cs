namespace Datasources;

public class MiddlewareHost : IMiddlewareHost
{
    private readonly List<MiddlewareHook> _befores = new();
    private readonly List<MiddlewareHook> _afters = new();

    public IMiddlewareHost Use(MiddlewareHook? before = null, MiddlewareHook? after = null)
    {
        if (before is not null) _befores.Add(before);
        if (after is not null) _afters.Add(after);
        return this;
    }

    public async Task<T> Run<T>(
        string operation,
        Func<Task<T>> action,
        string? text = null,
        IReadOnlyList<Parameter>? parameters = null)
    {
        var ctx = new MiddlewareContext
        {
            Operation = operation,
            Host = this,
            Text = text,
            Parameters = parameters,
        };

        foreach (var before in _befores) await before(ctx);

        try
        {
            var result = await action();
            foreach (var after in _afters) await after(ctx);
            return result;
        }
        catch (Exception ex)
        {
            ctx.Error = ex;
            foreach (var after in _afters) await after(ctx);
            throw;
        }
    }

    public Task Run(string operation, Func<Task> action, string? text = null, IReadOnlyList<Parameter>? parameters = null) =>
        Run(operation, async () =>
        {
            await action();
            return 0;
        }, text, parameters);
}
