namespace Datasources;

public interface IFieldConverter
{
    object? To(object? value);
    object? From(object? value);
}
