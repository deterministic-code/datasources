namespace Datasources;

public sealed class FieldConverter(Func<object?, object?> to, Func<object?, object?> from) : IFieldConverter
{
    public object? To(object? value) => to(value);
    public object? From(object? value) => from(value);
}
