namespace Datasources.Postgres;

internal sealed class BinaryConverter : TypeConverter<Value, Value>
{
    public override Value Convert(Value from) => Bytes(from);
    public override Value ReverseConvert(Value to) => Bytes(to);

    static Value Bytes(Value value) => value switch
    {
        Value.Text(var text) => new Value.Binary(System.Convert.FromBase64String(text)),
        _ => value
    };
}
