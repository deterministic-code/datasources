using System.Collections;
using System.Reflection;
using Datasources.Builders;

namespace Datasources;

public class DeepCRUDRepository<T> : EnrichedReadonlyRepository<T>, ICRUDRepository<T>
{
    readonly ICRUDRepository<T> inner;

    public DeepCRUDRepository(ICRUDRepository<T> inner) : base(inner) =>
        this.inner = inner;

    public async Task<Response> Create(T item)
    {
        var response = await inner.Create(item);
        ApplyIdentity(item!, response);
        await Persist(Tree.Parent, [item!], Mode.Create);
        return response;
    }

    public async Task<Response> Update(T item, IWhere? byWhere = null)
    {
        var response = await inner.Update(item, byWhere);
        await Persist(Tree.Parent, [item!], Mode.Update);
        return response;
    }

    public async Task<Response> Delete(T item, IWhere? byWhere = null)
    {
        await Persist(Tree.Parent, [item!], Mode.Delete);
        return await inner.Delete(item, byWhere);
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
            ApplyIdentity(related, await writer.Create(related));
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
        var keep = payload.Select(Id).Where(HasId).Select(IdKey).ToHashSet();
        foreach (var row in existing.Where(row => !keep.Contains(IdKey(Id(row)))))
        {
            await Persist(node, [row], Mode.Delete);
            await writer.Delete(row);
        }
        foreach (var related in payload)
        {
            var id = Id(related);
            if (HasId(id) && existing.Any(row => IdKey(Id(row)) == IdKey(id)))
                await writer.Update(related);
            else
                ApplyIdentity(related, await writer.Create(related));
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
        await foreach (var item in writer.Get(WhereBuilder.Equals(enrichment.By, Value.From(key))))
            items.Add(item);
        return items;
    }

    static void ApplyIdentity(object item, Response response)
    {
        if (response.Result is Value.Null) return;
        var name = item.GetType().GetProperty("id") is not null ? "id"
            : item.GetType().GetProperty("Id") is not null ? "Id"
            : null;
        if (name is not null)
            Set(item, name, response.Result.ToObject());
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

    static object? Id(object item)
    {
        var type = item.GetType();
        return (type.GetProperty("id") ?? type.GetProperty("Id"))?.GetValue(item);
    }

    static bool HasId(object? id) =>
        id is not null and not 0 and not 0L and not 0u and not "";

    static string IdKey(object? id) => id?.ToString() ?? "";

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

    enum Mode { Create, Update, Delete }
}
