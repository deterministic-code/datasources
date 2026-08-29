namespace Datasources;

public interface ICommandMiddleware
{
    Task<(string Query, IReadOnlyList<Parameter>? Parameters)?> BeforeQuery(
        string query,
        IReadOnlyList<Parameter>? parameters);

    Task AfterQuery(
        string query,
        IReadOnlyList<Parameter>? parameters,
        IReadOnlyList<IReadOnlyDictionary<string, Value>> results,
        double elapsedMs,
        Exception? error = null);
}
