namespace Datasources;

public sealed class PluralizerMapper : IMapper<string, string>
{
    readonly PluralizerConverter pluralizer = new();

    public string Map(string value) => pluralizer.Convert(value);

    public string ReverseMap(string value) => pluralizer.ReverseConvert(value);
}
