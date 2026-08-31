using System.Collections;
using System.Reflection;
using Datasources.Builders;

namespace Datasources;

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
    IConnection Connection,
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
        var converters = options.Mapping ?? new RowTypeConverter<T>(null, options.Fields);
        var mapping = new NameSourceDestinationMapper(options.Connection.Provider, "csharp");
        var table = options.Table ?? typeof(T).Name;
        return new(
            options.Connection,
            converters,
            mapping,
            new RepositoryStatements(table, Columns()),
            options.Ids,
            options.AssignIdentityOnAdd,
            table);
    }

    public static Repository<T> Build(RepositoryBuilderOptions<T> options) => new(Parts(options));

    static string[] Columns() =>
        typeof(IDictionary).IsAssignableFrom(typeof(T))
            ? []
            : [.. typeof(T).GetProperties(BindingFlags.Public | BindingFlags.Instance)
                .Where(prop => prop.CanRead && prop.GetIndexParameters().Length == 0)
                .Select(prop => prop.Name)
                .OrderBy(name => name)];
}
