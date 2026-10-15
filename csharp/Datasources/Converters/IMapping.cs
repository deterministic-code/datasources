namespace Datasources;

public interface IMapping<TFrom, TTo>
{
    TTo Convert(TFrom from);
    TFrom ReverseConvert(TTo to);
}
