namespace Datasources;

public interface ICommand : IMiddlewareHost
{
    IConnection Connection { get; }
    Task<IReader> ExecuteReader(string? text = null, IReadOnlyList<Parameter>? parameters = null);
    Task<Response> Execute(string? text = null, IReadOnlyList<Parameter>? parameters = null);
}
