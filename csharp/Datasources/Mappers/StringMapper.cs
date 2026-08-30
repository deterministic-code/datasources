namespace Datasources;

public class StringMapper : IMapper<string>
{
    public string To(string name) => name;
    public string From(string name) => name;
}
