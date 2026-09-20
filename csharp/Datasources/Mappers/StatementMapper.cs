namespace Datasources;

public sealed class StatementMapper<T> : IMapper<T, T> where T : new()
{
    public IMapper<string, string> Entity { get; init; } =
        new TransformMapper<string, string>(static name => name, static name => name);
    public IMapper<string, string> Columns { get; init; } =
        new TransformMapper<string, string>(static name => name, static name => name);
    public IMapper<IQuery, IQuery>? Query { get; init; }
    public IMapper<IInsertStatement, IInsertStatement>? Insert { get; init; }
    public IMapper<IUpdateStatement, IUpdateStatement>? Update { get; init; }
    public IMapper<IDeleteStatement, IDeleteStatement>? Delete { get; init; }
    public IMapper<IStoredProcedureStatement, IStoredProcedureStatement>? StoredProcedure { get; init; }
    public IMapper<T, T> Rows { get; init; } =
        new TransformMapper<T, T>(static value => value, static value => value);

    public T Map(T value) => Rows.Map(value);
    public T ReverseMap(T value) => Rows.ReverseMap(value);
}
