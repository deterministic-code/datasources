using System.Text;

namespace Datasources.Mysql;

internal sealed class BinaryConverter : TypeConverter<Value, Value>
{
    public override Value Convert(Value from) => Bytes(from);
    public override Value ReverseConvert(Value to) => Bytes(to);

    static Value Bytes(Value value)
    {
        if (value is not Value.Text(var text))
            return value;
        try
        {
            return new Value.Binary(System.Convert.FromBase64String(text));
        }
        catch (FormatException)
        {
            return new Value.Binary(Encoding.Latin1.GetBytes(text));
        }
    }
}
