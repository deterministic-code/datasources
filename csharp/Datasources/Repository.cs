using Datasources.Builders;

namespace Datasources;

public sealed class Repository<T> : IRepository<T> where T : new()
{
    readonly RepositoryCommands commands;
    readonly TypeConverter<T> converters;
    readonly NameSourceDestinationMapper mapping;
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
    public TypeConverter<T> Converters => converters;
    public NameSourceDestinationMapper Mapping => mapping;

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
            yield return converters.Convert(Map(AsRow(reader.Current), mapping.Map));
    }

    public async Task<Response> Add(T obj)
    {
        var fields = Params(Sql(obj), col => !assignIdentityOnAdd || !ids.Contains(col));
        return await commands.Add.Execute(statements.Insert(fields).GetText(), fields);
    }

    public async Task<Response> Update(T obj, IWhere? byWhere = null)
    {
        var record = Sql(obj);
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
        var where = byWhere ?? IdWhere(Sql(obj));
        return await commands.Delete.Execute(statements.Delete(where).GetText(), where.Parameters);
    }

    IWhere IdWhere(Row record) =>
        WhereBuilder.And([.. ids.Select(col =>
            record.TryGetValue(col, out var value)
                ? WhereBuilder.Equals(col, value)
                : throw new DatasourceException($"Missing Id value \"{col}\""))]);

    static IReadOnlyList<Parameter> Params(
        Row record, Func<string, bool> include) =>
        record.Where(pair => include(pair.Key)).OrderBy(pair => pair.Key)
            .Select(pair => new Parameter(pair.Key, pair.Value)).ToArray();

    Row Sql(T obj) => Map(converters.ReverseConvert(obj), mapping.ReverseMap);

    static Row Map(Row row, Func<string, string> name) =>
        new(row.Select(pair => KeyValuePair.Create(name(pair.Key), pair.Value)));

    static Row AsRow(object? current) => current switch
    {
        Row row => row,
        IReadOnlyDictionary<string, Value> row => new Row(row),
        _ => throw new DatasourceException("Reader current must be a row")
    };
}
