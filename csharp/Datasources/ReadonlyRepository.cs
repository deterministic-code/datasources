namespace Datasources;

public class ReadonlyRepository<T> : IReadonlyRepository<T> where T : new()
{
    readonly IConnection? connection;
    readonly List<ICommandMiddleware> middlewares = [];
    readonly TypeConverter<T, Row>? rowConverter;
    readonly IMapper<string, string> columns = new IdentityMapper<string>();
    readonly RepositoryStatements? statements;

    public Type Type => typeof(T);
    public IMapper<T, T>? Mapper { get; }

    public ReadonlyRepository(IConnection connection) :
        this(connection, CreateRepositoryOptions.FromType<T>()) { }

    public ReadonlyRepository(IConnection connection, ReadonlyRepositoryOptions<T> options) :
        this(RepositoryBuilder<T>.Parts(connection, options)) { }

    public ReadonlyRepository(BuiltRepository<T> built) :
        this(built.Connection, built.RowConverter, built.RowsMapper, built.ColumnMappings, built.Statements) { }

    protected ReadonlyRepository() { }

    protected ReadonlyRepository(
        IConnection connection,
        TypeConverter<T, Row> rowConverter,
        IMapper<T, T> mapper,
        IMapper<string, string> columns,
        RepositoryStatements statements)
    {
        this.connection = connection;
        this.rowConverter = rowConverter;
        Mapper = mapper;
        this.columns = columns;
        this.statements = statements;
    }

    public ReadonlyRepository<T> AddMiddleware(ICommandMiddleware middleware)
    {
        middlewares.Add(middleware);
        return this;
    }

    public virtual IAsyncEnumerable<T> Read(IWhere where) => Read(Compiled(Stored(where)));

    public virtual Task<T?> ReadSingle(IWhere where) => ReadSingle(Compiled(Stored(where)));

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
            yield return Mapper!.Map(RowConverter.Convert(Renamed(reader.Current
                ?? throw new DatasourceException("Reader current must be a row"))));
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
        where is WhereClause clause ? new WhereClause(Rewrite(clause.Condition, fieldsApplied)) : where;

    protected string Column(string property) => columns.Map(property);

    protected Value Cell(string property, Value value, bool fieldsApplied)
    {
        if (rowConverter is not RowTypeConverter<T> typed)
            return value;
        if (typed.Fields.TryGetValue(property, out var field))
            return fieldsApplied ? value : field.ReverseConvert(value);
        var kind = value switch
        {
            Value.Bool => "bool",
            Value.DateTime => "datetime",
            Value.Guid => "uuid",
            Value.Decimal => "decimal",
            Value.Binary => "binary",
            _ => null
        };
        return kind is not null && typed.Types.TryGetValue(kind, out var converter)
            ? converter.ReverseConvert(value)
            : value;
    }

    IQuery Compiled(IWhere where) =>
        statements is { } sql ? sql.Query(where) : new QueryStatement(where.GetText(), where);

    Row Renamed(Row row) =>
        new(row.Select(pair => KeyValuePair.Create(columns.ReverseMap(pair.Key), pair.Value)));

    IExpression Rewrite(IExpression expression, bool fieldsApplied) => expression switch
    {
        WhereClause where => Rewrite(where.Condition, fieldsApplied),
        Identifier id => new Identifier(columns.Map(id.Name)),
        Literal => expression,
        ArrayExpression arr => new ArrayExpression([.. arr.Elements.Select(element => Rewrite(element, fieldsApplied))]),
        Operator.Not not => new Operator.Not(Rewrite(not.Argument, fieldsApplied)),
        Operator.And(var left, var right) => new Operator.And(Rewrite(left, fieldsApplied), Rewrite(right, fieldsApplied)),
        Operator.Or(var left, var right) => new Operator.Or(Rewrite(left, fieldsApplied), Rewrite(right, fieldsApplied)),
        Operator.In(var left, var right) => new Operator.In(Rewrite(left, fieldsApplied), InRight(left, right, fieldsApplied)),
        Operator.Equals(var left, var right) => new Operator.Equals(Side(left, right, fieldsApplied), Side(right, left, fieldsApplied)),
        Operator.NotEquals(var left, var right) => new Operator.NotEquals(Side(left, right, fieldsApplied), Side(right, left, fieldsApplied)),
        Operator.GreaterThan(var left, var right) => new Operator.GreaterThan(Side(left, right, fieldsApplied), Side(right, left, fieldsApplied)),
        Operator.LessThan(var left, var right) => new Operator.LessThan(Side(left, right, fieldsApplied), Side(right, left, fieldsApplied)),
        Operator.GreaterThanOrEqual(var left, var right) => new Operator.GreaterThanOrEqual(Side(left, right, fieldsApplied), Side(right, left, fieldsApplied)),
        Operator.LessThanOrEqual(var left, var right) => new Operator.LessThanOrEqual(Side(left, right, fieldsApplied), Side(right, left, fieldsApplied)),
        _ => throw new InvalidOperationException("empty expression")
    };

    IExpression Side(IExpression expression, IExpression other, bool fieldsApplied) =>
        expression is Literal lit && other is Identifier id
            ? new Literal(Cell(id.Name, lit.Value, fieldsApplied))
            : Rewrite(expression, fieldsApplied);

    IExpression InRight(IExpression left, IExpression right, bool fieldsApplied) =>
        left is Identifier id && right is ArrayExpression arr
            ? new ArrayExpression([.. arr.Elements.Select(element => element is Literal lit
                ? (IExpression)new Literal(Cell(id.Name, lit.Value, fieldsApplied))
                : Rewrite(element, fieldsApplied))])
            : Rewrite(right, fieldsApplied);
}
