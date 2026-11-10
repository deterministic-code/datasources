namespace Datasources;

public interface IDefaultValueConverter
{
    string? Convert(string specType, string text);
    string ReverseConvert(string specType, string native);
}
