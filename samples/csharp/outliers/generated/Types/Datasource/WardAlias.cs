// schema-version: 1.0
namespace Backend.Types.Datasource;

/** Type WardAlias. */
public class WardAlias
{
    public long Id { get; set; }
    public long ParentRegionId { get; set; }
    public long ParentWardId { get; set; }
    public string Name { get; set; } = "";
}
