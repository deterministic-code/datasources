namespace Datasources;

public class ReadonlyRepository<T> : IReadonlyRepository<T>, IEnriched<T>, IEnrichmentHost where T : new()
{
    readonly IConnection? connection;
    readonly List<StoredEnrichment> enrichments = [];
    readonly List<ICommandMiddleware> middlewares = [];
    readonly TypeConverter<Row, Row>? rowConverter;
    readonly TypeConverter<T, Row>? objectConverter;
    readonly StatementMappings statementMappings = new(new IdentityMapper<string>(), new IdentityMapper<string>());
    readonly ReadonlyRepositoryStatements? readonlyStatements;

    public Type Type => typeof(T);
    IReadOnlyList<StoredEnrichment> IEnrichmentHost.Enrichments => enrichments;
    protected bool Enriched => enrichments.Count > 0;

    public ReadonlyRepository(IConnection connection) :
        this(connection, CreateCRUDRepositoryOptions.FromType<T>()) { }

    public ReadonlyRepository(IConnection connection, ReadonlyRepositoryOptions<T> options) :
        this(new ReadonlyRepositoryBuilder<T>().Parts(connection, options)) { }

    public ReadonlyRepository(BuiltRepository<T> built) :
        this(built.Connection, built.RowConverter, built.ObjectConverter, built.PropertyMappings, built.EntityMapper, built.Statements) { }

    protected ReadonlyRepository() { }

    protected ReadonlyRepository(
        IConnection connection,
        TypeConverter<Row, Row>? rowConverter,
        TypeConverter<T, Row> objectConverter,
        IMapper<string, string> propertyMappers,
        IMapper<string, string> entityMappers,
        ReadonlyRepositoryStatements statements)
    {
        this.connection = connection;
        this.rowConverter = rowConverter;
        this.objectConverter = objectConverter;
        statementMappings = new StatementMappings(entityMappers, propertyMappers);
        readonlyStatements = statements;
    }

    public ReadonlyRepository<T> AddMiddleware(ICommandMiddleware middleware)
    {
        middlewares.Add(middleware);
        return this;
    }

    public IEnriched<T> AddEnriched<TRelated>(
        Enrichment enrichment,
        IReadonlyRepository<TRelated> via,
        IReadonlyEnrichmentStrategy<T>? strategy = null)
    {
        var chosen = strategy ?? new DefaultReadonlyEnrichmentStrategy<T>();
        enrichments.Add(new StoredEnrichment(
            enrichment,
            (parents, sources) => chosen.Apply(via, enrichment, parents, sources)));
        return this;
    }

    public virtual IAsyncEnumerable<T> Read(IWhere where) =>
        Read(Compiled(statementMappings.Apply(where, (name, cell) => Cell(name, cell, false)) ?? where));

    public virtual Task<T?> ReadSingle(IWhere where) =>
        ReadSingle(Compiled(statementMappings.Apply(where, (name, cell) => Cell(name, cell, false)) ?? where));

    public virtual IAsyncEnumerable<T> Read(IStatement? query = null, IReadOnlyList<Parameter>? parameters = null) =>
        Items(ReadTracked(statementMappings.Apply(query, (name, cell) => Cell(name, cell, false)), parameters));

    public virtual async Task<T?> ReadSingle(IStatement? query = null, IReadOnlyList<Parameter>? parameters = null) =>
        await Read(query, parameters).FirstOrDefaultAsync() ?? default;

    protected virtual IAsyncEnumerable<(T Item, Row? Source)> ReadItems(
        IStatement? query = null, IReadOnlyList<Parameter>? parameters = null) =>
        ReadRows(query, parameters);

    protected ICommand Command() =>
        middlewares.Aggregate(
            (connection ?? throw new DatasourceException("Connection required")).CreateCommand(),
            (command, middleware) => command.AddMiddleware(middleware));

    ReadonlyRepositoryStatements ReadonlyStatements =>
        readonlyStatements ?? throw new DatasourceException("Statements required");

    protected TypeConverter<Row, Row>? RowConverter => rowConverter;

    protected TypeConverter<T, Row> ObjectConverter =>
        objectConverter ?? throw new DatasourceException("Object converter required");

    protected string Column(string property) => statementMappings.Column(property);

    protected Value Cell(string property, Value value, bool fieldsApplied) =>
        fieldsApplied ? value : Field(property)?.ReverseConvert(value) ?? value;

    TypeConverter<Value, Value>? Field(string property) =>
        objectConverter is MetaDataRowTypeConverter<T> typed && typed.Fields.TryGetValue(property, out var field) ? field
        : null;

    private IAsyncEnumerable<(T Item, Row? Source)> ReadTracked(
        IStatement? query = null, IReadOnlyList<Parameter>? parameters = null) =>
        Apply(ReadItems(query, parameters ?? (query as ISelectStatement)?.Where?.Parameters));

    private IAsyncEnumerable<(T Item, Row? Source)> Apply(IAsyncEnumerable<(T Item, Row? Source)> pairs) =>
        Enriched ? Sourced(pairs) : pairs;

    private async IAsyncEnumerable<Row> ReadSources(IStatement? query = null, IReadOnlyList<Parameter>? parameters = null)
    {
        var reader = await Command().ExecuteReader(
            query ?? (ReadonlyStatements.Select() is SelectStatement typed
                ? statementMappings.Apply(typed, (name, cell) => Cell(name, cell, false))!
                : ReadonlyStatements.Select()),
            parameters);
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

    private async IAsyncEnumerable<(T Item, Row? Source)> ReadRows(IStatement? query, IReadOnlyList<Parameter>? parameters)
    {
        await foreach (var renamed in ReadSources(query, parameters))
            yield return (ObjectConverter.Convert(rowConverter?.Convert(renamed) ?? renamed), renamed);
    }

    private async IAsyncEnumerable<(T Item, Row? Source)> Sourced(IAsyncEnumerable<(T Item, Row? Source)> pairs)
    {
        var items = new List<T>();
        var sources = new List<Row?>();
        await foreach (var (item, source) in pairs)
        {
            items.Add(item);
            sources.Add(source);
        }
        var box = items.Cast<object>().ToList();
        await Enrich(box, sources);
        items.Clear();
        foreach (var item in box)
            items.Add((T)item);
        var paired = items.Count == sources.Count;
        for (var i = 0; i < items.Count; i++)
            yield return (items[i], paired ? sources[i] : null);
    }

    private async Task Enrich(IList<object> parents, IReadOnlyList<Row?>? sources)
    {
        foreach (var entry in enrichments)
            await entry.Apply(parents, sources);
    }

    private static async IAsyncEnumerable<T> Items(IAsyncEnumerable<(T Item, Row? Source)> pairs)
    {
        await foreach (var (item, _) in pairs)
            yield return item;
    }

    private ISelectStatement Compiled(IWhere where) =>
        readonlyStatements is { } sql ? sql.Select(where) : new SelectStatement(where.GetText(), where);

    private Row Renamed(Row row) =>
        new(row.Select(pair => KeyValuePair.Create(statementMappings.Property(pair.Key), pair.Value)));
}
