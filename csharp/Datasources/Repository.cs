using Datasources.Builders;

namespace Datasources;

public sealed class Repository<T> : IRepository<T> where T : new()
{
    readonly RepositoryCommands commands;
    readonly TypeMapping<T> converters;
    readonly DatasourceMapper mapping;
    readonly RepositoryStatements statements;
    readonly HashSet<string> ids;
    readonly bool assignIdentityOnAdd;

    public Repository(IConnection connection) :
        this(CreateRepositoryBuilderOptions.FromType<T>(connection)) { }

    public Repository(RepositoryBuilderOptions<T> options) :
        this(RepositoryBuilder<T>.Parts(options)) { }

    public Repository(BuiltRepository<T> built)
    {
        commands = built.Commands;
        converters = built.Converters;
        mapping = built.Mapping;
        statements = built.Statements;
        ids = new(built.Ids);
        assignIdentityOnAdd = built.AssignIdentityOnAdd;
    }

    public IConnection Connection => commands.Get.Connection;
    public DatasourceMapper Mapping => mapping;

    public Task<T?> GetSingle(IWhere where) => GetSingle(statements.Query(where));

    public IAsyncEnumerable<T> Get(IWhere where) => Get(statements.Query(where));

    public async Task<T?> GetSingle(IQuery? query = null)
    {
        await foreach (var item in Get(query))
            return item;
        return default;
    }

    public async IAsyncEnumerable<T> Get(IQuery? query = null)
    {
        var statement = query ?? statements.Query();
        var reader = await commands.Get.ExecuteReader(
            statement.GetText(), statement.Where?.Parameters ?? statement.Parameters);
        while (await reader.Next())
            yield return converters.To(AsRow(reader.Current));
    }

    public async Task<Response> Add(T obj)
    {
        var fields = Params(converters.From(obj), col => !assignIdentityOnAdd || !ids.Contains(col));
        return await commands.Add.Execute(statements.Insert(fields).GetText(), fields);
    }

    public async Task<Response> Update(T obj, IWhere? byWhere = null)
    {
        var record = converters.From(obj);
        var where = byWhere ?? IdWhere(record);
        var fields = Params(record, col => !ids.Contains(col));
        if (fields.Count == 0)
            throw new DatasourceException("Update requires at least one non-Id field");
        return await commands.Update.Execute(
            statements.Update(fields, where).GetText(),
            [.. fields, .. where.Parameters]);
    }

    public async Task<Response> Delete(T obj, IWhere? byWhere = null)
    {
        var where = byWhere ?? IdWhere(converters.From(obj));
        return await commands.Delete.Execute(statements.Delete(where).GetText(), where.Parameters);
    }

    IWhere IdWhere(IReadOnlyDictionary<string, object?> record) =>
        WhereBuilder.And([.. ids.Select(col =>
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
