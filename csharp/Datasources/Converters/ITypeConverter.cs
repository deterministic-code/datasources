namespace Datasources;

public interface ITypeConverter<To, From>
{
    To Convert(From from);
    From ReverseConvert(To to);
}
