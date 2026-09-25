using System.Collections;
using System.Reflection;

namespace Datasources;

public sealed class RepositoryTreeNode : IRepositoryTreeNode
{
    readonly List<RepositoryTreeNode> items = [];
    readonly Func<IReadOnlyList<object>, IReadOnlyList<Row?>?, Task> apply;
    readonly IReadonlyEnrichmentBinder binder;

    public RepositoryTreeNode(
        object value,
        string propertyName,
        IRepositoryTreeNode? parent = null,
        Func<IReadOnlyList<object>, Task>? apply = null,
        Enrichment? enrichment = null,
        IReadonlyEnrichmentBinder? binder = null) :
        this(value, propertyName, parent, (parents, _) => (apply ?? (_ => Task.CompletedTask))(parents), enrichment, binder)
    {
    }

    internal RepositoryTreeNode(
        object value,
        string propertyName,
        IRepositoryTreeNode? parent,
        Func<IReadOnlyList<object>, IReadOnlyList<Row?>?, Task> apply,
        Enrichment? enrichment,
        IReadonlyEnrichmentBinder? binder)
    {
        Value = value;
        PropertyName = propertyName;
        Parent = parent;
        Enrichment = enrichment;
        this.binder = binder ?? new DefaultReadonlyEnrichmentStrategy<object>();
        this.apply = apply;
    }

    public object Value { get; }
    public string PropertyName { get; }
    public IRepositoryTreeNode? Parent { get; }
    public IReadOnlyList<IRepositoryTreeNode> Items => items;
    public Enrichment? Enrichment { get; }
    internal IChildPersist? Persist { get; private set; }
    internal IChildPersist? Link { get; private set; }

    public IRepositoryTreeNode Include<TRelated>(
        IReadonlyRepository<TRelated> via,
        string into,
        string from,
        string by,
        string? select = null,
        bool many = false) =>
        Include(via, into, from, by, select, many, binder);

    internal RepositoryTreeNode Include<TRelated>(
        IReadonlyRepository<TRelated> via,
        string into,
        string from,
        string by,
        string? select,
        bool many,
        IReadonlyEnrichmentBinder strategy)
    {
        var enrichment = new Enrichment(into, from, by, select, many);
        var child = new RepositoryTreeNode(
            via,
            into,
            this,
            (parents, sources) => ((ITrackedApply)strategy).ApplyTracked(via, enrichment, parents, sources),
            enrichment,
            strategy)
        {
            Persist = via is ICRUDRepository<TRelated> crud ? new ChildPersist<TRelated>(crud) : null
        };
        items.Add(child);
        return child;
    }

    public IRepositoryTreeNode IncludeLinked<TRelated, TLink>(
        IReadonlyRepository<TRelated> via,
        string into,
        string from,
        string by,
        ICRUDRepository<TLink> link,
        string parent,
        string related) =>
        IncludeLinked(via, into, from, by, link, parent, related, binder);

    internal RepositoryTreeNode IncludeLinked<TRelated, TLink>(
        IReadonlyRepository<TRelated> via,
        string into,
        string from,
        string by,
        ICRUDRepository<TLink> link,
        string parent,
        string related,
        IReadonlyEnrichmentBinder strategy)
    {
        var enrichment = new Enrichment(into, from, by, null, true, new Association(parent, related));
        var writer = new ChildPersist<TLink>(link);
        var child = new RepositoryTreeNode(
            via,
            into,
            this,
            (parents, sources) => ApplyLinked(via, writer, enrichment, parents, sources),
            enrichment,
            strategy)
        {
            Link = writer
        };
        items.Add(child);
        return child;
    }

    public Task EnrichFrom(IReadOnlyList<object> parents) => EnrichFrom(parents, null);

    internal async Task EnrichFrom(IReadOnlyList<object> parents, IReadOnlyList<Row?>? sources)
    {
        await apply(parents, sources);
        var next = Related(parents, PropertyName);
        var nested = PropertyName.Length == 0 ? sources : null;
        foreach (var child in items)
            await child.EnrichFrom(next, nested);
    }

    internal static object? SourceKey(object parent, string name, Row? source) =>
        source is not null && source.TryGetValue(name, out var cell) && cell is not global::Datasources.Value.Null
            ? cell.ToObject()
            : parent.GetType().GetProperty(name)?.GetValue(parent);

    internal static string KeyOf(object? key) => key?.ToString() ?? "";

    internal static IReadOnlyList<object> Related(IReadOnlyList<object> parents, string propertyName)
    {
        if (propertyName.Length == 0) return parents;
        var next = new List<object>();
        foreach (var parent in parents)
            switch (Get(parent, propertyName))
            {
                case null or string: break;
                case IEnumerable rows:
                    foreach (var item in rows)
                        if (item is not null and not string) next.Add(item);
                    break;
                case { } value when !value.GetType().IsValueType:
                    next.Add(value);
                    break;
            }
        return next;
    }

    internal static object? Get(object obj, string name) => Prop(obj, name).GetValue(obj);

    internal static void Set(object obj, string name, object? value) => Prop(obj, name).SetValue(obj, value);

    static PropertyInfo Prop(object obj, string name) =>
        obj.GetType().GetProperty(name)
        ?? throw new DatasourceException($"Unknown property \"{name}\"");

    static async Task ApplyLinked<TRelated>(
        IReadonlyRepository<TRelated> via,
        IChildPersist link,
        Enrichment enrichment,
        IReadOnlyList<object> parents,
        IReadOnlyList<Row?>? sources)
    {
        var association = enrichment.Association ?? throw new DatasourceException("Missing association");
        for (var i = 0; i < parents.Count; i++)
        {
            var key = SourceKey(parents[i], enrichment.From, At(sources, i));
            if (key is null) continue;
            var related = new List<TRelated>();
            await foreach (var row in link.Read(new Where(new Operator.Equals(association.Parent, global::Datasources.Value.From(key)))))
            {
                if (Get(row, association.Related) is not { } id) continue;
                var entity = await via.ReadSingle(new Where(new Operator.Equals(enrichment.By, global::Datasources.Value.From(id))));
                if (entity is not null)
                    related.Add(entity);
            }
            Set(parents[i], enrichment.Into, related);
        }
    }

    static Row? At(IReadOnlyList<Row?>? sources, int index) =>
        sources is not null && index < sources.Count ? sources[index] : null;

    internal interface IChildPersist
    {
        IReadOnlyList<string> Ids { get; }
        object New();
        Task<object> Create(object item);
        void AssignIdentity(object item, object created);
        Task<Response> Update(object item, IWhere? byWhere = null);
        Task<Response> Delete(object item, IWhere? byWhere = null);
        IAsyncEnumerable<object> Read(IStatement? query = null);
        IAsyncEnumerable<object> Read(IWhere where);
    }

    sealed class ChildPersist<T> : IChildPersist
    {
        readonly ICRUDRepository<T> inner;
        public IReadOnlyList<string> Ids { get; }
        public ChildPersist(ICRUDRepository<T> inner)
        {
            this.inner = inner;
            Ids = inner.Ids;
        }
        public object New() => Activator.CreateInstance(typeof(T))!;
        public async Task<object> Create(object item) => (await inner.Create((T)item))!;
        public void AssignIdentity(object item, object created)
        {
            if (Ids.Count != 1 || ReferenceEquals(item, created)) return;
            var name = Ids[0];
            var value = created is IDictionary source
                ? source.Contains(name) ? source[name] : null
                : created.GetType().GetProperty(name)?.GetValue(created);
            if (value is null) return;
            if (item is IDictionary dict)
            {
                dict[name] = value;
                return;
            }
            var prop = item.GetType().GetProperty(name);
            if (prop is not { CanWrite: true }) return;
            var target = Nullable.GetUnderlyingType(prop.PropertyType) ?? prop.PropertyType;
            prop.SetValue(item, target.IsInstanceOfType(value) ? value : Convert.ChangeType(value, target));
        }
        public Task<Response> Update(object item, IWhere? byWhere = null) =>
            inner.Update((T)item, byWhere);
        public Task<Response> Delete(object item, IWhere? byWhere = null) =>
            inner.Delete((T)item, byWhere);
        public async IAsyncEnumerable<object> Read(IStatement? query = null)
        {
            await foreach (var item in inner.Read(query))
                yield return item!;
        }
        public async IAsyncEnumerable<object> Read(IWhere where)
        {
            await foreach (var item in inner.Read(where))
                yield return item!;
        }
    }
}
