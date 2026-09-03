namespace Datasources;

public interface IFlatToTypeMapper : IMapper<Row, Dictionary<string, MappedValue>>
{
    TypeMap Spec { get; }
    IReadOnlyList<Dictionary<string, MappedValue>> Collapse(IEnumerable<Dictionary<string, MappedValue>> rows);
    IReadOnlyList<Dictionary<string, MappedValue>> Collapse(IEnumerable<Row> rows);
}
