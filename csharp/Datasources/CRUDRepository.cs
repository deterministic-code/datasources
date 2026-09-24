using System.Collections;
using System.Reflection;

namespace Datasources;

public class CRUDRepository<T> : ReadonlyRepository<T>, ICRUDRepository<T>, IEnriched<T> where T : new()
{
    readonly ICRUDRepository<T>? inner;
    readonly IReadonlyEnrichmentStrategy<T> strategy;
    readonly IReadOnlyList<string> idNames;
    readonly HashSet<string> ids;
    readonly IMapper<Value, T>? create;

    public IRepositoryTree Tree { get; }

    public CRUDRepository(IConnection connection) :
        this(connection, CreateRepositoryOptions.FromType<T>()) { }

    public CRUDRepository(IConnection connection, CRUDRepositoryOptions<T> options) :
        this(RepositoryBuilder<T>.Parts(connection, options)) { }

    public CRUDRepository(ICRUDRepository<T> inner, IReadonlyEnrichmentStrategy<T>? strategy = null)
    {
        this.inner = inner;
        this.strategy = strategy ?? new DefaultReadonlyEnrichmentStrategy<T>();
        Tree = new RepositoryTree(new RepositoryTreeNode(inner, "", binder: this.strategy));
        if (inner is CRUDRepository<T> crud)
        {
            idNames = crud.idNames;
            create = crud.create;
        }
        else
        {
            idNames = CreateRepositoryOptions.FromType<T>().Ids;
            create = CRUDRepositoryOptions<T>.Identity(idNames);
        }
        ids = [.. idNames];
    }

    public CRUDRepository(BuiltRepository<T> built) : base(
        built.Connection, built.RowConverter, built.RowsMapper, built.ColumnMappings, built.Statements)
    {
        create = built.Create;
        idNames = built.Ids;
        ids = [.. built.Ids];
        strategy = new DefaultReadonlyEnrichmentStrategy<T>();
        Tree = new RepositoryTree(new RepositoryTreeNode(this, "", binder: strategy));
    }

    bool Enriched => Tree.Parent.Items.Count > 0;

    public IReadOnlyList<string> Ids => idNames;

    public new CRUDRepository<T> AddMiddleware(ICommandMiddleware middleware)
    {
        base.AddMiddleware(middleware);
        return this;
    }

    public IEnriched<T> AddEnriched<TRelated>(
        string into,
        string from,
        IReadonlyRepository<TRelated> via,
        string by,
        string? select = null,
        bool many = false,
        Action<IRepositoryTreeNode>? nest = null)
    {
        var child = Tree.Parent.Include(via, into, from, by, select, many);
        nest?.Invoke(child);
        return this;
    }

    public Task Enrich(IList<T> items) => strategy.Enrich(Tree, items);

    public override IAsyncEnumerable<T> Read(IStatement? query = null) =>
        inner is null ? base.Read(query) : Rows(ReadTracked(query));

    public override IAsyncEnumerable<T> Read(IWhere where) =>
        inner is null ? base.Read(where) : Rows(ReadTracked(where));

    public override async Task<T?> ReadSingle(IWhere where)
    {
        if (inner is null) return await base.ReadSingle(where);
        await foreach (var item in Read(where))
            return item;
        return default;
    }

    public override async Task<T?> ReadSingle(IStatement? query = null)
    {
        if (inner is null) return await base.ReadSingle(query);
        await foreach (var item in Read(query))
            return item;
        return default;
    }

    protected internal override IAsyncEnumerable<(T Item, Row? Source)> ReadTracked(IStatement? query = null) =>
        Enriched ? Sourced(Source(query)) : Source(query);

    protected internal override IAsyncEnumerable<(T Item, Row? Source)> ReadTracked(IWhere where) =>
        Enriched ? Sourced(Source(where)) : Source(where);

    public async Task<Response> Create(T item)
    {
        var stored = Enriched ? StripEnriched(item) : item;
        var response = inner is null ? await Insert(stored) : await inner.Create(stored);
        if (!Enriched) return response;
        ApplyGenerated(item, response.Result);
        await Persist(Tree.Parent, [item!], Mode.Create);
        return response;
    }

    public async Task<Response> Update(T item, IWhere? byWhere = null)
    {
        var stored = Enriched ? StripEnriched(item) : item;
        var response = inner is null ? await Save(stored, byWhere) : await inner.Update(stored, byWhere);
        if (Enriched)
            await Persist(Tree.Parent, [item!], Mode.Update);
        return response;
    }

    public async Task<Response> Delete(T item, IWhere? byWhere = null)
    {
        if (Enriched)
            await Persist(Tree.Parent, [item!], Mode.Delete);
        return inner is null ? await Remove(item, byWhere) : await inner.Delete(item, byWhere);
    }

    public async Task<T?> ReadSingleById(Value id)
    {
        if (inner is null) return await ReadSingle(IdWhere(Single(id)));
        var item = await inner.ReadSingleById(id);
        if (item is null || !Enriched) return item;
        await Enrich([item]);
        return item;
    }

    public async Task<T?> ReadSingleById(Row key)
    {
        if (inner is not null)
        {
            var item = await inner.ReadSingleById(key);
            if (item is null || !Enriched) return item;
            await Enrich([item]);
            return item;
        }
        if (ids.Count == 0)
            throw new DatasourceException("ReadSingleById requires an Id column");
        return await ReadSingle(IdWhere(key));
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
        var (text, parameters) = Distinct(statement.GetText(), fields, where.Parameters);
        return await Command().Execute(text, parameters);
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

    IAsyncEnumerable<(T Item, Row? Source)> Source(IStatement? query) =>
        inner is null ? base.ReadTracked(query) : Tracked(inner, query);

    IAsyncEnumerable<(T Item, Row? Source)> Source(IWhere where) =>
        inner is null ? base.ReadTracked(where) : Tracked(inner, where);

    async IAsyncEnumerable<(T Item, Row? Source)> Sourced(IAsyncEnumerable<(T Item, Row? Source)> pairs)
    {
        var items = new List<T>();
        var sources = new List<Row?>();
        await foreach (var (item, source) in pairs)
        {
            items.Add(item);
            sources.Add(source);
        }
        await strategy.Enrich(Tree, items, sources);
        for (var i = 0; i < items.Count; i++)
            yield return (items[i], sources[i]);
    }

    static async IAsyncEnumerable<T> Rows(IAsyncEnumerable<(T Item, Row? Source)> pairs)
    {
        await foreach (var (item, _) in pairs)
            yield return item;
    }

    Row Single(Value id) =>
        ids.Count switch
        {
            1 => new Row { [ids.Single()] = id },
            0 => throw new DatasourceException("ReadSingleById requires an Id column"),
            _ => throw new DatasourceException("Composite id requires a row keyed by each Id column")
        };

    IWhere IdWhere(Row record) =>
        new WhereClause(ids.Select(col =>
            record.TryGetValue(col, out var value)
                ? (IExpression)new Operator.Equals(col, value)
                : throw new DatasourceException($"Missing Id value \"{col}\"")
        ).DefaultIfEmpty(new Operator.Equals(new Literal(1), new Literal(1)))
         .Aggregate((left, right) => new Operator.And(left, right)));

    IReadOnlyList<Parameter> Params(Row record, Func<string, bool> include) =>
        [.. record.Where(pair => include(pair.Key)).OrderBy(pair => pair.Key)
            .Select(pair => new Parameter(Column(pair.Key), Cell(pair.Key, pair.Value, fieldsApplied: true)))];

    Row ReverseConvert(T item) => RowConverter.ReverseConvert(Mapper!.ReverseMap(item));

    void ApplyGenerated(T item, Value result)
    {
        if (result is Value.Null || create is not { } map) return;
        var identity = map.Map(result);
        if (item is IDictionary dest && identity is IDictionary src)
        {
            foreach (var name in idNames)
                if (src.Contains(name))
                    dest[name] = src[name];
            return;
        }
        var unset = new T();
        foreach (var name in idNames)
        {
            var prop = typeof(T).GetProperty(name);
            if (prop is not { CanRead: true, CanWrite: true }) continue;
            var value = prop.GetValue(identity);
            if (!Equals(value, prop.GetValue(unset)))
                prop.SetValue(item, value);
        }
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

    async Task Persist(IRepositoryTreeNode node, IReadOnlyList<object> parents, Mode mode)
    {
        foreach (var child in node.Items)
            await PersistNode(child, parents, mode);
    }

    async Task PersistNode(IRepositoryTreeNode child, IReadOnlyList<object> parents, Mode mode)
    {
        if (child.Enrichment is not { Many: true, Select: null } enrichment)
            return;
        var writer = (child as RepositoryTreeNode)?.Persist
            ?? throw new DatasourceException(
                $"Deep write requires a CRUD repository for \"{enrichment.Into}\"");
        foreach (var parent in parents)
            await (mode switch
            {
                Mode.Create => CreateChildren(child, writer, enrichment, parent),
                Mode.Update => SyncChildren(child, writer, enrichment, parent),
                Mode.Delete => DeleteChildren(child, writer, enrichment, parent),
                _ => throw new DatasourceException("Unknown persist mode")
            });
    }

    async Task CreateChildren(
        IRepositoryTreeNode node,
        RepositoryTreeNode.IChildPersist writer,
        Enrichment enrichment,
        object parent)
    {
        foreach (var related in Payload(parent, enrichment.Into))
        {
            Stamp(related, enrichment.By, Get(parent, enrichment.From));
            writer.AssignIdentity(related, await writer.Create(Strip(node, related)));
            await Persist(node, [related], Mode.Create);
        }
    }

    async Task SyncChildren(
        IRepositoryTreeNode node,
        RepositoryTreeNode.IChildPersist writer,
        Enrichment enrichment,
        object parent)
    {
        var existing = await Load(writer, enrichment, parent);
        var payload = Payload(parent, enrichment.Into);
        foreach (var related in payload)
            Stamp(related, enrichment.By, Get(parent, enrichment.From));
        var keys = writer.Ids;
        var keep = payload.Where(row => HasKey(row, keys)).Select(row => KeyOf(row, keys)).ToHashSet();
        foreach (var row in existing.Where(row => !keep.Contains(KeyOf(row, keys))))
        {
            await Persist(node, [row], Mode.Delete);
            await writer.Delete(row);
        }
        foreach (var related in payload)
        {
            if (HasKey(related, keys) && existing.Any(row => KeyOf(row, keys) == KeyOf(related, keys)))
                await writer.Update(Strip(node, related));
            else
                writer.AssignIdentity(related, await writer.Create(Strip(node, related)));
            await Persist(node, [related], Mode.Update);
        }
    }

    async Task DeleteChildren(
        IRepositoryTreeNode node,
        RepositoryTreeNode.IChildPersist writer,
        Enrichment enrichment,
        object parent)
    {
        var existing = await Load(writer, enrichment, parent);
        await Persist(node, existing, Mode.Delete);
        foreach (var row in existing)
            await writer.Delete(row);
    }

    static async Task<List<object>> Load(
        RepositoryTreeNode.IChildPersist writer,
        Enrichment enrichment,
        object parent)
    {
        var key = Get(parent, enrichment.From);
        if (key is null) return [];
        var items = new List<object>();
        await foreach (var item in writer.Read(new WhereClause(new Operator.Equals(enrichment.By, Value.From(key)))))
            items.Add(item);
        return items;
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

    static IReadOnlyList<object> Payload(object parent, string into) =>
        Get(parent, into) switch
        {
            null or string => [],
            IEnumerable rows => [.. rows.Cast<object>().Where(item => item is not null and not string)],
            _ => []
        };

    static void Stamp(object related, string by, object? key)
    {
        if (key is not null)
            Set(related, by, key);
    }

    static bool HasKey(object item, IReadOnlyList<string> keys) =>
        keys.Count > 0 && keys.All(name => Get(item, name) is not null and not 0 and not 0L and not 0u and not "");

    static string KeyOf(object item, IReadOnlyList<string> keys) =>
        string.Join("\u001f", keys.Select(name => Get(item, name)?.ToString() ?? ""));

    static object? Get(object obj, string name) => Prop(obj, name).GetValue(obj);

    static void Set(object obj, string name, object? value)
    {
        var prop = Prop(obj, name);
        var target = Nullable.GetUnderlyingType(prop.PropertyType) ?? prop.PropertyType;
        prop.SetValue(obj, value is null || target.IsInstanceOfType(value)
            ? value
            : Convert.ChangeType(value, target));
    }

    static PropertyInfo Prop(object obj, string name) =>
        obj.GetType().GetProperty(name)
        ?? throw new DatasourceException($"Unknown property \"{name}\"");

    static (string Text, List<Parameter> Parameters) Distinct(
        string text, IReadOnlyList<Parameter> fields, IReadOnlyList<Parameter> where)
    {
        var split = text.LastIndexOf(" WHERE ", StringComparison.Ordinal);
        var head = split < 0 ? text : text[..split];
        var tail = split < 0 ? "" : text[split..];
        var bound = fields.ToList();
        var values = fields.GroupBy(field => field.Name).ToDictionary(group => group.Key, group => group.Last().Value);
        foreach (var parameter in where)
            if (values.TryGetValue(parameter.Name, out var current) && !Equals(current, parameter.Value))
            {
                var alias = $"{parameter.Name}__where";
                tail = Alias(tail, parameter.Name, alias);
                bound.Add(new Parameter(alias, parameter.Value));
            }
            else
                bound.Add(parameter);
        return (head + tail, bound);
    }

    static string Alias(string text, string name, string alias)
    {
        var token = "$" + name;
        var next = "$" + alias;
        var index = 0;
        var result = new System.Text.StringBuilder();
        while (text.IndexOf(token, index, StringComparison.Ordinal) is var at && at >= 0)
        {
            var end = at + token.Length;
            var boundary = end >= text.Length || !char.IsAsciiLetterOrDigit(text[end]) && text[end] != '_';
            result.Append(text[index..at]).Append(boundary ? next : token);
            index = end;
        }
        return result.Append(text[index..]).ToString();
    }

    enum Mode { Create, Update, Delete }
}
