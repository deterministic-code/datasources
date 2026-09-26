using System.Collections;
using System.Reflection;

namespace Datasources;

public class CRUDRepository<T> : ReadonlyRepository<T>, ICRUDRepository<T> where T : new()
{
    readonly IWriteEnrichmentStrategy<T> write;
    readonly IReadOnlyList<string> ids;
    readonly IMapper<Value, T>? create;

    public CRUDRepository(IConnection connection) :
        this(connection, CreateRepositoryOptions.FromType<T>()) { }

    public CRUDRepository(IConnection connection, CRUDRepositoryOptions<T> options) :
        this(RepositoryBuilder<T>.Parts(connection, options), options.Write) { }

    public CRUDRepository(BuiltRepository<T> built, IWriteEnrichmentStrategy<T>? write = null) : base(
        built.Connection, built.RowConverter, built.RowsMapper, built.ColumnMappings, built.Statements)
    {
        create = built.Create;
        ids = built.Ids;
        this.write = write ?? new DefaultWriteEnrichmentStrategy<T>();
    }

    protected CRUDRepository(IWriteEnrichmentStrategy<T>? write = null)
    {
        ids = CreateRepositoryOptions.FromType<T>().Ids;
        create = CRUDRepositoryOptions<T>.Identity(ids);
        this.write = write ?? new DefaultWriteEnrichmentStrategy<T>();
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
        var created = await Inserted(stored);
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
        var response = await Save(stored, byWhere);
        if (Enriched)
            await write.Persist(Tree, [item!], WriteMode.Update);
        return response;
    }

    public async Task<Response> Delete(T item, IWhere? byWhere = null)
    {
        if (Enriched)
            await write.Persist(Tree, [item!], WriteMode.Delete);
        return await Remove(item, byWhere);
    }

    public Task<T?> ReadSingleById(Value id) =>
        ReadSingle(IdWhere(Single(id)));

    public async Task<T?> ReadSingleById(Row key)
    {
        if (ids.Count == 0)
            throw new DatasourceException("ReadSingleById requires an Id column");
        return await ReadSingle(IdWhere(key));
    }

    protected virtual async Task<T> Inserted(T item)
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

    protected virtual async Task<Response> Save(T item, IWhere? byWhere)
    {
        var record = ReverseConvert(item);
        var where = byWhere is null ? Mapped(IdWhere(record), fieldsApplied: true) : Mapped(byWhere, fieldsApplied: false);
        var fields = Params(record, col => !ids.Contains(col));
        if (fields.Count == 0)
            throw new DatasourceException("Update requires at least one non-Id field");
        var statement = Statements.Update(fields, where);
        return await Command().Execute(statement, UpdateStatement.ParameterList(fields, where));
    }

    protected virtual async Task<Response> Remove(T item, IWhere? byWhere)
    {
        var record = ReverseConvert(item);
        var where = byWhere is null ? Mapped(IdWhere(record), fieldsApplied: true) : Mapped(byWhere, fieldsApplied: false);
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

    IWhere Mapped(IWhere where, bool fieldsApplied) =>
        where is Where clause
            ? clause.Map(Column, (name, cell) => Cell(name, cell, fieldsApplied))
            : where;

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

    T StripEnriched(T item) => (T)DefaultWriteEnrichmentStrategy<T>.Strip(this, item!);
}
