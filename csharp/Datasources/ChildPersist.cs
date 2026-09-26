using System.Collections;

namespace Datasources;

internal interface IChildPersist
{
    IReadOnlyList<string> Ids { get; }
    object New();
    Task<object> Create(object item);
    void AssignIdentity(object item, object created);
    Task<Response> Update(object item, IWhere? byWhere = null);
    Task<Response> Delete(object item, IWhere? byWhere = null);
    IAsyncEnumerable<object> Read(IWhere where);
}

internal static class ChildPersist
{
    internal static IChildPersist For<T>(ICRUDRepository<T> repository) => new Adapter<T>(repository);

    sealed class Adapter<T> : IChildPersist
    {
        readonly ICRUDRepository<T> inner;
        public IReadOnlyList<string> Ids { get; }

        public Adapter(ICRUDRepository<T> inner)
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

        public async IAsyncEnumerable<object> Read(IWhere where)
        {
            await foreach (var item in inner.Read(where))
                yield return item!;
        }
    }
}
