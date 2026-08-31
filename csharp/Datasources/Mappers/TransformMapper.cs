namespace Datasources;

public sealed class TransformMapper<TFrom, TTo>(
    Func<TFrom, TTo> map,
    Func<TTo, TFrom> reverseMap) : IMapper<TFrom, TTo>
{
    public TTo Map(TFrom value) => map(value);
    public TFrom ReverseMap(TTo value) => reverseMap(value);
}
