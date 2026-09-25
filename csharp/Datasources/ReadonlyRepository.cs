namespace Datasources;

public class ReadonlyRepository<T> : IReadonlyRepository<T>, IEnriched<T> where T : new()
{
    readonly IConnection? connection;
    readonly IMapper<IEnumerable<Row>, IReadOnlyList<T>>? result;
    readonly List<ICommandMiddleware> middlewares = [];
    readonly TypeConverter<T, Row>? rowConverter;
    readonly IMapper<string, string> columns = new IdentityMapper<string>();
    readonly RepositoryStatements? statements;

    public Type Type => typeof(T);
    public IMapper<T, T>? Mapper { get; }
    public IRepositoryTree Tree { get; }
    protected bool Enriched => Tree.Parent.Items.Count > 0;

    public ReadonlyRepository(IConnection connection) :
        this(connection, CreateRepositoryOptions.FromType<T>()) { }

    public ReadonlyRepository(IConnection connection, ReadonlyRepositoryOptions<T> options) :
        this(RepositoryBuilder<T>.Parts(connection, options)) { }

    public ReadonlyRepository(BuiltRepository<T> built) :
        this(built.Connection, built.RowConverter, built.RowsMapper, built.Result, built.ColumnMappings, built.Statements) { }

    protected ReadonlyRepository(IMapper<IEnumerable<Row>, IReadOnlyList<T>>? result = null)
    {
        this.result = result;
        Tree = new RepositoryTree(new RepositoryTreeNode(this, "", binder: new DefaultReadonlyEnrichmentStrategy<T>()));
    }

    protected ReadonlyRepository(
        IConnection connection,
        TypeConverter<T, Row> rowConverter,
        IMapper<T, T> mapper,
        IMapper<IEnumerable<Row>, IReadOnlyList<T>>? result,
        IMapper<string, string> columns,
        RepositoryStatements statements)
    {
        this.connection = connection;
        this.rowConverter = rowConverter;
        Mapper = mapper;
        this.result = result;
        this.columns = columns;
        this.statements = statements;
        Tree = new RepositoryTree(new RepositoryTreeNode(this, "", binder: new DefaultReadonlyEnrichmentStrategy<T>()));
    }

    public ReadonlyRepository<T> AddMiddleware(ICommandMiddleware middleware)
    {
        middlewares.Add(middleware);
        return this;
    }

    public IEnriched<T> AddEnriched<TRelated>(
        string into,
        string from,
        IReadonlyRepository<TRelated> via,
        string by,
        string? select = null,
        bool many = false,
        IReadonlyEnrichmentStrategy<T>? strategy = null)
    {
        ((RepositoryTreeNode)Tree.Parent).Include(
            via, into, from, by, select, many, strategy ?? new DefaultReadonlyEnrichmentStrategy<T>());
        return this;
    }

    public IEnriched<T> AddLinked<TRelated, TLink>(
        string into,
        string from,
        IReadonlyRepository<TRelated> via,
        string by,
        ICRUDRepository<TLink> link,
        string parent,
        string related,
        IReadonlyEnrichmentStrategy<T>? strategy = null)
    {
        ((RepositoryTreeNode)Tree.Parent).IncludeLinked(
            via, into, from, by, link, parent, related, strategy ?? new DefaultReadonlyEnrichmentStrategy<T>());
        return this;
    }

    public Task Enrich(IList<T> items) =>
        ((RepositoryTreeNode)Tree.Parent).EnrichFrom(items.Cast<object>().ToList());

    public virtual IAsyncEnumerable<T> Read(IWhere where) =>
        Read(Compiled(Stored(where)));

    protected internal virtual IAsyncEnumerable<(T Item, Row? Source)> ReadTracked(IWhere where) =>
        ReadTracked(Compiled(Stored(where)));

    public virtual Task<T?> ReadSingle(IWhere where) =>
        ReadSingle(Compiled(Stored(where)));

    public virtual IAsyncEnumerable<T> Read(IStatement? query = null) =>
        Items(ReadTracked(query));

    protected internal virtual IAsyncEnumerable<(T Item, Row? Source)> ReadTracked(IStatement? query = null) =>
        Apply(ReadItems(query));

    protected virtual IAsyncEnumerable<(T Item, Row? Source)> ReadItems(IStatement? query = null) =>
        result is null ? ReadRows(query) : Collapse(query);

    IAsyncEnumerable<(T Item, Row? Source)> Apply(IAsyncEnumerable<(T Item, Row? Source)> pairs) =>
        Enriched ? Sourced(pairs) : pairs;

    async IAsyncEnumerable<(T Item, Row? Source)> Collapse(IStatement? query)
    {
        var rows = new List<Row>();
        await foreach (var row in ReadSources(query))
            rows.Add(row);
        foreach (var item in result!.Map(rows))
            yield return (item, null);
    }

    protected virtual async IAsyncEnumerable<Row> ReadSources(IStatement? query = null)
    {
        var statement = query switch
        {
            IExecuteStoredProcedureStatement call => (IStatement)call,
            ISelectStatement q => q,
            null => Statements.Query(),
            _ => throw new DatasourceException("Read requires ISelectStatement, IWhere, or IExecuteStoredProcedureStatement")
        };
        var parameters = statement switch
        {
            ISelectStatement q => q.Where?.Parameters ?? q.Parameters,
            _ => statement.Parameters
        };
        var reader = await Command().ExecuteReader(statement, parameters);
        try
        {
            while (await reader.Next())
                yield return Renamed(reader.Current ?? throw new DatasourceException("Reader current must be a row"));
        }
        finally
        {
            if (reader is IAsyncDisposable disposable)
                await disposable.DisposeAsync();
        }
    }

    async IAsyncEnumerable<(T Item, Row? Source)> ReadRows(IStatement? query)
    {
        await foreach (var renamed in ReadSources(query))
            yield return (Mapper!.Map(RowConverter.Convert(renamed)), renamed);
    }

    async IAsyncEnumerable<(T Item, Row? Source)> Sourced(IAsyncEnumerable<(T Item, Row? Source)> pairs)
    {
        var items = new List<T>();
        var sources = new List<Row?>();
        await foreach (var (item, source) in pairs)
        {
            items.Add(item);
            sources.Add(source);
        }
        await ((RepositoryTreeNode)Tree.Parent).EnrichFrom(items.Cast<object>().ToList(), sources);
        for (var i = 0; i < items.Count; i++)
            yield return (items[i], i < sources.Count ? sources[i] : null);
    }

    static async IAsyncEnumerable<T> Items(IAsyncEnumerable<(T Item, Row? Source)> pairs)
    {
        await foreach (var (item, _) in pairs)
            yield return item;
    }

    protected static async IAsyncEnumerable<(T Item, Row? Source)> WithoutSource(IAsyncEnumerable<T> items)
    {
        await foreach (var item in items)
            yield return (item, null);
    }

    public virtual async Task<T?> ReadSingle(IStatement? query = null) =>
        await Read(query).FirstOrDefaultAsync() ?? default;

    protected ICommand Command() =>
        middlewares.Aggregate(
            (connection ?? throw new DatasourceException("Connection required")).CreateCommand(),
            (command, middleware) => command.AddMiddleware(middleware));

    protected RepositoryStatements Statements =>
        statements ?? throw new DatasourceException("Statements required");

    protected TypeConverter<T, Row> RowConverter =>
        rowConverter ?? throw new DatasourceException("Row converter required");

    protected IWhere Stored(IWhere where, bool fieldsApplied = false) =>
        where is Where clause
            ? clause.Map(Column, (name, cell) => Cell(name, cell, fieldsApplied))
            : where;

    protected string Column(string property) => columns.Map(property);

    protected Value Cell(string property, Value value, bool fieldsApplied) =>
        rowConverter is RowTypeConverter<T> typed
        && !fieldsApplied
        && typed.Fields.TryGetValue(property, out var field)
            ? field.ReverseConvert(value)
            : value;

    ISelectStatement Compiled(IWhere where) =>
        statements is { } sql ? sql.Query(where) : new SelectStatement(where.GetText(), where);

    Row Renamed(Row row) =>
        new(row.Select(pair => KeyValuePair.Create(columns.ReverseMap(pair.Key), pair.Value)));
}
