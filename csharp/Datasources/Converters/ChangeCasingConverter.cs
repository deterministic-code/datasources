namespace Datasources;

public enum Casing
{
    Camel,
    Capital,
    Constant,
    Dot,
    Kebab,
    No,
    Pascal,
    PascalSnake,
    Path,
    Sentence,
    Snake,
    Train,
}

public sealed class ChangeCasingConverter(Casing casing) : TypeConverter<string, string>
{
    public override string Convert(string from) => Join(Words(from), casing);
    public override string ReverseConvert(string to) => to;

    static string Join(IReadOnlyList<string> words, Casing casing) => casing switch
    {
        Casing.Camel => string.Concat(words.Select((word, i) => i == 0 ? word : Cap(word))),
        Casing.Capital => string.Join(' ', words.Select(Cap)),
        Casing.Constant => string.Join('_', words.Select(word => word.ToUpperInvariant())),
        Casing.Dot => string.Join('.', words),
        Casing.Kebab => string.Join('-', words),
        Casing.No => string.Join(' ', words),
        Casing.Pascal => string.Concat(words.Select(Cap)),
        Casing.PascalSnake => string.Join('_', words.Select(Cap)),
        Casing.Path => string.Join('/', words),
        Casing.Sentence => string.Join(' ', words.Select((word, i) => i == 0 ? Cap(word) : word)),
        Casing.Snake => string.Join('_', words),
        Casing.Train => string.Join('-', words.Select(Cap)),
        _ => throw new ArgumentOutOfRangeException(nameof(casing), casing, null)
    };

    static List<string> Words(string text)
    {
        var words = new List<string>();
        var start = 0;
        for (var i = 0; i < text.Length; i++)
        {
            var split = Sep(text[i])
                || (i > 0 && char.IsLower(text[i - 1]) && char.IsUpper(text[i]))
                || (i > 0 && i + 1 < text.Length && char.IsUpper(text[i - 1]) && char.IsUpper(text[i]) && char.IsLower(text[i + 1]))
                || (i > 0 && char.IsLetter(text[i - 1]) != char.IsLetter(text[i]) && (char.IsLetterOrDigit(text[i - 1]) && char.IsLetterOrDigit(text[i])));
            if (!split) continue;
            if (i > start) words.Add(text[start..i]);
            start = Sep(text[i]) ? i + 1 : i;
        }
        if (start < text.Length) words.Add(text[start..]);
        return [.. words.Select(word => word.ToLowerInvariant()).Where(word => word.Length > 0)];
    }

    static bool Sep(char c) => c is '_' or '.' or '-' or '/' || char.IsWhiteSpace(c);
    static string Cap(string word) => char.ToUpperInvariant(word[0]) + word[1..];
}
