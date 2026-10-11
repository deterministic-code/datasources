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

    static Value Bytes(Value value) => value switch
    {
        Value.Text(var text) => new Value.Binary(System.Convert.FromBase64String(text)),
        _ => value
    };
}
