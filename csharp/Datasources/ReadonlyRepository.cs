namespace Datasources;

public class ReadonlyRepository<T> : IReadonlyRepository<T>, IEnriched<T>, IEnrichmentHost where T : new()
{
    readonly IConnection? connection;
    readonly List<StoredEnrichment<T>> enrichments = [];
    readonly List<ICommandMiddleware> middlewares = [];
    readonly TypeConverter<Row, Row>? rowConverter;
    readonly TypeConverter<T, Row>? objectConverter;
    readonly IMapper<string, string> entityMappers = new IdentityMapper<string>();
    readonly IMapper<string, string> propertyMappers = new IdentityMapper<string>();
    readonly StatementMappers statementMappers = new(new IdentityMapper<string>(), new IdentityMapper<string>());
    readonly ReadonlyRepositoryStatements? readonlyStatements;

    public Type Type => typeof(T);
    IReadOnlyList<StoredEnrichment> IEnrichmentHost.Enrichments => enrichments;
    protected bool Enriched => enrichments.Count > 0;

    public ReadonlyRepository(IConnection connection) :
        this(connection, CreateReadonlyRepositoryMappingOptions.FromType<T>()) { }

    public ReadonlyRepository(IConnection connection, ReadonlyRepositoryMappingOptions<T> options) :
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
        this.propertyMappers = propertyMappers;
        this.entityMappers = entityMappers;
        statementMappers = new StatementMappers(entityMappers, propertyMappers);
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
        enrichments.Add(new StoredEnrichment<T>(enrichment, new ReadEnrichment<T, TRelated>(via, chosen)));
        return this;
    }

    public virtual IAsyncEnumerable<T> Read(IWhere where, ReadonlyRepositoryMappingOptions<T>? options = null) =>
        Read(
            CompileStatement(Mappers(options).Apply(where, (name, cell) => Cell(name, cell, false, options)) ?? where),
            null,
            options);

    public virtual Task<T?> ReadSingle(IWhere where, ReadonlyRepositoryMappingOptions<T>? options = null) =>
        ReadSingle(
            CompileStatement(Mappers(options).Apply(where, (name, cell) => Cell(name, cell, false, options)) ?? where),
            null,
            options);

    public virtual IAsyncEnumerable<T> Read(
        IStatement? query = null,
        IReadOnlyList<Parameter>? parameters = null,
        ReadonlyRepositoryMappingOptions<T>? options = null)
    {
        var mappers = Mappers(options);
        var objects = Objects(options);
        return Items(ReadTracked(
            mappers.Apply(query, (name, cell) => Cell(name, cell, false, options)),
            parameters,
            mappers,
            Rows(options),
            objects));
    }

    public virtual async Task<T?> ReadSingle(
        IStatement? query = null,
        IReadOnlyList<Parameter>? parameters = null,
        ReadonlyRepositoryMappingOptions<T>? options = null) =>
        await Read(query, parameters, options).FirstOrDefaultAsync() ?? default;

    protected ICommand Command() =>
        middlewares.Aggregate(
            (connection ?? throw new DatasourceException("Connection required")).CreateCommand(),
            (command, middleware) => command.AddMiddleware(middleware));

    ReadonlyRepositoryStatements ReadonlyStatements =>
        readonlyStatements ?? throw new DatasourceException("Statements required");

    protected TypeConverter<Row, Row>? RowConverter => rowConverter;

    protected TypeConverter<T, Row> ObjectConverter =>
        objectConverter ?? throw new DatasourceException("Object converter required");

    protected string Column(string property) => statementMappers.Column(property);

    protected Value Cell(string property, Value value, bool fieldsApplied) =>
        Cell(property, value, fieldsApplied, (ReadonlyRepositoryMappingOptions<T>?)null);

    Value Cell(string property, Value value, bool fieldsApplied, ReadonlyRepositoryMappingOptions<T>? options) =>
        fieldsApplied ? value
        : Field(property, options?.ObjectConverter ?? objectConverter)?.ReverseConvert(value) ?? value;

    static TypeConverter<Value, Value>? Field(string property, TypeConverter<T, Row>? objects) =>
        objects is MetaDataRowTypeConverter<T> typed && typed.Fields.TryGetValue(property, out var field) ? field
        : null;

    StatementMappers Mappers(ReadonlyRepositoryMappingOptions<T>? options) =>
        options?.EntityMapper is null && options?.PropertyMappings is null
            ? statementMappers
            : new StatementMappers(
                options?.EntityMapper ?? entityMappers,
                options?.PropertyMappings ?? propertyMappers);

    TypeConverter<Row, Row>? Rows(ReadonlyRepositoryMappingOptions<T>? options) =>
        options?.RowConverter ?? rowConverter;

    TypeConverter<T, Row> Objects(ReadonlyRepositoryMappingOptions<T>? options) =>
        options?.ObjectConverter ?? ObjectConverter;

    private IAsyncEnumerable<(T Item, Row? Source)> ReadTracked(
        IStatement? query,
        IReadOnlyList<Parameter>? parameters,
        StatementMappers mappers,
        TypeConverter<Row, Row>? rows,
        TypeConverter<T, Row> objects) =>
        Apply(ReadItems(query, parameters ?? (query as ISelectStatement)?.Where?.Parameters, mappers, rows, objects));

    private IAsyncEnumerable<(T Item, Row? Source)> Apply(IAsyncEnumerable<(T Item, Row? Source)> pairs) =>
        Enriched ? Sourced(pairs) : pairs;

    private async IAsyncEnumerable<(T Item, Row? Source)> ReadItems(
        IStatement? query,
        IReadOnlyList<Parameter>? parameters,
        StatementMappers mappers,
        TypeConverter<Row, Row>? rows,
        TypeConverter<T, Row> objects)
    {
        var reader = await Command().ExecuteReader(
            query ?? (ReadonlyStatements.Select() is SelectStatement typed
                ? mappers.Apply(typed, (name, cell) => Cell(name, cell, false, (ReadonlyRepositoryMappingOptions<T>?)null))!
                : ReadonlyStatements.Select()),
            parameters);
        try
        {
            while (await reader.Next())
            {
                var renamed = ColumnToPropertyName(
                    reader.Current ?? throw new DatasourceException("Reader current must be a row"),
                    mappers);
                yield return (objects.Convert(rows?.Convert(renamed) ?? renamed), renamed);
            }
        }
        finally
        {
            if (reader is IAsyncDisposable disposable)
                await disposable.DisposeAsync();
        }
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
        await Enrich(items, sources);
        var paired = items.Count == sources.Count;
        for (var i = 0; i < items.Count; i++)
            yield return (items[i], paired ? sources[i] : null);
    }

    private async Task Enrich(IList<T> parents, IReadOnlyList<Row?>? sources)
    {
        foreach (var entry in enrichments)
            await entry.Read.Apply(entry.Enrichment, parents, sources);
    }

    private static async IAsyncEnumerable<T> Items(IAsyncEnumerable<(T Item, Row? Source)> pairs)
    {
        await foreach (var (item, _) in pairs)
            yield return item;
    }

    private ISelectStatement CompileStatement(IWhere where) =>
        readonlyStatements is { } sql ? sql.Select(where) : new SelectStatement(new() { Where = where });

    private static Row ColumnToPropertyName(Row row, StatementMappers mappers) =>
        new(row.Select(pair => KeyValuePair.Create(mappers.Property(pair.Key), pair.Value)));

}
