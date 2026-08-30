using Datasources.Builders;

namespace Datasources;

public sealed record RepositoryCommands(ICommand Get, ICommand Add, ICommand Update, ICommand Delete);

public sealed class RepositoryStatements
{
    readonly string table;
    readonly IReadOnlyList<string> columns;

    public RepositoryStatements(string table, IReadOnlyList<string> columns)
    {
        this.table = table;
        this.columns = columns;
    }

    public IQuery Query(IWhere? where = null) => StatementBuilder.CreateQuery(table, columns, where);
    public IInsertStatement Insert(IReadOnlyList<Parameter> fields) => StatementBuilder.CreateInsert(table, fields);
    public IUpdateStatement Update(IReadOnlyList<Parameter> fields, IWhere? where = null) =>
        StatementBuilder.CreateUpdate(table, fields, where);
    public IDeleteStatement Delete(IWhere? where = null) => StatementBuilder.CreateDelete(table, where);
}

public sealed record BuiltRepository<T>(
    RepositoryCommands Commands,
    TypeConverter<T, Row> Converters,
    NameSourceDestinationMapper Mapping,
    RepositoryStatements Statements,
    IReadOnlyList<string> Ids,
    bool AssignIdentityOnAdd,
    string Table) where T : new();

public static class RepositoryBuilder<T> where T : new()
{
    public static BuiltRepository<T> Parts(RepositoryBuilderOptions<T> options)
    {
        var converters = options.Mapping ?? new RowConverter<T>(options.Converters, options.Fields);
        var columns = RowConverter<T>.Columns.OrderBy(name => name).ToArray();
        var mapping = new NameSourceDestinationMapper(options.Connection.Provider, "csharp");
        var table = options.Table ?? typeof(T).Name;
        return new(
            new RepositoryCommands(
                options.Connection.CreateCommand(),
                options.Connection.CreateCommand(),
                options.Connection.CreateCommand(),
                options.Connection.CreateCommand()),
            converters,
            mapping,
            new RepositoryStatements(table, columns),
            options.Ids,
            options.AssignIdentityOnAdd,
            table);
    }

    public static Repository<T> Build(RepositoryBuilderOptions<T> options) => new(Parts(options));
}
