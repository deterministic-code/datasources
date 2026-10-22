using System.Collections;
using System.Reflection;

namespace Datasources;

public class CRUDRepository<T> : ReadonlyRepository<T>, ICRUDRepository<T> where T : new()
{
    readonly IWriteEnrichmentStrategy<T> write;
    readonly IReadOnlyList<string> ids;
    readonly CRUDRepositoryStatements crudStatements;
    readonly IRepositoryTree tree;

    public CRUDRepository(IConnection connection) :
        this(connection, CreateCRUDRepositoryOptions.FromType<T>()) { }

    public CRUDRepository(IConnection connection, CRUDRepositoryOptions<T> options) :
        this(Compose(connection, options)) { }

    static (BuiltRepository<T> Built, CRUDRepositoryStatements Statements, IWriteEnrichmentStrategy<T>? Write) Compose(
        IConnection connection,
        CRUDRepositoryOptions<T> options)
    {
        var builder = new CRUDRepositoryBuilder<T>();
        return (
            builder.Parts(connection, options),
            builder.CRUDStatements ?? throw new DatasourceException("Statements required"),
            options.Write);
    }

    private CRUDRepository((BuiltRepository<T> Built, CRUDRepositoryStatements Statements, IWriteEnrichmentStrategy<T>? Write) composed) :
        base(composed.Built)
    {
        ids = composed.Built.Ids;
        crudStatements = composed.Statements;
        write = composed.Write ?? new DefaultWriteEnrichmentStrategy<T>();
        tree = new RepositoryTree(new RepositoryTreeNode(this, ""));
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
        where TRelated : new()
    {
        base.AddEnriched(enrichment, via, strategy);
        if (via is ICRUDRepository<TRelated> crud)
            Last().Persist = new PersistEnrichment<T, TRelated>(crud);
        return this;
    }

    public IEnriched<T> AddLinked<TRelated, TLink>(
        Enrichment enrichment,
        IReadonlyRepository<TRelated> via,
        ICRUDRepository<TLink> link,
        IReadonlyEnrichmentStrategy<T>? strategy = null)
        where TLink : new()
    {
        var chosen = strategy ?? new DefaultReadonlyEnrichmentStrategy<T>();
        base.AddEnriched(enrichment, via, strategy);
        var stored = Last();
        stored.Read = new LinkedReadEnrichment<T, TRelated, TLink>(via, link, chosen);
        stored.Link = new LinkEnrichment<T, TLink>(link);
        return this;
    }

    StoredEnrichment<T> Last() => (StoredEnrichment<T>)((IEnrichmentHost)this).Enrichments[^1];

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
        await write.Persist(tree, [item!], WriteMode.Create);
        if (ids.Count != 1) return item;
        var id = IdValue(item);
        return id is Value.Null ? item : await ReadSingleById(id) ?? item;
    }

    async Task<Response> UpdateCore(T item, IWhere? byWhere = null)
    {
        var response = await Save(item, byWhere);
        if (Enriched)
            await write.Persist(tree, [item!], WriteMode.Update);
        return response;
    }

    async Task<Response> DeleteCore(T item, IWhere? byWhere = null)
    {
        if (Enriched)
            await write.Persist(tree, [item!], WriteMode.Delete);
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

    CRUDRepositoryStatements CRUDStatements => crudStatements;

    async Task<Response> Insert(T item)
    {
        var record = ReverseConvert(item);
        var fields = Params(record, col => !ids.Contains(col) || Assigned(record, col));
        var statement = CRUDStatements.Insert(fields);
        var response = await Command().Execute(statement, statement.Fields);
        if (ids.Count == 1 && fields.All(field => field.Name != ids[0]))
            await CopyInsertedId(item, statement.Fields);
        return response;
    }

    async Task CopyInsertedId(T target, IReadOnlyList<Parameter> fields)
    {
        var column = ids[0];
        var statement = CRUDStatements.SelectInsertedId(column);
        var sqlColumn = statement.Column;
        IStatement read = statement;
        var reader = await Command().ExecuteReader(read, read is IReturnStatement ? fields : null);
        try
        {
            if (!await reader.Next()) return;
            var row = Identified(Named(reader.Current ?? throw new DatasourceException("Reader current must be a row"), column, sqlColumn), column);
            var source = ObjectConverter.Convert(RowConverter?.Convert(row) ?? row);
            if (target is IDictionary dest && source is IDictionary src && src.Contains(column) && src[column] is { } cell && cell is not Value.Null)
                dest[column] = cell;
            else if (typeof(T).GetProperty(column) is { CanWrite: true } prop && prop.GetValue(source) is { } value)
                prop.SetValue(target, value);
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
        var fields = Params(record, col => !ids.Contains(col));
        if (fields.Count == 0)
            throw new DatasourceException("Update requires at least one non-Id field");
        var statement = CRUDStatements.Update(fields, byWhere ?? IdWhere(record), (name, value) => Cell(name, value, byWhere is null));
        return await Command().Execute(statement, statement.Parameters);
    }

    async Task<Response> Remove(T item, IWhere? byWhere)
    {
        var record = ReverseConvert(item);
        var statement = CRUDStatements.Delete(byWhere ?? IdWhere(record), (name, value) => Cell(name, value, byWhere is null));
        var parameters = statement.Where?.Parameters is { Count: > 0 } listed
            ? listed
            : DefaultStatementTextRenderer.Instance.Render(statement).Contains('$') ? Params(record, _ => true) : [];
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
        [.. record.Where(pair => include(pair.Key) && Writable(pair.Key) && !EnrichmentInto(pair.Key) && pair.Value is not Value.Null)
            .OrderBy(pair => pair.Key)
            .Select(pair => new Parameter(pair.Key, Cell(pair.Key, pair.Value, fieldsApplied: true)))];

    bool Writable(string col) =>
        crudStatements.ReadColumns.Count == 0
        || crudStatements.ReadColumns.Contains(col)
        || crudStatements.ReadColumns.Any(name => Column(name) == col);

    static bool Assigned(Row record, string col) =>
        record.TryGetValue(col, out var value) && value switch
        {
            Value.Null => false,
            Value.Text(var text) => text.Length > 0,
            Value.Int(0) => false,
            _ => true
        };

    protected bool EnrichmentInto(string name) =>
        ((IEnrichmentHost)this).Enrichments.Any(entry => entry.Enrichment.DestinationProperty == name);

    Row ReverseConvert(T item)
    {
        var row = ObjectConverter.ReverseConvert(item);
        return RowConverter?.ReverseConvert(row) ?? row;
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
