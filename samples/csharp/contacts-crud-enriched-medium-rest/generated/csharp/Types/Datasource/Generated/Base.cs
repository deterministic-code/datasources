// schema-version: 1.0
namespace Backend.Types.Datasource;

/** Type Base. */
public class Base
{
    public long Id { get; set; }
    public string Uuid { get; set; } = Guid.NewGuid().ToString();
    public System.DateTime Created { get; set; } = DateTime.UtcNow;
    public System.DateTime Updated { get; set; } = DateTime.UtcNow;
    public byte[] Version { get; set; } = new byte[] { 0 };
}
