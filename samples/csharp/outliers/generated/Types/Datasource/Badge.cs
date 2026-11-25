// schema-version: 1.0
namespace Backend.Types.Datasource;

/** Type Badge. */
public class Badge
{
    public System.Guid BadgeId { get; set; }
    public string Code { get; set; } = "";
    public string Name { get; set; } = "";
}
