using System.Collections;
using System.Reflection;

namespace Datasources;

public sealed class RepositoryTreeNode : IRepositoryTreeNode
{
    public RepositoryTreeNode(object value, string propertyName, IRepositoryTreeNode? parent = null, Enrichment? enrichment = null)
    {
        Value = value;
        PropertyName = propertyName;
        Parent = parent;
        Enrichment = enrichment;
    }

    public object Value { get; }
    public string PropertyName { get; }
    public IRepositoryTreeNode? Parent { get; }
    public Enrichment? Enrichment { get; }

    internal static IChildPersist Writer<T>(ICRUDRepository<T> repository) => new ChildPersist<T>(repository);

    internal static object? SourceKey(object parent, string name, Row? source) =>
        source is not null && source.TryGetValue(name, out var cell) && cell is not global::Datasources.Value.Null
            ? cell.ToObject()
            : parent.GetType().GetProperty(name)?.GetValue(parent);

    internal static string KeyOf(object? key) => key?.ToString() ?? "";

    internal static IList<object> Related(IList<object> parents, string propertyName)
    {
        if (propertyName.Length == 0) return parents as IList<object> ?? [.. parents];
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
