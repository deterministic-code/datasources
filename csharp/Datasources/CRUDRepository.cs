using System.Collections;
using System.Reflection;

namespace Datasources;

public class CRUDRepository<T> : ReadonlyRepository<T>, ICRUDRepository<T> where T : new()
{
    readonly IWriteEnrichmentStrategy<T> write;
    readonly IReadOnlyList<string> ids;
    readonly IMapper<IReturnStatement, IReturnStatement>? returnMapper;

    public CRUDRepository(IConnection connection) :
        this(connection, CreateCRUDRepositoryOptions.FromType<T>()) { }

    public CRUDRepository(IConnection connection, CRUDRepositoryOptions<T> options) :
        this(RepositoryBuilder<T>.Parts(connection, options), options.Write, options.Return) { }

    private CRUDRepository(
        BuiltRepository<T> built,
        IWriteEnrichmentStrategy<T>? write = null,
        IMapper<IReturnStatement, IReturnStatement>? returnMapper = null) : base(
        built.Connection, built.RowConverter, built.ObjectConverter, built.RowsMapper, built.ColumnMappings, built.Statements)
    {
        ids = built.Ids;
        this.write = write ?? new DefaultWriteEnrichmentStrategy<T>();
        this.returnMapper = returnMapper;
    }

    public IReadOnlyList<string> Ids => ids;

    bool UseTransaction => write.UseTransaction;

    public new CRUDRepository<T> AddMiddleware(ICommandMiddleware middleware)
    {
        base.AddMiddleware(middleware);
        return this;
    }

    public new IEnriched<T> AddEnriched<TRelated>(
        Enrichment enrichment,
        IReadonlyRepository<TRelated> via,
        IReadonlyEnrichmentStrategy<T>? strategy = null)
    {
        base.AddEnriched(enrichment, via, strategy);
        if (via is ICRUDRepository<TRelated> crud)
            ((IEnrichmentHost)this).Enrichments[^1].AttachPersist(ChildPersist.For(crud));
        return this;
    }

    public IEnriched<T> AddLinked<TRelated, TLink>(
        Enrichment enrichment,
        IReadonlyRepository<TRelated> via,
        ICRUDRepository<TLink> link,
        IReadonlyEnrichmentStrategy<T>? strategy = null)
    {
        var chosen = strategy ?? new DefaultReadonlyEnrichmentStrategy<T>();
        base.AddEnriched(enrichment, via, strategy);
        ((IEnrichmentHost)this).Enrichments[^1].AttachLink(
            (parents, sources) => chosen.ApplyLinked(via, link, enrichment, parents, sources),
            ChildPersist.For(link));
        return this;
    }

    public Task<T> Create(T item) => InTransaction(() => CreateCore(item));

    public Task<Response> Update(T item, IWhere? byWhere = null) =>
        InTransaction(() => UpdateCore(item, byWhere));

    public Task<Response> Delete(T item, IWhere? byWhere = null) =>
        InTransaction(() => DeleteCore(item, byWhere));

    async Task<TResult> InTransaction<TResult>(Func<Task<TResult>> write)
    {
        if (!UseTransaction) return await write();
        var work = new UnitOfWorkTransaction(Command().Connection);
        await work.Begin();
        try
        {
            var result = await write();
            await work.Commit();
            return result;
        }
        catch
        {
            await work.Abort();
            throw;
        }
    }

    async Task<T> CreateCore(T item)
    {
        await Insert(item);
        if (!Enriched) return item;
        await write.Persist(Tree, [item!], WriteMode.Create);
        if (ids.Count != 1) return item;
        var id = IdValue(item);
        return id is Value.Null ? item : await ReadSingleById(id) ?? item;
    }

    async Task<Response> UpdateCore(T item, IWhere? byWhere = null)
    {
        var response = await Save(item, byWhere);
        if (Enriched)
            await write.Persist(Tree, [item!], WriteMode.Update);
        return response;
    }

    async Task<Response> DeleteCore(T item, IWhere? byWhere = null)
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

    async Task<Response> Insert(T item)
    {
        var fields = Params(ReverseConvert(item), col => !ids.Contains(col));
        var response = await Command().Execute(Statements.Insert(fields), fields);
        if (ids.Count == 1)
            await CopyInsertedId(item, fields);
        return response;
    }

    async Task CopyInsertedId(T target, IReadOnlyList<Parameter> fields)
    {
        var column = ids[0];
        var sqlColumn = Column(column);
        IStatement statement = returnMapper is { } map
            ? map.Map(new ReturnStatement(sqlColumn, fields))
            : Statements.SelectInsertedId(sqlColumn);
        var reader = await Command().ExecuteReader(statement, statement is IReturnStatement ? fields : null);
        try
        {
            if (!await reader.Next()) return;
            var row = Identified(Named(reader.Current ?? throw new DatasourceException("Reader current must be a row"), column, sqlColumn), column);
            WriteId(target, IdValue(Mapper!.Map(ObjectConverter.Convert(RowConverter?.Convert(row) ?? row))));
        }
        finally
        {
            if (reader is IAsyncDisposable disposable)
                await disposable.DisposeAsync();
        }
    }

    static Row Named(Row row, string property, string sqlColumn)
    {
        if (property == sqlColumn || !row.TryGetValue(sqlColumn, out var cell)) return row;
        var named = new Row(row.Where(pair => pair.Key != sqlColumn));
        named[property] = cell;
        return named;
    }

    static Row Identified(Row row, string column) =>
        row.ContainsKey(column) || row.Count != 1
            ? row
            : new Row { [column] = row.First().Value };

    async Task<Response> Save(T item, IWhere? byWhere)
    {
        var record = ReverseConvert(item);
        var where = byWhere is null ? Mapped(IdWhere(record), fieldsApplied: true) : Mapped(byWhere, fieldsApplied: false);
        var fields = Params(record, col => !ids.Contains(col));
        if (fields.Count == 0)
            throw new DatasourceException("Update requires at least one non-Id field");
        var statement = Statements.Update(fields, where);
        return await Command().Execute(statement, UpdateStatement.ParameterList(fields, where));
    }

    async Task<Response> Remove(T item, IWhere? byWhere)
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
        [.. record.Where(pair => include(pair.Key) && !EnrichmentInto(pair.Key) && pair.Value is not Value.Null)
            .OrderBy(pair => pair.Key)
            .Select(pair => new Parameter(Column(pair.Key), Cell(pair.Key, pair.Value, fieldsApplied: true)))];

    protected bool EnrichmentInto(string name) =>
        ((IEnrichmentHost)this).Enrichments.Any(entry => entry.Enrichment.Into == name);

    Row ReverseConvert(T item)
    {
        var row = ObjectConverter.ReverseConvert(Mapper!.ReverseMap(item));
        return RowConverter?.ReverseConvert(row) ?? row;
    }

    void WriteId(T target, Value id)
    {
        if (ids.Count != 1 || id is Value.Null) return;
        var name = ids[0];
        var row = new Row { [name] = id };
        var patch = ObjectConverter.Convert(RowConverter?.Convert(row) ?? row);
        if (target is IDictionary dest && patch is IDictionary src && src.Contains(name))
        {
            dest[name] = src[name];
            return;
        }
        var prop = typeof(T).GetProperty(name);
        if (prop is { CanWrite: true })
            prop.SetValue(target, prop.GetValue(patch));
    }

    Value IdValue(T item)
    {
        if (ids.Count != 1) return Value.Null.Instance;
        var name = ids[0];
        if (item is IDictionary dict)
            return dict.Contains(name) ? Value.From(dict[name]) : Value.Null.Instance;
        return typeof(T).GetProperty(name)?.GetValue(item) is { } value
            ? Value.From(value)
            : Value.Null.Instance;
    }
}
