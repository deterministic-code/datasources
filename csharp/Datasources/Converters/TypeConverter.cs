namespace Datasources;

public sealed class TypeConverter<T>(string? name = null) where T : new()
{
    public string Name { get; } = name ?? typeof(T).Name;
    public RowConverter Rows { get; } = new();
    public T Convert(Row row) => Rows.Convert<T>(row);
    public Row ReverseConvert(T obj) => Rows.ReverseConvert(obj);
}
