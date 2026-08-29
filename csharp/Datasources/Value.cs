namespace Datasources;

public abstract record Value
{
    public sealed record Null : Value
    {
        public static readonly Null Instance = new();
    }

    public sealed record Bool(bool Item) : Value;
    public sealed record Int(long Item) : Value;
    public sealed record Float(double Item) : Value;
    public sealed record Text(string Item) : Value;

    public static implicit operator Value(bool value) => new Bool(value);
    public static implicit operator Value(int value) => new Int(value);
    public static implicit operator Value(long value) => new Int(value);
    public static implicit operator Value(double value) => new Float(value);
    public static implicit operator Value(string value) => new Text(value);

    public static Value From(object? value) => value switch
    {
        null or DBNull => Null.Instance,
        Value v => v,
        bool flag => new Bool(flag),
        byte n => new Int(n),
        sbyte n => new Int(n),
        short n => new Int(n),
        ushort n => new Int(n),
        int n => new Int(n),
        uint n => new Int(n),
        long n => new Int(n),
        float n => new Float(n),
        double n => new Float(n),
        string text => new Text(text),
        DateTime date => new Text(date.ToUniversalTime().ToString("yyyy-MM-dd'T'HH:mm:ss.fff'Z'")),
        Guid guid => new Text(guid.ToString()),
        decimal number => new Text(number.ToString(System.Globalization.CultureInfo.InvariantCulture)),
        byte[] bytes => new Text(Convert.ToBase64String(bytes)),
        _ => new Text(value.ToString() ?? "")
    };

    public object? ToObject() => this switch
    {
        Null => null,
        Bool(var flag) => flag,
        Int(var number) => number,
        Float(var number) => number,
        Text(var text) => text,
        _ => null
    };
}
