using Datasources;

namespace Datasources.Memory;

internal sealed class MemoryCommand : CommandMiddlewareHost, ICommand
{
    readonly StatementTextRenderer statements;

    public MemoryCommand(MemoryConnection connection, StatementTextRenderer? statements = null)
    {
        Connection = connection;
        this.statements = statements ?? new StatementTextRenderer(t => new MemoryStatementRenderer(t));
    }

    public IConnection Connection { get; }
    MemoryConnection Typed => (MemoryConnection)Connection;
    MemoryEngine Engine => Typed.Engine;

    public Task<IReader<Row>> ExecuteReader(Native text, IReadOnlyList<Parameter>? parameters = null) =>
        ExecuteReader(text.Text, parameters);

    public Task<Response> Execute(Native text, IReadOnlyList<Parameter>? parameters = null) =>
        Execute(text.Text, parameters);

    public override Task<IReader<Row>> ExecuteReader(string text, IReadOnlyList<Parameter>? parameters = null) =>
        Run(text, parameters, (query, paramsList) =>
        {
            Typed.EnsureOpen();
            Engine.Execute(query, paramsList);
            IReadOnlyList<Row> rows = Engine.Records;
            return Task.FromResult(((IReader<Row>)new ArrayReader<Row>(rows), rows));
        });

    public Task<IReader<Row>> ExecuteReader(IStatement statement, IReadOnlyList<Parameter>? parameters = null) =>
        ExecuteReader(statements.Render(statement), parameters);

    public Task<Response> Execute(IStatement statement, IReadOnlyList<Parameter>? parameters = null) =>
        Execute(statements.Render(statement), parameters);

    public override Task<Response> Execute(string text, IReadOnlyList<Parameter>? parameters = null) =>
        string.IsNullOrWhiteSpace(text)
            ? throw new InvalidOperationException("empty command")
            : Run(text, parameters, (query, paramsList) =>
            {
                Typed.EnsureOpen();
                var response = Engine.Execute(query, paramsList);
                return Task.FromResult((response, (IReadOnlyList<Row>)[]));
            });
}
