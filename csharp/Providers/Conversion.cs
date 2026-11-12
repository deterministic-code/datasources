using Xunit;

namespace Datasources;

public readonly record struct Conversion(string Type, object? From, object? To)
{
    public string Text =>
        From as string ?? throw new ArgumentException(From?.GetType().FullName);

    public object? Map(Func<string, string> names, Func<SpecField, string> fields) =>
        From switch
        {
            SpecField field => fields(field),
            string text => names(text),
            _ => throw new ArgumentException(From?.GetType().FullName),
        };

    public void Assert(Func<object?> act)
    {
        if (To is Type type)
        {
            Xunit.Assert.Throws(type, act);
            return;
        }
        Xunit.Assert.Equal(To, act());
    }

    public static void Types(
        IEnumerable<Conversion> rows,
        Func<string, string> names,
        Func<SpecField, string> fields)
    {
        foreach (var row in rows)
            row.Assert(() => row.Map(names, fields));
    }

    public static void ReverseTypes(IMapping<string, string> types, IEnumerable<Conversion> rows)
    {
        foreach (var row in rows)
            row.Assert(() => types.ReverseConvert(row.Text));
    }

    public static void Defaults(IDefaultValueConverter defaults, IEnumerable<Conversion> rows, bool reverse = false)
    {
        foreach (var row in rows)
            row.Assert(() =>
                reverse
                    ? defaults.ReverseConvert(row.Type, row.Text)
                    : defaults.Convert(row.Type, row.Text));
    }

    public static void Cells(
        IReadOnlyDictionary<string, TypeConverter<Value, Value>> converters,
        IEnumerable<Conversion> rows,
        bool reverse = false)
    {
        foreach (var row in rows)
            row.Assert(() =>
                (reverse
                    ? converters[row.Type].ReverseConvert(Value.From(row.From))
                    : converters[row.Type].Convert(Value.From(row.From))).ToObject());
    }
}
