using System.Collections;
using System.Reflection;

namespace Datasources;

public class CRUDRepository<T> : ReadonlyRepository<T>, ICRUDRepository<T> where T : new()
{
    readonly IWriteEnrichmentStrategy<T> write;
    readonly IReadOnlyList<string> ids;
    readonly IMapper<Value, T>? create;

    ICRUDRepository<T>? Delegate => InnerRepo as ICRUDRepository<T>;

    public CRUDRepository(IConnection connection) :
        this(connection, CreateRepositoryOptions.FromType<T>()) { }

    public CRUDRepository(IConnection connection, CRUDRepositoryOptions<T> options) :
        this(RepositoryBuilder<T>.Parts(connection, options)) { }

    public CRUDRepository(
        ICRUDRepository<T> inner,
        IReadonlyEnrichmentStrategy<T>? strategy = null,
        IWriteEnrichmentStrategy<T>? write = null) : base(
            inner,
            strategy ?? new DefaultReadonlyEnrichmentStrategy<T>())
    {
        this.write = write ?? new DefaultWriteEnrichmentStrategy<T>();
        if (inner is CRUDRepository<T> crud)
        {
            ids = crud.ids;
            create = crud.create;
        }
        else
        {
            ids = CreateRepositoryOptions.FromType<T>().Ids;
            create = CRUDRepositoryOptions<T>.Identity(ids);
        }
    }

    public CRUDRepository(BuiltRepository<T> built) : base(
        built.Connection, built.RowConverter, built.RowsMapper, built.ColumnMappings, built.Statements)
    {
        create = built.Create;
        ids = built.Ids;
        write = new DefaultWriteEnrichmentStrategy<T>();
    }

    public IReadOnlyList<string> Ids => ids;

    public new CRUDRepository<T> AddMiddleware(ICommandMiddleware middleware)
    {
        base.AddMiddleware(middleware);
        return this;
    }

    public async Task<T> Create(T item)
    {
        var stored = Enriched ? StripEnriched(item) : item;
        var created = Delegate is null ? await Inserted(stored) : await Delegate.Create(stored);
        if (!Enriched) return created;
        CopyIds(item, created);
        await write.Persist(Tree, [item!], WriteMode.Create);
        if (ids.Count != 1) return item;
        var id = IdValue(item);
        return id is Value.Null ? item : await ReadSingleById(id) ?? item;
    }

    public async Task<Response> Update(T item, IWhere? byWhere = null)
    {
        var stored = Enriched ? StripEnriched(item) : item;
        var response = Delegate is null ? await Save(stored, byWhere) : await Delegate.Update(stored, byWhere);
        if (Enriched)
            await write.Persist(Tree, [item!], WriteMode.Update);
        return response;
    }

    public async Task<Response> Delete(T item, IWhere? byWhere = null)
    {
        if (Enriched)
            await write.Persist(Tree, [item!], WriteMode.Delete);
        return Delegate is null ? await Remove(item, byWhere) : await Delegate.Delete(item, byWhere);
    }

    public async Task<T?> ReadSingleById(Value id)
    {
        if (Delegate is null) return await ReadSingle(IdWhere(Single(id)));
        var item = await Delegate.ReadSingleById(id);
        if (item is null || !Enriched) return item;
        await Enrich([item]);
        return item;
    }

    public async Task<T?> ReadSingleById(Row key)
    {
        if (Delegate is not null)
        {
            var item = await Delegate.ReadSingleById(key);
            if (item is null || !Enriched) return item;
            await Enrich([item]);
            return item;
        }
        if (ids.Count == 0)
            throw new DatasourceException("ReadSingleById requires an Id column");
        return await ReadSingle(IdWhere(key));
    }

    async Task<T> Inserted(T item)
    {
        await Insert(item);
        return item;
    }

    async Task<Response> Insert(T item)
    {
        var fields = Params(ReverseConvert(item), col => !ids.Contains(col));
        var response = await Command().Execute(Statements.Insert(fields), fields);
        if (response.Result is not Value.Null && create is { } map)
            Apply(item, map.Map(response.Result));
        return response;
    }

    async Task<Response> Save(T item, IWhere? byWhere)
    {
        var record = ReverseConvert(item);
        var where = byWhere is null ? Stored(IdWhere(record), fieldsApplied: true) : Stored(byWhere);
        var fields = Params(record, col => !ids.Contains(col));
        if (fields.Count == 0)
            throw new DatasourceException("Update requires at least one non-Id field");
        var statement = Statements.Update(fields, where);
        return await Command().Execute(statement, statement.Parameters);
    }

    async Task<Response> Remove(T item, IWhere? byWhere)
    {
        var record = ReverseConvert(item);
        var where = byWhere is null ? Stored(IdWhere(record), fieldsApplied: true) : Stored(byWhere);
        var statement = Statements.Delete(where);
        var parameters = where.Parameters is { Count: > 0 } listed
            ? listed
            : statement.GetText().Contains('$') ? Params(record, _ => true) : [];
        return await Command().Execute(statement, parameters);
    }

    Row Single(Value id) =>
        ids.Count switch
        {
            1 => new Row { [ids.Single()] = id },
            0 => throw new DatasourceException("ReadSingleById requires an Id column"),
            _ => throw new DatasourceException("Composite id requires a row keyed by each Id column")
        };

    IWhere IdWhere(Row record) =>
        new Where(ids.Select(col =>
            record.TryGetValue(col, out var value)
                ? (IExpression)new Operator.Equals(col, value)
                : throw new DatasourceException($"Missing Id value \"{col}\"")
        ).DefaultIfEmpty(new Operator.Equals(new Literal(1), new Literal(1)))
         .Aggregate((left, right) => new Operator.And(left, right)));

    IReadOnlyList<Parameter> Params(Row record, Func<string, bool> include) =>
        [.. record.Where(pair => include(pair.Key)).OrderBy(pair => pair.Key)
            .Select(pair => new Parameter(Column(pair.Key), Cell(pair.Key, pair.Value, fieldsApplied: true)))];

    Row ReverseConvert(T item) => RowConverter.ReverseConvert(Mapper!.ReverseMap(item));

    void CopyIds(T item, T created)
    {
        if (ReferenceEquals(item, created)) return;
        if (item is IDictionary dest && created is IDictionary src)
        {
            foreach (var name in ids)
                if (src.Contains(name))
                    dest[name] = src[name];
            return;
        }
        var unset = new T();
        foreach (var name in ids)
        {
            var prop = typeof(T).GetProperty(name);
            if (prop is not { CanRead: true, CanWrite: true }) continue;
            var value = prop.GetValue(created);
            if (!Equals(value, prop.GetValue(unset)))
                prop.SetValue(item, value);
        }
    }

    Value IdValue(T item)
    {
        var name = ids[0];
        if (item is IDictionary dict)
            return dict.Contains(name) ? Value.From(dict[name]) : Value.Null.Instance;
        return typeof(T).GetProperty(name)?.GetValue(item) is { } value
            ? Value.From(value)
            : Value.Null.Instance;
    }

    static void Apply(T item, T identity)
    {
        if (item is IDictionary dest && identity is IDictionary src)
        {
            foreach (DictionaryEntry entry in src)
                dest[entry.Key] = entry.Value;
            return;
        }
        var unset = new T();
        foreach (var prop in typeof(T).GetProperties(BindingFlags.Public | BindingFlags.Instance)
            .Where(prop => prop is { CanRead: true, CanWrite: true } && prop.GetIndexParameters().Length == 0))
        {
            var value = prop.GetValue(identity);
            if (!Equals(value, prop.GetValue(unset)))
                prop.SetValue(item, value);
        }
    }

    T StripEnriched(T item) => (T)Strip(Tree.Parent, item!);

    static object Strip(IRepositoryTreeNode node, object item)
    {
        var skip = node.Items
            .Select(child => child.Enrichment?.Into)
            .OfType<string>()
            .ToHashSet();
        if (item is IDictionary<string, Value> row)
        {
            var copy = new Row(row);
            foreach (var name in skip)
                copy.Remove(name);
            return copy;
        }
        var dest = Activator.CreateInstance(item.GetType())!;
        foreach (var prop in item.GetType().GetProperties(BindingFlags.Public | BindingFlags.Instance)
            .Where(prop => prop is { CanRead: true, CanWrite: true } && prop.GetIndexParameters().Length == 0))
        {
            if (skip.Contains(prop.Name))
            {
                if (!prop.PropertyType.IsValueType)
                    prop.SetValue(dest, null);
                continue;
            }
            prop.SetValue(dest, prop.GetValue(item));
        }
        return dest;
    }
}
