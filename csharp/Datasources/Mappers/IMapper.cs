namespace Datasources;

public interface IMapper<TFrom, TTo>
{
    TTo Map(TFrom value);
    TFrom ReverseMap(TTo value);
}
