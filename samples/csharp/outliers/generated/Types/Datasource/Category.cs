// schema-version: 1.0
namespace Backend.Types.Datasource;

/** Type Category. */
public class Category
{
    public long Id { get; set; }
    public long? ParentCategoryId { get; set; }
    public string Name { get; set; } = "";
}
