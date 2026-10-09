using System.Globalization;

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
    public sealed record DateTime(System.DateTime Item) : Value;
    public sealed record Guid(System.Guid Item) : Value;
    public sealed record Decimal(decimal Item) : Value;
    public sealed record Binary(byte[] Item) : Value;

    public static implicit operator Value(bool value) => new Bool(value);
    public static implicit operator Value(int value) => new Int(value);
    public static implicit operator Value(long value) => new Int(value);
    public static implicit operator Value(double value) => new Float(value);
    public static implicit operator Value(string value) => new Text(value);
    public static implicit operator Value(System.DateTime value) => new DateTime(value);
    public static implicit operator Value(System.Guid value) => new Guid(value);
    public static implicit operator Value(decimal value) => new Decimal(value);
    public static implicit operator Value(byte[] value) => new Binary(value);

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
        ulong n => new Int((long)n),
        float n => new Float(n),
        double n => new Float(n),
        string text => new Text(text),
        System.DateTime date => new DateTime(date),
        System.Guid guid => new Guid(guid),
        decimal number => new Decimal(number),
        byte[] bytes => new Binary(bytes),
        _ => new Text(value.ToString() ?? "")
    };

    public object? ToObject() => this switch
    {
        Null => null,
        Bool(var flag) => flag,
        Int(var number) => number,
        Float(var number) => number,
        Text(var text) => text,
        DateTime(var date) => date,
        Guid(var guid) => guid,
        Decimal(var number) => number,
        Binary(var bytes) => bytes,
        _ => null
    };

    public string IsoDateTime() => this is DateTime(var date)
        ? (date.Kind == DateTimeKind.Utc ? date : date.ToUniversalTime())
            .ToString("yyyy-MM-dd'T'HH:mm:ss.fff'Z'", CultureInfo.InvariantCulture)
        : "";
}
