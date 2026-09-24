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

    public IRepositoryTreeNode Include<TRelated>(
        IReadonlyRepository<TRelated> via,
        string into,
        string from,
        string by,
        string? select = null,
        bool many = false)
    {
        var enrichment = new Enrichment(into, from, by, select, many);
        var child = new RepositoryTreeNode(
            via,
            into,
            this,
            (parents, sources) => ((ITrackedApply)binder).ApplyTracked(via, enrichment, parents, sources),
            enrichment,
            binder)
        {
            Persist = via is ICRUDRepository<TRelated> crud ? new ChildPersist<TRelated>(crud) : null
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

    internal interface IChildPersist
    {
        Task<Response> Create(object item);
        Task<Response> Update(object item, IWhere? byWhere = null);
        Task<Response> Delete(object item, IWhere? byWhere = null);
        IAsyncEnumerable<object> Read(IStatement? query = null);
        IAsyncEnumerable<object> Read(IWhere where);
    }

    sealed class ChildPersist<T> : IChildPersist
    {
        readonly ICRUDRepository<T> inner;
        public ChildPersist(ICRUDRepository<T> inner) => this.inner = inner;
        public Task<Response> Create(object item) => inner.Create((T)item);
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
