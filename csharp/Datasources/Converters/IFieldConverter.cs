namespace Datasources;

public interface IFieldConverter
{
    Value To(Value? value);
    Value From(Value? value);
}
