namespace Datasources;

public sealed class RepositoryStatements
{
    readonly string table;
    readonly IReadOnlyList<string> columns;
    readonly IMapper<ISelectStatement, ISelectStatement>? select;
    readonly IMapper<IInsertStatement, IInsertStatement>? insert;
    readonly IMapper<IUpdateStatement, IUpdateStatement>? update;
    readonly IMapper<IDeleteStatement, IDeleteStatement>? delete;

    public RepositoryStatements(string table, IReadOnlyList<string> columns)
        : this(table, columns, null, null, null, null) { }

    public RepositoryStatements(
        string table,
        IReadOnlyList<string> columns,
        IMapper<ISelectStatement, ISelectStatement>? select,
        IMapper<IInsertStatement, IInsertStatement>? insert,
        IMapper<IUpdateStatement, IUpdateStatement>? update,
        IMapper<IDeleteStatement, IDeleteStatement>? delete)
    {
        this.table = table;
        this.columns = columns;
        this.select = select;
        this.insert = insert;
        this.update = update;
        this.delete = delete;
    }

    public IReadOnlyList<string> ReadColumns => columns;

    public ISelectStatement Select(IWhere? where = null) =>
        Mapped(new SelectStatement(new() { Table = table, Fields = columns, Where = where }), select);
    public IInsertStatement Insert(IReadOnlyList<Parameter> fields) =>
        Mapped(new InsertStatement(table, fields), insert);
    public IUpdateStatement Update(IReadOnlyList<Parameter> fields, IWhere? where = null) =>
        Mapped(new UpdateStatement(table, fields, where), update);
    public IDeleteStatement Delete(IWhere? where = null) =>
        Mapped(new DeleteStatement(table, where), delete);
    public SelectInsertedIdStatement SelectInsertedId(string column) => new(table, column);

    static T Mapped<T>(T built, IMapper<T, T>? mapper) =>
        mapper switch
        {
            { } map => map.Map(built),
            _ => built
        };
}
