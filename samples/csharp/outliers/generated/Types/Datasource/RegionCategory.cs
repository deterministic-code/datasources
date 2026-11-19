// schema-version: 1.0
namespace Backend.Types.Datasource;

/** Type RegionCategory. */
public class RegionCategory
{
    public long RegionId { get; set; }
    public long CategoryId { get; set; }
    public long? ParentRegionId { get; set; }
    public long? ParentCategoryId { get; set; }
    public string Name { get; set; } = "";
}
