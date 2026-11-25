// schema-version: 1.0
namespace Backend.Types.Datasource;

/** Type Ward. */
public class Ward
{
    public long RegionId { get; set; }
    public long WardId { get; set; }
    public string Name { get; set; } = "";
}
