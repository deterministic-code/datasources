namespace Datasources;

public sealed class FieldConverter(Func<object?, object?> to, Func<object?, object?> from) : IFieldConverter
{
    public Value To(Value? value) => Value.From(to((value ?? Value.Null.Instance).ToObject()));
    public Value From(Value? value) => Value.From(from((value ?? Value.Null.Instance).ToObject()));
}
