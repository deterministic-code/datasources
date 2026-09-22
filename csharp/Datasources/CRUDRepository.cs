namespace Datasources;

public sealed class CRUDRepository<T> : ReadonlyRepository<T>, ICRUDRepository<T> where T : new()
{
    readonly HashSet<string> ids;

    public CRUDRepository(IConnection connection) :
        this(CreateRepositoryBuilderOptions.FromType<T>(connection)) { }

    public CRUDRepository(RepositoryBuilderOptions<T> options) :
        this(RepositoryBuilder<T>.Parts(options)) { }

    public CRUDMapper<T> CRUDMapper { get; }

    public CRUDRepository(BuiltRepository<T> built) : base(
        built.Connection, built.RowConverter, built.CRUDMapper.Rows, built.Statements)
    {
        CRUDMapper = built.CRUDMapper;
        ids = new(built.Ids);
    }

    public new CRUDRepository<T> AddMiddleware(ICommandMiddleware middleware)
    {
        base.AddMiddleware(middleware);
        return this;
    }

    public async Task<Response> Create(T item)
    {
        var fields = Params(ReverseConvert(item), col => !ids.Contains(col));
        var response = await Command().Execute(Statements.Insert(fields), fields);
        if (response.Result is not Value.Null)
            Apply(item, CRUDMapper.Create.Map(response.Result));
        return response;
    }

    public async Task<Response> Update(T item, IWhere? byWhere = null)
    {
        var record = ReverseConvert(item);
        var where = byWhere ?? IdWhere(record);
        var fields = Params(record, col => !ids.Contains(col));
        if (fields.Count == 0)
            throw new DatasourceException("Update requires at least one non-Id field");
        return await Command().Execute(
            Statements.Update(fields, where),
            [.. fields, .. where.Parameters]);
    }

    public async Task<Response> Delete(T item, IWhere? byWhere = null)
    {
        var record = ReverseConvert(item);
        var where = byWhere ?? IdWhere(record);
        return await Command().Execute(
            Statements.Delete(where),
            where.Parameters is { Count: > 0 } parameters ? parameters : Params(record, _ => true));
    }

    IWhere IdWhere(Row record) =>
        new WhereClause(ids.Select(col =>
            record.TryGetValue(col, out var value)
                ? (IExpression)new Operator.Equals(col, value)
                : throw new DatasourceException($"Missing Id value \"{col}\"")
        ).DefaultIfEmpty(new Operator.Equals(new Literal(1), new Literal(1)))
         .Aggregate((left, right) => new Operator.And(left, right)));

    static IReadOnlyList<Parameter> Params(
        Row record, Func<string, bool> include) =>
        record.Where(pair => include(pair.Key)).OrderBy(pair => pair.Key)
            .Select(pair => new Parameter(pair.Key, pair.Value)).ToArray();

    Row ReverseConvert(T item) => RowConverter.ReverseConvert(Mapper!.ReverseMap(item));

    static void Apply(T item, T identity)
    {
        if (item is System.Collections.IDictionary dest && identity is System.Collections.IDictionary src)
        {
            foreach (System.Collections.DictionaryEntry entry in src)
                dest[entry.Key] = entry.Value;
            return;
        }
        var unset = new T();
        foreach (var prop in typeof(T).GetProperties(System.Reflection.BindingFlags.Public | System.Reflection.BindingFlags.Instance)
            .Where(prop => prop is { CanRead: true, CanWrite: true } && prop.GetIndexParameters().Length == 0))
        {
            var value = prop.GetValue(identity);
            if (!Equals(value, prop.GetValue(unset)))
                prop.SetValue(item, value);
        }
    }
}
