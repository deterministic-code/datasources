namespace Datasources;

public interface ICommand
{
    IConnection Connection { get; }
    ICommand AddMiddleware(ICommandMiddleware middleware);
    Task<IReader<Row>> ExecuteReader(Native text, IReadOnlyList<Parameter>? parameters = null);
    Task<Response> Execute(Native text, IReadOnlyList<Parameter>? parameters = null);
    Task<IReader<Row>> ExecuteReader(string text, IReadOnlyList<Parameter>? parameters = null) =>
        string.IsNullOrWhiteSpace(text)
            ? ExecuteReader(new Native(text), parameters)
            : ExecuteReader(StatementParser.Parse(text), parameters);
    Task<Response> Execute(string text, IReadOnlyList<Parameter>? parameters = null) =>
        string.IsNullOrWhiteSpace(text)
            ? Execute(new Native(text), parameters)
            : Execute(StatementParser.Parse(text), parameters);
    Task<IReader<Row>> ExecuteReader(IStatement statement, IReadOnlyList<Parameter>? parameters = null) =>
        ExecuteReader(new Native(statement.GetText()), parameters);
    Task<Response> Execute(IStatement statement, IReadOnlyList<Parameter>? parameters = null) =>
        Execute(new Native(statement.GetText()), parameters);
}
