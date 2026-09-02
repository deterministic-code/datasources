using Datasources.Builders;

namespace Datasources;

public sealed class Repository<T> : IRepository<T> where T : new()
{
    readonly IConnection connection;
    readonly List<ICommandMiddleware> middlewares = [];
    readonly TypeConverter<T, Row> rowConverter;
    public StatementMapper<T> StatementsMapper { get; }
    public IMapper<T, T> Mapper => StatementsMapper.Rows;
    readonly RepositoryStatements statements;
    readonly HashSet<string> ids;
    readonly bool assignIdentityOnAdd;

    public Repository(IConnection connection) :
        this(CreateRepositoryBuilderOptions.FromType<T>(connection)) { }

    public Repository(RepositoryBuilderOptions<T> options) :
        this(RepositoryBuilder<T>.Parts(options)) { }

    public Repository(BuiltRepository<T> built)
    {
        connection = built.Connection;
        rowConverter = built.RowConverter;
        StatementsMapper = built.StatementMapper;
        statements = built.Statements;
        ids = new(built.Ids);
        assignIdentityOnAdd = built.AssignIdentityOnAdd;
    }

    public Repository<T> Use(ICommandMiddleware middleware)
    {
        middlewares.Add(middleware);
        return this;
    }

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
        var reader = await Command().ExecuteReader(
            statement.GetText(), statement.Where?.Parameters ?? statement.Parameters);
        while (await reader.Next())
            yield return Mapper.Map(rowConverter.Convert(reader.Current
                ?? throw new DatasourceException("Reader current must be a row")));
    }

    public async Task<Response> Add(T item)
    {
        var fields = Params(ReverseConvert(item), col => !assignIdentityOnAdd || !ids.Contains(col));
        return await Command().Execute(statements.Insert(fields).GetText(), fields);
    }

    public async Task<Response> Update(T item, IWhere? byWhere = null)
    {
        var record = ReverseConvert(item);
        var where = byWhere ?? IdWhere(record);
        var fields = Params(record, col => !ids.Contains(col));
        if (fields.Count == 0)
            throw new DatasourceException("Update requires at least one non-Id field");
        return await Command().Execute(
            statements.Update(fields, where).GetText(),
            [.. fields, .. where.Parameters]);
    }

    public async Task<Response> Delete(T item, IWhere? byWhere = null)
    {
        var where = byWhere ?? IdWhere(ReverseConvert(item));
        return await Command().Execute(statements.Delete(where).GetText(), where.Parameters);
    }

    ICommand Command() =>
        middlewares.Aggregate(connection.CreateCommand(), (command, middleware) => command.Use(middleware));

    IWhere IdWhere(Row record) =>
        WhereBuilder.And([.. ids.Select(col =>
            record.TryGetValue(col, out var value)
                ? WhereBuilder.Equals(col, value)
                : throw new DatasourceException($"Missing Id value \"{col}\""))]);

    static IReadOnlyList<Parameter> Params(
        Row record, Func<string, bool> include) =>
        record.Where(pair => include(pair.Key)).OrderBy(pair => pair.Key)
            .Select(pair => new Parameter(pair.Key, pair.Value)).ToArray();

    Row ReverseConvert(T item) => rowConverter.ReverseConvert(Mapper.ReverseMap(item));
}
