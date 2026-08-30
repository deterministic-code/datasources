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
    TypeConverter<T> Converters,
    NameSourceDestinationMapper Mapping,
    RepositoryStatements Statements,
    IReadOnlyList<string> Ids,
    bool AssignIdentityOnAdd) where T : new();

public static class RepositoryBuilder<T> where T : new()
{
    public static BuiltRepository<T> Parts(RepositoryBuilderOptions<T> options)
    {
        var converters = options.Mapping ?? new TypeConverter<T>();
        var columns = RowConverter.Props(typeof(T)).Select(prop => prop.Name).OrderBy(name => name).ToArray();
        var mapping = new NameSourceDestinationMapper(options.Connection.Provider, "csharp");
        return new(
            new RepositoryCommands(
                options.Connection.CreateCommand(),
                options.Connection.CreateCommand(),
                options.Connection.CreateCommand(),
                options.Connection.CreateCommand()),
            converters,
            mapping,
            new RepositoryStatements(converters.Name, columns),
            options.Ids,
            options.AssignIdentityOnAdd);
    }

    public static Repository<T> Build(RepositoryBuilderOptions<T> options) => new(Parts(options));
}
