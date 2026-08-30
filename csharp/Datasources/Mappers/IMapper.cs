namespace Datasources;

public interface IMapper<T>
{
    T To(T value);
    T From(T value);
}
