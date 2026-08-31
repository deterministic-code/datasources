using System.Text.RegularExpressions;

namespace Datasources;

public sealed class RegexConverter(Regex regex, string replacement) : TypeConverter<string, string>
{
    public RegexConverter(string pattern, string replacement) : this(new Regex(pattern), replacement) { }

    public override string Convert(string from) => regex.Replace(from, replacement);
    public override string ReverseConvert(string to) => to;
}
