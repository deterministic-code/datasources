using System.Reflection;

namespace Datasources;

public sealed class RepositoryTreeNode(
    object value,
    string propertyName,
    IRepositoryTreeNode? parent = null,
    Enrichment? enrichment = null) : IRepositoryTreeNode
{
    public object Value { get; } = value;
    public string PropertyName { get; } = propertyName;
    public IRepositoryTreeNode? Parent { get; } = parent;
    public Enrichment? Enrichment { get; } = enrichment;

    internal static object? SourceKey(object parent, string name, Row? source) =>
        source is not null && source.TryGetValue(name, out var cell) && cell is not global::Datasources.Value.Null
            ? cell.ToObject()

            : parent.GetType().GetProperty(name)?.GetValue(parent);

    internal static string KeyOf(object? key) => key?.ToString() ?? "";

    internal static object? Get(object obj, string name) => Prop(obj, name).GetValue(obj);

    internal static void Set(object obj, string name, object? value) => Prop(obj, name).SetValue(obj, value);

    static PropertyInfo Prop(object obj, string name) =>
        obj.GetType().GetProperty(name)
        ?? throw new DatasourceException($"Unknown property \"{name}\"");
}
