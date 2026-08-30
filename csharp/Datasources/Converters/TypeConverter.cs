namespace Datasources;

public abstract class TypeConverter<To, From> : ITypeConverter<To, From>
{
    public abstract To Convert(From from);
    public abstract From ReverseConvert(To to);
}
