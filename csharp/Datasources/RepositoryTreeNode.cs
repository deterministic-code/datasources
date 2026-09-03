using System.Collections;
using System.Reflection;
using Datasources.Builders;

namespace Datasources;

public sealed class RepositoryTreeNode : IRepositoryTreeNode
{
    readonly List<RepositoryTreeNode> items = [];
    readonly Func<IReadOnlyList<object>, Task> apply;

    public RepositoryTreeNode(
        object value,
        string propertyName,
        IRepositoryTreeNode? parent = null,
        Func<IReadOnlyList<object>, Task>? apply = null,
        Enrichment? enrichment = null)
    {
        Value = value;
        PropertyName = propertyName;
        Parent = parent;
        Enrichment = enrichment;
        this.apply = apply ?? (_ => Task.CompletedTask);
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
            via, into, this, parents => Apply(via, enrichment, parents), enrichment)
        {
            Persist = via is ICRUDRepository<TRelated> crud ? new ChildPersist<TRelated>(crud) : null
        };
        items.Add(child);
        return child;
    }

    public async Task EnrichFrom(IReadOnlyList<object> parents)
    {
        await apply(parents);
        var next = Related(parents, PropertyName);
        foreach (var child in items)
            await child.EnrichFrom(next);
    }

    static Task Apply<TRelated>(
        IReadonlyRepository<TRelated> via,
        Enrichment enrichment,
        IReadOnlyList<object> parents) =>
        enrichment.Many ? ApplyMany(via, enrichment, parents) : ApplyLookup(via, enrichment, parents);

    static async Task ApplyLookup<TRelated>(
        IReadonlyRepository<TRelated> via,
        Enrichment enrichment,
        IReadOnlyList<object> parents)
    {
        var cache = new Dictionary<string, TRelated?>();
        foreach (var parent in parents)
        {
            var key = Get(parent, enrichment.From);
            if (key is null) continue;
            var cacheKey = key.ToString() ?? "";
            if (!cache.TryGetValue(cacheKey, out var related))
            {
                related = await via.GetSingle(WhereBuilder.Equals(enrichment.By, Datasources.Value.From(key)));
                cache[cacheKey] = related;
            }
            if (related is null) continue;
            Set(parent, enrichment.Into, enrichment.Select is { } select ? Get(related, select) : related);
        }
    }

    static async Task ApplyMany<TRelated>(
        IReadonlyRepository<TRelated> via,
        Enrichment enrichment,
        IReadOnlyList<object> parents)
    {
        foreach (var parent in parents)
        {
            var key = Get(parent, enrichment.From);
            if (key is null) continue;
            var related = new List<TRelated>();
            await foreach (var item in via.Get(WhereBuilder.Equals(enrichment.By, Datasources.Value.From(key))))
                related.Add(item);
            Set(parent, enrichment.Into, related);
        }
    }

    static IReadOnlyList<object> Related(IReadOnlyList<object> parents, string propertyName)
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

    static object? Get(object obj, string name) => Prop(obj, name).GetValue(obj);

    static void Set(object obj, string name, object? value) => Prop(obj, name).SetValue(obj, value);

    static PropertyInfo Prop(object obj, string name) =>
        obj.GetType().GetProperty(name)
        ?? throw new DatasourceException($"Unknown property \"{name}\"");

    internal interface IChildPersist
    {
        Task<Response> Create(object item);
        Task<Response> Update(object item, IWhere? byWhere = null);
        Task<Response> Delete(object item, IWhere? byWhere = null);
        IAsyncEnumerable<object> Get(IStatement? query = null);
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
        public async IAsyncEnumerable<object> Get(IStatement? query = null)
        {
            await foreach (var item in inner.Get(query))
                yield return item!;
        }
    }
}
