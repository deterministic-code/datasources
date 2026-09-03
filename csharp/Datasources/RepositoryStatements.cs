using Datasources.Builders;

namespace Datasources;

public sealed class RepositoryStatements
{
    readonly string table;
    readonly IReadOnlyList<string> columns;
    readonly IMapper<IQuery, IQuery>? query;
    readonly IMapper<IInsertStatement, IInsertStatement>? insert;
    readonly IMapper<IUpdateStatement, IUpdateStatement>? update;
    readonly IMapper<IDeleteStatement, IDeleteStatement>? delete;

    public RepositoryStatements(string table, IReadOnlyList<string> columns)
        : this(table, columns, null, null, null, null) { }

    public RepositoryStatements(
        string table,
        IReadOnlyList<string> columns,
        IMapper<IQuery, IQuery>? query,
        IMapper<IInsertStatement, IInsertStatement>? insert,
        IMapper<IUpdateStatement, IUpdateStatement>? update,
        IMapper<IDeleteStatement, IDeleteStatement>? delete)
    {
        this.table = table;
        this.columns = columns;
        this.query = query;
        this.insert = insert;
        this.update = update;
        this.delete = delete;
    }

    public IReadOnlyList<string> Columns => columns;

    public IQuery Query(IWhere? where = null) =>
        Mapped(StatementBuilder.CreateQuery(table, columns, where), query);
    public IInsertStatement Insert(IReadOnlyList<Parameter> fields) =>
        Mapped(StatementBuilder.CreateInsert(table, fields), insert);
    public IUpdateStatement Update(IReadOnlyList<Parameter> fields, IWhere? where = null) =>
        Mapped(StatementBuilder.CreateUpdate(table, fields, where), update);
    public IDeleteStatement Delete(IWhere? where = null) =>
        Mapped(StatementBuilder.CreateDelete(table, where), delete);

    static T Mapped<T>(T built, IMapper<T, T>? mapper) =>
        mapper switch
        {
            { } map => map.Map(built),
            _ => built
        };
}
