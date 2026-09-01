namespace Datasources;

public class RowTypeMapper<T>(TypeConverter<T, Row> converter)
    : TransformMapper<Row, T>(converter.Convert, converter.ReverseConvert)
    where T : new();
