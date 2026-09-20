namespace Datasources;

public interface IFlatToNestedMapper : IMapper<IEnumerable<Row>, IReadOnlyList<Dictionary<string, MappedValue>>>
{
    TypeMap Spec { get; }
}
