using System.Collections;
using System.Reflection;

namespace Datasources;

public class DefaultWriteEnrichmentStrategy<T> : IWriteEnrichmentStrategy<T>
{
    public Task Persist(IRepositoryTree tree, IList<T> items, WriteMode mode) =>
        Walk(tree.Parent, [.. items.Cast<object>()], mode);

    protected virtual bool SkipUnloaded => false;

    async Task Walk(IRepositoryTreeNode node, IReadOnlyList<object> parents, WriteMode mode)
    {
        foreach (var child in node.Items)
            await PersistNode(child, parents, mode);
    }

    async Task PersistNode(IRepositoryTreeNode child, IReadOnlyList<object> parents, WriteMode mode)
    {
        if (child.Enrichment is { Association: { } association } enrichment)
        {
            var writer = (child as RepositoryTreeNode)?.Link
                ?? throw new DatasourceException($"Deep write requires a CRUD repository for \"{enrichment.Into}\"");
            foreach (var parent in parents)
            {
                if (Skip(parent, enrichment.Into, mode)) continue;
                await (mode switch
                {
                    WriteMode.Create => CreateLinks(writer, enrichment, association, parent),
                    WriteMode.Update => SyncLinks(writer, enrichment, association, parent),
                    WriteMode.Delete => DeleteLinks(writer, enrichment, association, parent),
                    _ => throw new DatasourceException("Unknown persist mode")
                });
            }
            return;
        }
        if (child.Enrichment is not { Many: true, Select: null } owned)
            return;
        var childWriter = (child as RepositoryTreeNode)?.Persist
            ?? throw new DatasourceException($"Deep write requires a CRUD repository for \"{owned.Into}\"");
        foreach (var parent in parents)
        {
            if (Skip(parent, owned.Into, mode)) continue;
            await (mode switch
            {
                WriteMode.Create => CreateChildren(child, childWriter, owned, parent),
                WriteMode.Update => SyncChildren(child, childWriter, owned, parent),
                WriteMode.Delete => DeleteChildren(child, childWriter, owned, parent),
                _ => throw new DatasourceException("Unknown persist mode")
            });
        }
    }

    bool Skip(object parent, string into, WriteMode mode) =>
        SkipUnloaded && mode is not WriteMode.Delete && Get(parent, into) is null;

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
            await Walk(node, [related], WriteMode.Create);
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
            await Walk(node, [row], WriteMode.Delete);
            await writer.Delete(row);
        }
        foreach (var related in payload)
        {
            if (HasKey(related, keys) && existing.Any(row => KeyOf(row, keys) == KeyOf(related, keys)))
                await writer.Update(Strip(node, related));
            else
                writer.AssignIdentity(related, await writer.Create(Strip(node, related)));
            await Walk(node, [related], WriteMode.Update);
        }
    }

    async Task DeleteChildren(
        IRepositoryTreeNode node,
        RepositoryTreeNode.IChildPersist writer,
        Enrichment enrichment,
        object parent)
    {
        var existing = await Load(writer, enrichment, parent);
        await Walk(node, existing, WriteMode.Delete);
        foreach (var row in existing)
            await writer.Delete(row);
    }

    static async Task CreateLinks(
        RepositoryTreeNode.IChildPersist writer,
        Enrichment enrichment,
        Association association,
        object parent)
    {
        var parentKey = Required(parent, enrichment.From, enrichment.Into);
        foreach (var id in RelatedIds(parent, enrichment))
        {
            var row = writer.New();
            Set(row, association.Parent, parentKey);
            Set(row, association.Related, id);
            await writer.Create(row);
        }
    }

    static async Task SyncLinks(
        RepositoryTreeNode.IChildPersist writer,
        Enrichment enrichment,
        Association association,
        object parent)
    {
        var parentKey = Required(parent, enrichment.From, enrichment.Into);
        var existing = await LoadLinks(writer, association.Parent, parentKey);
        var wanted = RelatedIds(parent, enrichment);
        var keep = wanted.Select(id => id.ToString()).ToHashSet();
        foreach (var row in existing)
        {
            var id = Get(row, association.Related);
            if (id?.ToString() is { } key && keep.Contains(key)) continue;
            await writer.Delete(row, LinkWhere(association, parentKey, id));
        }
        foreach (var id in wanted)
        {
            if (existing.Any(row => Get(row, association.Related)?.ToString() == id.ToString()))
                continue;
            var row = writer.New();
            Set(row, association.Parent, parentKey);
            Set(row, association.Related, id);
            await writer.Create(row);
        }
    }

    static async Task DeleteLinks(
        RepositoryTreeNode.IChildPersist writer,
        Enrichment enrichment,
        Association association,
        object parent)
    {
        var parentKey = Get(parent, enrichment.From);
        if (parentKey is null) return;
        foreach (var row in await LoadLinks(writer, association.Parent, parentKey))
            await writer.Delete(row, LinkWhere(association, parentKey, Get(row, association.Related)));
    }

    static async Task<List<object>> LoadLinks(
        RepositoryTreeNode.IChildPersist writer,
        string column,
        object key)
    {
        var items = new List<object>();
        await foreach (var item in writer.Read(new WhereClause(new Operator.Equals(column, Value.From(key)))))
            items.Add(item);
        return items;
    }

    static List<object> RelatedIds(object parent, Enrichment enrichment)
    {
        var ids = new List<object>();
        var seen = new HashSet<string>();
        foreach (var related in Payload(parent, enrichment.Into))
        {
            if (Get(related, enrichment.By) is not object id || Missing(id))
                throw new DatasourceException($"{enrichment.Into} requires \"{enrichment.By}\"");
            if (seen.Add(id.ToString() ?? ""))
                ids.Add(id);
        }
        return ids;
    }

    static object Required(object parent, string name, string into) =>
        Get(parent, name) is { } key && !Missing(key)
            ? key
            : throw new DatasourceException($"{into} requires \"{name}\"");

    static bool Missing(object? value) => value is null or 0 or 0L or 0u or "";

    static IWhere LinkWhere(Association association, object parentKey, object? relatedKey) =>
        new WhereClause(new Operator.And(
            new Operator.Equals(association.Parent, Value.From(parentKey)),
            new Operator.Equals(association.Related, Value.From(relatedKey))));

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
}
