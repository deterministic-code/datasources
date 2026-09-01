namespace Datasources;

public interface ICommand
{
    IConnection Connection { get; }
    ICommand Use(ICommandMiddleware middleware);
    Task<IReader<Row>> ExecuteReader(string text, IReadOnlyList<Parameter>? parameters = null);
    Task<Response> Execute(string text, IReadOnlyList<Parameter>? parameters = null);
}
