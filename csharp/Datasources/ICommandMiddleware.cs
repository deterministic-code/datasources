namespace Datasources;

public interface ICommandMiddleware
{
    Task<(string Text, IReadOnlyList<Parameter>? Parameters)?> BeforeCommand(
        string text,
        IReadOnlyList<Parameter>? parameters);

    Task AfterCommand(
        string text,
        IReadOnlyList<Parameter>? parameters,
        IReadOnlyList<Row> results,
        double elapsedMs,
        Exception? error = null);
}
