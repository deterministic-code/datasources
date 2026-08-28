using Datasources.Builders;

namespace Datasources;

public sealed class Repository<T> : IRepository<T> where T : new()
{
    readonly RepositoryOptions<T> options;
    readonly TypeMapping<T> mapping;
    readonly HashSet<string> ids;

    public Repository(IConnection connection) : this(RepositoryOptions.FromType<T>(connection)) { }

    public Repository(RepositoryOptions<T> options)
    {
        this.options = options;
        ids = new(options.Ids);
        mapping = options.Mapping ?? TypeConverter.Mapping<T>(
            options.Connection.Provider,
            schema: options.Schema,
            converters: options.Converters,
            names: options.Names);
    }

    public IConnection Connection => options.Connection;
    ICommand Command() => options.Connection.CreateCommand();

    public async Task<T?> GetSingle(IQuery? query = null)
    {
        await foreach (var item in Get(query))
            return item;
        return default;
    }

    public async IAsyncEnumerable<T> Get(IQuery? query = null)
    {
        var statement = query ?? StatementBuilder.CreateQuery(options.DatasourceName, Fields());
        var reader = await Command().ExecuteReader(
            statement.GetText(), statement.Where?.Parameters ?? statement.Parameters);
        while (await reader.Next())
            yield return mapping.To(AsRow(reader.Current));
    }

    public async Task<Response> Add(T obj)
    {
        var fields = Params(mapping.From(obj), col => !options.AssignIdentityOnAdd || !ids.Contains(col));
        return await Command().Execute(
            StatementBuilder.CreateInsert(options.DatasourceName, fields).GetText(), fields);
    }

    public async Task<Response> Update(T obj, IWhere? byWhere = null)
    {
        var record = mapping.From(obj);
        var where = byWhere ?? IdWhere(record);
        var fields = Params(record, col => !ids.Contains(col));
        if (fields.Count == 0)
            throw new DatasourceException("Update requires at least one non-Id field");
        return await Command().Execute(
            StatementBuilder.CreateUpdate(options.DatasourceName, fields, where).GetText(),
            [.. fields, .. where.Parameters]);
    }

    public async Task<Response> Delete(T obj, IWhere? byWhere = null)
    {
        var where = byWhere ?? IdWhere(mapping.From(obj));
        return await Command().Execute(
            StatementBuilder.CreateDelete(options.DatasourceName, where).GetText(), where.Parameters);
    }

    IReadOnlyList<string> Fields() => mapping.Schema.Keys.OrderBy(key => key).ToArray();

    IWhere IdWhere(IReadOnlyDictionary<string, object?> record) =>
        WhereBuilder.And([.. options.Ids.Select(col =>
            record.TryGetValue(col, out var value)
                ? WhereBuilder.Equals(col, value)
                : throw new DatasourceException($"Missing Id value \"{col}\""))]);

    static IReadOnlyList<Parameter> Params(
        IReadOnlyDictionary<string, object?> record, Func<string, bool> include) =>
        record.Where(pair => include(pair.Key)).OrderBy(pair => pair.Key)
            .Select(pair => new Parameter(pair.Key, pair.Value)).ToArray();

    static IReadOnlyDictionary<string, object?> AsRow(object? current) =>
        current is IReadOnlyDictionary<string, object?> row
            ? row
            : throw new DatasourceException("Reader current must be a row dictionary");
}
