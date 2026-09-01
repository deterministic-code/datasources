namespace Datasources;

public class TransformConverterMapper<T>(
    TypeConverter<T, T> converter,
    Func<T, bool>? filter = null) : TransformMapper<T, T>(
        value => filter?.Invoke(value) != false ? converter.Convert(value) : value,
        value => filter?.Invoke(value) != false ? converter.ReverseConvert(value) : value);

public class TransformConverterMapper<TTo, TFrom>(TypeConverter<TTo, TFrom> converter)
    : TransformMapper<TFrom, TTo>(converter.Convert, converter.ReverseConvert);
