namespace Datasources;

public class IdentityMapper<T> : IMapper<T, T>
{
    public T Map(T value) => value;
    public T ReverseMap(T value) => value;
}
