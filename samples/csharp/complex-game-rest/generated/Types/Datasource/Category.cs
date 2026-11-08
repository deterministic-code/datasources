// schema-version: 1.0
namespace Backend.Types.Datasource;

/** Type Category. */
public class Category : Base
{
    public long? CategoryId { get; set; }
    public string Name { get; set; } = "";
    public string Description { get; set; } = "";
    public bool IsEnabled { get; set; } = true;
    public bool IsDeleted { get; set; }
}
