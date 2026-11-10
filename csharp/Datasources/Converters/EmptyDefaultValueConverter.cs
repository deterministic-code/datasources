namespace Datasources;

public sealed class EmptyDefaultValueConverter : IDefaultValueConverter
{
    public static EmptyDefaultValueConverter Instance { get; } = new();
    EmptyDefaultValueConverter() { }
    public string? Convert(string specType, string text) => null;
    public string ReverseConvert(string specType, string native) =>
        throw new InvalidOperationException($"Native default '{native}' is not a valid {specType} default");
}
