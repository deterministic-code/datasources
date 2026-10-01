namespace Datasources;

public sealed class CRUDRepositoryStatements : ReadonlyRepositoryStatements
{
    readonly StatementMappings mappings;

    public CRUDRepositoryStatements(string table, IReadOnlyList<string> columns)
        : this(table, columns, new IdentityMapper<string>(), new IdentityMapper<string>()) { }

    public CRUDRepositoryStatements(
        string table,
        IReadOnlyList<string> columns,
        IMapper<string, string> entityMappers,
        IMapper<string, string> propertyMappers) : base(table, columns) =>
        mappings = new(entityMappers, propertyMappers);

    public InsertStatement Insert(IReadOnlyList<Parameter> fields) =>
        (InsertStatement)mappings.Apply(new InsertStatement(Table, fields), (_, value) => value)!;

    public UpdateStatement Update(
        IReadOnlyList<Parameter> fields,
        IWhere? where = null,
        Func<string, Value, Value>? cell = null) =>
        (UpdateStatement)mappings.Apply(
            new UpdateStatement(Table, fields, where),
            cell ?? ((_, value) => value))!;

    public DeleteStatement Delete(IWhere? where = null, Func<string, Value, Value>? cell = null) =>
        (DeleteStatement)mappings.Apply(
            new DeleteStatement(Table, where),
            cell ?? ((_, value) => value))!;

    public SelectInsertedIdStatement SelectInsertedId(string column) =>
        (SelectInsertedIdStatement)mappings.Apply(
            new SelectInsertedIdStatement(Table, column),
            (_, value) => value)!;
}
