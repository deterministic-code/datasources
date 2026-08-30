namespace Datasources;

public sealed class ToLowerCaseConverter : TypeConverter<string, string>
{
    public override string Convert(string from) => from.ToLowerInvariant();
    public override string ReverseConvert(string to) => to;
}
