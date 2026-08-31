namespace Datasources;

public sealed class ReplaceConverter(string search, string replacement) : TypeConverter<string, string>
{
    public override string Convert(string from) => from.Replace(search, replacement);
    public override string ReverseConvert(string to) => to.Replace(replacement, search);
}
