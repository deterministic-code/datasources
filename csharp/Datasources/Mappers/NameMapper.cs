namespace Datasources;

public abstract class NameMapper : IMapper<string, string>
{
    public string Map(string name) => name;
    public string ReverseMap(string name) => name;
}
