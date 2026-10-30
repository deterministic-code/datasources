namespace Datasources;

public sealed class RowTypeConverter : TypeConverter<Row, Row>
{
    readonly IReadOnlyDictionary<string, TypeConverter<Value, Value>> fields;

    public RowTypeConverter(IReadOnlyDictionary<string, TypeConverter<Value, Value>> fields) =>
        this.fields = fields;

    public static RowTypeConverter From(
        IReadOnlyDictionary<string, TypeConverter<Value, Value>> converters,
        IReadOnlyDictionary<string, string> columns)
    {
        var fields = new Dictionary<string, TypeConverter<Value, Value>>();
        foreach (var (name, type) in columns)
            if (FieldConverter(converters, type) is { } converter)
                fields[name] = converter;
        return new(fields);
    }

    static TypeConverter<Value, Value>? FieldConverter(
        IReadOnlyDictionary<string, TypeConverter<Value, Value>> converters,
        string type) =>
        converters.TryGetValue(type, out var converter) ? converter
        : type.EndsWith("integer", StringComparison.Ordinal) && converters.TryGetValue("integer", out converter)
            ? converter
            : null;

    public override Row Convert(Row from) => Apply(from, reverse: false);

    public override Row ReverseConvert(Row to) => Apply(to, reverse: true);

    Row Apply(Row row, bool reverse) =>
        new(row.Select(pair => KeyValuePair.Create(
            pair.Key,
            fields.TryGetValue(pair.Key, out var converter)
                ? reverse ? converter.ReverseConvert(pair.Value) : converter.Convert(pair.Value)
                : pair.Value)));
}
