namespace Datasources;

public class ReadonlyRepository<T> : IReadonlyRepository<T> where T : new()
{
    readonly IConnection? connection;
    readonly List<ICommandMiddleware> middlewares = [];
    readonly TypeConverter<T, Row>? rowConverter;
    readonly RepositoryStatements? statements;

    public Type Type => typeof(T);
    public IMapper<T, T>? Mapper { get; }

    public ReadonlyRepository(IConnection connection) :
        this(connection, CreateRepositoryOptions.FromType<T>()) { }

    public ReadonlyRepository(IConnection connection, RepositoryOptions<T> options) :
        this(RepositoryBuilder<T>.Parts(connection, options)) { }

    public ReadonlyRepository(BuiltRepository<T> built) :
        this(built.Connection, built.RowConverter, built.CRUDMapper.Rows, built.Statements) { }

    protected ReadonlyRepository() { }

    protected ReadonlyRepository(
        IConnection connection,
        TypeConverter<T, Row> rowConverter,
        IMapper<T, T> mapper,
        RepositoryStatements statements)
    {
        this.connection = connection;
        this.rowConverter = rowConverter;
        Mapper = mapper;
        this.statements = statements;
    }

    public ReadonlyRepository<T> AddMiddleware(ICommandMiddleware middleware)
    {
        middlewares.Add(middleware);
        return this;
    }

    public virtual IAsyncEnumerable<T> Read(IWhere where) =>
        Read(statements is { } sql ? sql.Query(where) : new QueryStatement(where.GetText(), where));

    public virtual Task<T?> ReadSingle(IWhere where) =>
        ReadSingle(statements is { } sql ? sql.Query(where) : new QueryStatement(where.GetText(), where));

    public virtual async IAsyncEnumerable<T> Read(IStatement? query = null)
    {
        var statement = query switch
        {
            IQuery q => q,
            null => Statements.Query(),
            _ => throw new DatasourceException("Read requires IQuery or IWhere")
        };
        var reader = await Command().ExecuteReader(
            statement, statement.Where?.Parameters ?? statement.Parameters);
        while (await reader.Next())
            yield return Mapper!.Map(RowConverter.Convert(reader.Current
                ?? throw new DatasourceException("Reader current must be a row")));
    }

    public virtual async Task<T?> ReadSingle(IStatement? query = null)
    {
        await foreach (var item in Read(query))
            return item;
        return default;
    }

    protected ICommand Command() =>
        middlewares.Aggregate(
            (connection ?? throw new DatasourceException("Connection required")).CreateCommand(),
            (command, middleware) => command.AddMiddleware(middleware));

    protected RepositoryStatements Statements =>
        statements ?? throw new DatasourceException("Statements required");

    protected TypeConverter<T, Row> RowConverter =>
        rowConverter ?? throw new DatasourceException("Row converter required");
}
