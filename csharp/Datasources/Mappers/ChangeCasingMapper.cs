namespace Datasources;

public sealed class ChangeCasingMapper(Casing from, Casing to) : IMapper<string, string>
{
    readonly ChangeCasingConverter forward = new(to);
    readonly ChangeCasingConverter reverse = new(from);

    public string Map(string value) => forward.Convert(value);

    public string ReverseMap(string value) => reverse.Convert(value);
}
