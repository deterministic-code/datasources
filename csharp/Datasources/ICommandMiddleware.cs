namespace Datasources;

public interface ICommandMiddleware
{
    Task<(string Query, IReadOnlyList<Parameter>? Parameters)?> BeforeQuery(
        string query,
        IReadOnlyList<Parameter>? parameters);

    Task AfterQuery(
        string query,
        IReadOnlyList<Parameter>? parameters,
        IReadOnlyList<IReadOnlyDictionary<string, object?>> results,
        double elapsedMs,
        Exception? error = null);
}
