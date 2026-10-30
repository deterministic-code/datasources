namespace Datasources;

public sealed class DeterministicTypeConverter : IMapping<string, string>
{
    readonly IReadOnlyDictionary<string, string> dictionary;
    readonly Dictionary<string, string> reverse = [];
    readonly SpecFieldRules rules;

    public IReadOnlyDictionary<string, string> Dictionary => dictionary;

    public DeterministicTypeConverter(IReadOnlyDictionary<string, string> dictionary, SpecFieldRules rules)
    {
        this.dictionary = dictionary;
        this.rules = rules;
        foreach (var (specType, native) in dictionary)
            reverse.TryAdd(native, specType);
    }

    public string Convert(string from) =>
        dictionary.TryGetValue(from, out var native)
            ? native
            : throw new InvalidOperationException($"Unknown spec field type: {from}");

    public string Convert(SpecField field)
    {
        if (field.Type == "string") return Sized(rules.StringUnlimited, rules.StringSized, field);
        if (field.Type == "character" && rules.Character is { } character)
            return $"{character}({CharLen(field)})";
        if (field.Type == "decimal" && rules.Decimal is { } rule)
        {
            var ps = DecimalPrecisionScale(field);
            return rule switch
            {
                DecimalRule.Required required => ps is { } sized
                    ? $"{required.Type}({sized.P}, {sized.S})"
                    : throw new InvalidOperationException(
                        $"decimal field \"{field.Name}\" requires size: [precision, scale] for {rules.DialectName ?? "sql"}"),
                DecimalRule.Optional optional => ps is { } sized
                    ? $"{optional.Type}({sized.P}, {sized.S})"
                    : optional.Fallback,
                _ => throw new InvalidOperationException("empty decimal rule"),
            };
        }
        if (field.Type == "binary")
        {
            if (rules.BinarySized is { } sized && rules.BinaryUnlimited is { } unlimited)
                return Sized(unlimited, sized, field);
        }
        return Convert(field.Type);
    }

    public string ReverseConvert(string native) =>
        reverse.TryGetValue(native, out var spec)
            ? spec
            : throw new InvalidOperationException($"Unknown native field type: {native}");

    static bool IsUnlimited(object? size) =>
        size is null or "unlimited";

    static string Sized(string unlimited, Func<int, string> render, SpecField field) =>
        IsUnlimited(field.Size) ? unlimited : render(System.Convert.ToInt32(field.Size));

    static int CharLen(SpecField field) =>
        field.Size is null ? 1 : System.Convert.ToInt32(field.Size);

    static (int P, int S)? DecimalPrecisionScale(SpecField field) =>
        field.Size switch
        {
            null => null,
            int[] { Length: 2 } pair when pair[0] == (int)pair[0] && pair[1] == (int)pair[1] =>
                (pair[0], pair[1]),
            IReadOnlyList<int> { Count: 2 } list => (list[0], list[1]),
            _ => throw new InvalidOperationException(
                $"decimal field \"{field.Name}\" size must be [precision, scale] tuple; got {field.Size}"),
        };
}
