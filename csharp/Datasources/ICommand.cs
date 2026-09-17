namespace Datasources;

public interface ICommand
{
    IConnection Connection { get; }
    ICommand Use(ICommandMiddleware middleware);
    Task<IReader<Row>> ExecuteReader(string text, IReadOnlyList<Parameter>? parameters = null);
    Task<Response> Execute(string text, IReadOnlyList<Parameter>? parameters = null);
    Task<IReader<Row>> ExecuteReader(IStatement statement, IReadOnlyList<Parameter>? parameters = null) =>
        ExecuteReader(statement.GetText(), parameters);
    Task<Response> Execute(IStatement statement, IReadOnlyList<Parameter>? parameters = null) =>
        Execute(statement.GetText(), parameters);
}
