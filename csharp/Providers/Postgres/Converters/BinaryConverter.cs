using System.Text;

namespace Datasources.Postgres;

internal sealed class BinaryConverter : TypeConverter<Value, Value>
{
    public override Value Convert(Value from) => Bytes(from);
    public override Value ReverseConvert(Value to) => Bytes(to);

    internal static bool IsBase64(string text)
    {
        text = text.Trim();
        if (text.Length < 4 || text.Length % 4 != 0 || !text.Contains('=', StringComparison.Ordinal))
            return false;
        foreach (var c in text)
            if (!char.IsAsciiLetterOrDigit(c) && c is not '+' and not '/' and not '=')
                return false;
        try
        {
            System.Convert.FromBase64String(text);
            return true;
        }
        catch (FormatException)
        {
            return false;
        }
    }

    static byte[]? Hex(string text)
    {
        text = text.Trim();
        if (text.Length < 4 || (text.Length & 1) != 0
            || !text.StartsWith("\\x", StringComparison.OrdinalIgnoreCase))
            return null;
        try
        {
            return System.Convert.FromHexString(text[2..]);
        }
        catch (FormatException)
        {
            return null;
        }
    }

    static Value Bytes(Value value) => value switch
    {
        Value.Text(var text) when IsBase64(text) => new Value.Binary(System.Convert.FromBase64String(text.Trim())),
        Value.Text(var text) when Hex(text) is byte[] hex => new Value.Binary(hex),
        Value.Text(var text) => new Value.Binary(Encoding.Latin1.GetBytes(text)),
        _ => value
    };
}
