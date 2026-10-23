using Datasources;

namespace Datasources.CodeGenerators;

public static class LastTokenPluralize
{
    public static string Of(string name)
    {
        var at = name.LastIndexOf('_');
        return at < 0
            ? new PluralizerConverter().Convert(name)
            : name[..(at + 1)] + new PluralizerConverter().Convert(name[(at + 1)..]);
    }
}
