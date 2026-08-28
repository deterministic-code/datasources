namespace Datasources;

public interface ICommand
{
    IConnection Connection { get; }
    ICommand Use(ICommandMiddleware middleware);
    Task<IReader> ExecuteReader(string? text = null, IReadOnlyList<Parameter>? parameters = null);
    Task<Response> Execute(string? text = null, IReadOnlyList<Parameter>? parameters = null);
}
