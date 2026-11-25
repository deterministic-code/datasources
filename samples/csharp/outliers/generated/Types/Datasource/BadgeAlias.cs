// schema-version: 1.0
namespace Backend.Types.Datasource;

/** Type BadgeAlias. */
public class BadgeAlias
{
    public long Id { get; set; }
    public System.Guid ParentBadgeId { get; set; }
    public string ParentCode { get; set; } = "";
    public string Name { get; set; } = "";
}
