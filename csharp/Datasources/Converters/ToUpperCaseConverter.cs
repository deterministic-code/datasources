namespace Datasources;

public sealed class ToUpperCaseConverter : TypeConverter<string, string>
{
    public override string Convert(string from) => from.ToUpperInvariant();
    public override string ReverseConvert(string to) => to;
}
