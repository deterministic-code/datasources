// schema-version: 1.0
namespace Backend.Types.Datasource;

/** Type ItemLink. */
public class ItemLink : Base
{
    public long FromItemId { get; set; }
    public long ToItemId { get; set; }
}
