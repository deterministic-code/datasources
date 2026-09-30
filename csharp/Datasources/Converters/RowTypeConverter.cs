namespace Datasources;

public sealed class RowTypeConverter : TypeConverter<Row, Row>
{
    readonly IReadOnlyDictionary<string, TypeConverter<Value, Value>> fields;

    public RowTypeConverter(IReadOnlyDictionary<string, TypeConverter<Value, Value>> fields) =>
        this.fields = fields;

    public static RowTypeConverter From(IDriver driver, IReadOnlyDictionary<string, string> columns)
    {
        var native = Native(driver);
        var fields = new Dictionary<string, TypeConverter<Value, Value>>();
        foreach (var (name, type) in columns)
            if (driver.RowTypeConverters.TryGetValue(type, out var converter) || native.TryGetValue(type, out converter))
                fields[name] = converter;
        return new(fields);
    }

    public override Row Convert(Row from) => Apply(from, reverse: false);

    public override Row ReverseConvert(Row to) => Apply(to, reverse: true);

    Row Apply(Row row, bool reverse) =>
        new(row.Select(pair => KeyValuePair.Create(
            pair.Key,
            fields.TryGetValue(pair.Key, out var converter)
                ? reverse ? converter.ReverseConvert(pair.Value) : converter.Convert(pair.Value)
                : pair.Value)));

    static Dictionary<string, TypeConverter<Value, Value>> Native(IDriver driver) =>
        driver.RowTypeConverters
            .GroupBy(pair => driver.ColumnType(pair.Key), StringComparer.OrdinalIgnoreCase)
            .Where(group => group.Count() == 1)
            .ToDictionary(group => group.Key, group => group.First().Value, StringComparer.OrdinalIgnoreCase);
}
